use gtk::prelude::*;
use std::collections::HashMap;

// TODO
// use crate::error_dialog::show_error;
use crate::state::GuiStateHandle;
use lognplot::tsdb::{DataChangeEvent, TsDbHandle};

pub struct SignalBrowser {
    model: gtk::TreeStore,
    db: TsDbHandle,

    // Mapping from signal name to row:
    model_map: HashMap<String, i32>,
}

impl SignalBrowser {
    /// Process a database data change event:
    async fn handle_event(&mut self, event: &DataChangeEvent) {
        if event.delete_all {
            self.delete_all();
        }
        self.add_new_signals(event.new_signals.iter()).await;
        self.update_signals(event.changed_signals.iter()).await;
    }

    /// Create new signals
    async fn add_new_signals<'a, I>(&mut self, new_signals: I)
    where
        I: Iterator<Item = &'a String>,
    {
        let mut updates = 0;
        for signal_name in new_signals {
            let iter = self.model.append(None);
            let row = self.model_map.len() as i32;
            self.model_map.insert(signal_name.clone(), row);
            self.model
                .set(&iter, &[0, 1, 2], &[&signal_name, &"-", &"-"]);

            updates += 1;
            if updates > 50 {
                // Pfew, take a brake to allow GUI to be responsive.
                debug!("Taking a break adding new signals in signal panel");
                updates = 0;
                glib::timeout_future_with_priority(glib::Priority::default(), 100).await;
            }
        }
    }

    /// Update existing signals in the model
    async fn update_signals<'a, I>(&self, changed_signals: I)
    where
        I: Iterator<Item = &'a String>,
    {
        let mut updates = 0;
        for signal_name in changed_signals {
            if let Some(summary) = self.db.quick_summary(&signal_name) {
                let row = self.model_map[signal_name];
                let path: gtk::TreePath = gtk::TreePath::new_from_indicesv(&[row]);
                if let Some(iter2) = self.model.get_iter(&path) {
                    self.model
                        .set_value(&iter2, 1, &summary.count.to_string().to_value());
                    self.model
                        .set_value(&iter2, 2, &summary.last_value().to_value());
                }
                updates += 1;
                if updates > 50 {
                    // Pfew, take a brake to allow GUI to be responsive.
                    debug!("Taking a break updating signal changes in panel");
                    updates = 0;
                    glib::timeout_future_with_priority(glib::Priority::default(), 100).await;
                }
            }
        }

        debug!("Updates: {}", updates);
    }

    /// Delete all signals from the model
    fn delete_all(&mut self) {
        self.model.clear();
        self.model_map.clear();
    }
}

/// Prepare a widget with a list of available signals.
pub fn setup_signal_repository(builder: &gtk::Builder, app_state: GuiStateHandle) {
    let model = gtk::TreeStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    setup_columns(builder);
    setup_filter_model(builder, &model);
    let tree_view: gtk::TreeView = builder.get_object("signal_tree_view").unwrap();
    setup_drag_drop(&tree_view);
    setup_dropping(&tree_view, app_state.clone());
    setup_activate(&tree_view, app_state.clone());
    setup_key_press_handler(&tree_view, app_state.clone());

    let db = { app_state.borrow().db.clone() };

    let signal_browser = SignalBrowser {
        model,
        db,
        model_map: HashMap::new(),
    };

    setup_notify_change(signal_browser);
}

fn setup_notify_change(mut signal_pane: SignalBrowser) {
    let mut receiver = signal_pane.db.new_notify_queue();

    // Insert async future function into the event loop:
    let main_context = glib::MainContext::default();
    main_context.spawn_local(async move {
        use futures::StreamExt;
        while let Some(event) = receiver.next().await {
            // println!("Event: {:?}", event);
            signal_pane.handle_event(&event).await;
            debug!("Done with updates!");

            // Delay to emulate rate limiting of events.
            glib::timeout_future_with_priority(glib::Priority::default(), 200).await;

            // Re-query database for some extra samples:
            signal_pane.db.poll_events();
        }
    });
}

fn setup_columns(builder: &gtk::Builder) {
    let name_column: gtk::TreeViewColumn = builder.get_object("column_name").unwrap();
    let size_column: gtk::TreeViewColumn = builder.get_object("column_size").unwrap();
    let last_value_column: gtk::TreeViewColumn = builder.get_object("column_last_value").unwrap();

    let cell = gtk::CellRendererText::new();
    name_column.pack_start(&cell, true);
    name_column.add_attribute(&cell, "text", 0);

    let cell = gtk::CellRendererText::new();
    size_column.pack_start(&cell, true);
    size_column.add_attribute(&cell, "text", 1);

    let cell = gtk::CellRendererText::new();
    last_value_column.pack_start(&cell, true);
    last_value_column.add_attribute(&cell, "text", 2);
}

fn setup_filter_model(builder: &gtk::Builder, model: &gtk::TreeStore) {
    let tree_view: gtk::TreeView = builder.get_object("signal_tree_view").unwrap();
    let filter_edit: gtk::SearchEntry = builder.get_object("signal_search_entry").unwrap();

    // Filter model:
    // Sort model:
    let sort_model = gtk::TreeModelSort::new(model);
    sort_model.set_sort_column_id(gtk::SortColumn::Index(0), gtk::SortType::Ascending);
    let filter_model = gtk::TreeModelFilter::new(&sort_model, None);

    filter_model.set_visible_func(clone!(@strong filter_edit => move |m, i| {
        let txt = filter_edit.get_text().unwrap().to_string();
        signal_filter_func(m, i, txt)
    }));

    tree_view.set_model(Some(&filter_model));

    filter_edit.connect_search_changed(move |_e| {
        filter_model.refilter();
    });
}

fn signal_filter_func(model: &gtk::TreeModel, iter: &gtk::TreeIter, filter_txt: String) -> bool {
    let optional_name = model.get_value(&iter, 0).get::<String>().unwrap();
    if let Some(name) = optional_name {
        filter_txt.is_empty() || name.contains(&filter_txt)
    } else {
        true
    }
}

/// Connect drag signal.
fn setup_drag_drop(tree_view: &gtk::TreeView) {
    let selection = tree_view.get_selection();
    selection.set_mode(gtk::SelectionMode::Multiple);

    let targets = vec![gtk::TargetEntry::new(
        super::mime_types::SIGNAL_NAMES_MIME_TYPE,
        gtk::TargetFlags::empty(),
        0,
    )];
    tree_view.drag_source_set(
        gdk::ModifierType::BUTTON1_MASK,
        &targets,
        gdk::DragAction::COPY,
    );
    tree_view.connect_drag_data_get(|w, _, data, info, _| {
        let selected_names = get_selected_signal_names(w);
        let mime_payload: String = serde_json::to_string(&selected_names).unwrap();
        let r = data.set_text(&mime_payload);
        if !r {
            error!("Drag data get transfer failed");
        }
        debug!("GET DATA {} {}", info, r);
    });
}

/// Enable files to be dropped on the widget:
fn setup_dropping(tree_view: &gtk::TreeView, app_state: GuiStateHandle) {
    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        gtk::TargetFlags::empty(),
        0,
    )];
    tree_view.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);

    tree_view.connect_drag_data_received(move |_w, _dc, _x, _y, data, _info, _time| {
        _w.stop_signal_emission("drag_data_received");
        let uris: Vec<String> = data.get_uris().iter().map(|u| u.to_string()).collect();
        info!("DROP {:?}", uris);
        for uri in uris {
            if let Err(err) = handle_drop_uri(uri, &app_state) {
                error!("Loading failed: {}", err);
            // TODO: show dialog box:
            // let toplevel = w.get_toplevel();
            // show_error(top_level, &err);
            } else {
                info!("Loaded!");
            }
        }
    });
}

fn handle_drop_uri(uri: String, app_state: &GuiStateHandle) -> Result<(), String> {
    info!("Loading uri {}", uri);
    let u = url::Url::parse(&uri).map_err(|e| e.to_string())?;

    if u.scheme() == "file" {
        let filepath = u
            .to_file_path()
            .map_err(|_| format!("Invalid file path url: {}", uri))?;
        info!("Loading file: {:?}", filepath);
        app_state.borrow().load(&filepath)
    } else {
        Err(format!("Wrong scheme for uri: {}", u.scheme()))
    }
}

fn get_selected_signal_names(w: &gtk::TreeView) -> Vec<String> {
    let selector = w.get_selection();
    let (selected_rows, tree_model) = selector.get_selected_rows();
    let mut selected_names: Vec<String> = vec![];
    for selected_row in selected_rows {
        if let Some(tree_iter) = tree_model.get_iter(&selected_row) {
            let value = get_signal_name(&tree_model, &tree_iter);
            selected_names.push(value);
        }
    }
    selected_names
}

fn setup_activate(tree_view: &gtk::TreeView, app_state: GuiStateHandle) {
    tree_view.connect_row_activated(move |tv, path, _| {
        let model = tv.get_model().unwrap();
        let iter = model.get_iter(path).unwrap();
        let value = get_signal_name(&model, &iter);

        debug!("Signal activated: {}, adding to chart.", value);
        // Add activated signal to plot:
        app_state.borrow().add_curve(&value, None);
    });
}

fn setup_key_press_handler(tree_view: &gtk::TreeView, app_state: GuiStateHandle) {
    tree_view.connect_key_press_event(move |tv, key| {
        let selected_signals = get_selected_signal_names(&tv);
        let chart_target = match key.get_keyval() {
            gdk::enums::key::_1 => Some(1),
            gdk::enums::key::_2 => Some(2),
            gdk::enums::key::_3 => Some(3),
            gdk::enums::key::_4 => Some(4),
            gdk::enums::key::_5 => Some(5),
            gdk::enums::key::_6 => Some(6),
            gdk::enums::key::_7 => Some(7),
            gdk::enums::key::_8 => Some(8),
            gdk::enums::key::_9 => Some(9),
            gdk::enums::key::A => Some(10),
            gdk::enums::key::B => Some(11),
            gdk::enums::key::C => Some(12),
            gdk::enums::key::D => Some(13),
            gdk::enums::key::E => Some(14),
            gdk::enums::key::F => Some(15),
            _ => None,
        };
        if chart_target.is_some() {
            for signal_name in selected_signals {
                debug!(
                    "Signal activated: {}, adding to chart {}.",
                    signal_name,
                    chart_target.expect("some value")
                );
                app_state.borrow().add_curve(&signal_name, chart_target);
            }
        }
        Inhibit(false)
    });
}

/// Given a model and an iterator get the signal name.
fn get_signal_name(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String {
    model.get_value(iter, 0).get::<String>().unwrap().unwrap()
}
