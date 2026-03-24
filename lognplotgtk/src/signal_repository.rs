use gdk::Key;
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
                .set(&iter, &[(0, signal_name), (1, &"-"), (2, &"-")]);

            updates += 1;
            if updates > 50 {
                // Pfew, take a brake to allow GUI to be responsive.
                debug!("Taking a break adding new signals in signal panel");
                updates = 0;
                glib::timeout_future_with_priority(
                    glib::Priority::default(),
                    std::time::Duration::from_millis(100),
                )
                .await;
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
                let path: gtk::TreePath = gtk::TreePath::from_indices(&[row]);
                if let Some(iter2) = self.model.iter(&path) {
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
                    glib::timeout_future_with_priority(
                        glib::Priority::default(),
                        std::time::Duration::from_millis(100),
                    )
                    .await;
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
pub fn setup_signal_repository(app_state: &GuiStateHandle) -> gtk::Box {
    let model = gtk::TreeStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    let search_entry = gtk::SearchEntry::new();
    let scrolled_window = gtk::ScrolledWindow::builder().vexpand(true).build();
    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let tree_view = gtk::TreeView::new();
    scrolled_window.set_child(Some(&tree_view));
    vbox.append(&search_entry);
    vbox.append(&scrolled_window);

    setup_columns(&tree_view);
    setup_filter_model(&tree_view, &search_entry, &model);
    setup_drag_drop(&tree_view);
    setup_dropping(&tree_view);
    setup_activate(&tree_view, app_state.clone());
    setup_key_press_handler(&tree_view, app_state.clone());

    let db = { app_state.borrow().db.clone() };

    let signal_browser = SignalBrowser {
        model,
        db,
        model_map: HashMap::new(),
    };

    setup_notify_change(signal_browser);

    vbox
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
            glib::timeout_future_with_priority(
                glib::Priority::default(),
                std::time::Duration::from_millis(200),
            )
            .await;

            // Re-query database for some extra samples:
            signal_pane.db.poll_events();
        }
    });
}

fn setup_columns(tree_view: &gtk::TreeView) {
    let name_column = gtk::TreeViewColumn::builder().title("Name").build();
    let size_column = gtk::TreeViewColumn::builder().title("Size").build();
    let last_value_column = gtk::TreeViewColumn::builder().title("Last value").build();
    tree_view.append_column(&name_column);
    tree_view.append_column(&size_column);
    tree_view.append_column(&last_value_column);

    let cell = gtk::CellRendererText::new();
    name_column.pack_start(&cell, true);
    name_column.add_attribute(&cell, "text", 0);
    name_column.set_resizable(true);

    let cell = gtk::CellRendererText::new();
    size_column.pack_start(&cell, true);
    size_column.add_attribute(&cell, "text", 1);
    size_column.set_resizable(true);

    let cell = gtk::CellRendererText::new();
    last_value_column.pack_start(&cell, true);
    last_value_column.add_attribute(&cell, "text", 2);
    last_value_column.set_resizable(true);
}

fn setup_filter_model(
    tree_view: &gtk::TreeView,
    filter_edit: &gtk::SearchEntry,
    model: &gtk::TreeStore,
) {
    // Filter model:
    // Sort model:
    let sort_model = gtk::TreeModelSort::with_model(model);
    sort_model.set_sort_column_id(gtk::SortColumn::Index(0), gtk::SortType::Ascending);
    let filter_model = gtk::TreeModelFilter::new(&sort_model, None);

    filter_model.set_visible_func(clone!(
        #[strong]
        filter_edit,
        move |m, i| {
            let txt = filter_edit.text().to_string();
            signal_filter_func(m, i, txt)
        }
    ));

    tree_view.set_model(Some(&filter_model));

    filter_edit.connect_search_changed(move |_e| {
        filter_model.refilter();
    });
}

fn signal_filter_func(model: &gtk::TreeModel, iter: &gtk::TreeIter, filter_txt: String) -> bool {
    if let Ok(name) = model.get_value(&iter, 0).get::<String>() {
        filter_txt.is_empty() || name.contains(&filter_txt)
    } else {
        true
    }
}

/// Connect drag signal.
fn setup_drag_drop(tree_view: &gtk::TreeView) {
    let selection = tree_view.selection();
    selection.set_mode(gtk::SelectionMode::Multiple);

    let drag_src = gtk::DragSource::builder().build();
    drag_src.connect_prepare(clone!(
        #[strong]
        tree_view,
        move |_drag_src, _x, _y| {
            let selected_names = get_selected_signal_names(&tree_view);
            let mime_payload: String = serde_json::to_string(&selected_names).unwrap();
            let byte_data = glib::Bytes::from(mime_payload.as_bytes());
            let content = gdk::ContentProvider::for_bytes(
                super::mime_types::SIGNAL_NAMES_MIME_TYPE,
                &byte_data,
            );
            info!("Drag signals");
            Some(content)
        }
    ));
    tree_view.add_controller(drag_src);
}

/// Enable files to be dropped on the widget:
fn setup_dropping(tree_view: &gtk::TreeView) {
    let formats = gdk::ContentFormats::builder()
        .add_type(glib::Type::BOXED)
        .add_mime_type("text/uri-list")
        .build();
    let drop_target = gtk::DropTarget::builder()
        .actions(gdk::DragAction::COPY)
        .formats(&formats)
        .build();

    drop_target.connect_drop(move |_target, value, _x, _y| {
        info!("Drop 1");
        let uris = value.get::<String>().expect("Works");
        info!("DROP {:?}", uris);
        /*
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
        */
        true
    });

    tree_view.add_controller(drop_target);
}

fn _handle_drop_uri(uri: String, app_state: &GuiStateHandle) -> Result<(), String> {
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

fn get_selected_signal_names(tree_view: &gtk::TreeView) -> Vec<String> {
    let selector = tree_view.selection();
    let (selected_rows, tree_model) = selector.selected_rows();
    let mut selected_names: Vec<String> = vec![];
    for selected_row in selected_rows {
        if let Some(tree_iter) = tree_model.iter(&selected_row) {
            let value = get_signal_name(&tree_model, &tree_iter);
            selected_names.push(value);
        }
    }
    selected_names
}

fn setup_activate(tree_view: &gtk::TreeView, app_state: GuiStateHandle) {
    tree_view.connect_row_activated(move |tv, path, _| {
        let model = tv.model().unwrap();
        let iter = model.iter(path).unwrap();
        let value = get_signal_name(&model, &iter);

        debug!("Signal activated: {}, adding to chart.", value);
        // Add activated signal to plot:
        app_state.borrow().add_curve(&value, None);
    });
}

fn setup_key_press_handler(tree_view: &gtk::TreeView, app_state: GuiStateHandle) {
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(clone!(
        #[strong]
        tree_view,
        move |_eck, key, _code, _state| {
            let selected_signals = get_selected_signal_names(&tree_view);
            let chart_target = match key {
                Key::_1 => Some(1),
                Key::_2 => Some(2),
                Key::_3 => Some(3),
                Key::_4 => Some(4),
                Key::_5 => Some(5),
                Key::_6 => Some(6),
                Key::_7 => Some(7),
                Key::_8 => Some(8),
                Key::_9 => Some(9),
                Key::A => Some(10),
                Key::B => Some(11),
                Key::C => Some(12),
                Key::D => Some(13),
                Key::E => Some(14),
                Key::F => Some(15),
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
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        }
    ));
    tree_view.add_controller(key_controller);
}

/// Given a model and an iterator get the signal name.
fn get_signal_name(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String {
    model.get_value(iter, 0).get::<String>().unwrap()
}
