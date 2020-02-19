use gtk::prelude::*;
use std::collections::HashMap;

use super::GuiStateHandle;
use lognplot::tsdb::DataChangeEvent;

pub struct SignalBrowser {
    model: gtk::TreeStore,
    app_state: GuiStateHandle,

    // Mapping from signal name to row:
    model_map: HashMap<String, i32>,
}

impl SignalBrowser {
    /// Process a database data change event:
    pub fn handle_event(&mut self, event: &DataChangeEvent) {
        if event.drop_all {
            self.drop_all();
        }
        self.add_new_signals(event.new_signals.iter());
        self.update_signals(event.changed_signals.iter());
    }

    /// Create new signals
    fn add_new_signals<'a, I>(&mut self, new_signals: I)
    where
        I: Iterator<Item = &'a String>,
    {
        for signal_name in new_signals {
            let iter = self.model.append(None);
            let row = self.model_map.len() as i32;
            self.model_map.insert(signal_name.clone(), row);
            self.model
                .set(&iter, &[0, 1, 2], &[&signal_name, &"-", &"-"]);
        }
    }

    /// Update existing signals in the model
    fn update_signals<'a, I>(&self, changed_signals: I)
    where
        I: Iterator<Item = &'a String>,
    {
        for signal_name in changed_signals {
            if let Some(summary) = self.app_state.borrow().get_signal_summary(&signal_name) {
                let row = self.model_map[signal_name];
                let path: gtk::TreePath = gtk::TreePath::new_from_indicesv(&[row]);
                if let Some(iter2) = self.model.get_iter(&path) {
                    self.model
                        .set_value(&iter2, 1, &summary.count.to_string().to_value());
                    self.model
                        .set_value(&iter2, 2, &summary.metrics().last.to_string().to_value());
                }
            }
        }
    }

    /// Drop all signals from the model
    fn drop_all(&mut self) {
        self.model.clear();
        self.model_map.clear();
    }
}

/// Prepare a widget with a list of available signals.
pub fn setup_signal_repository<F: Fn(&str) + 'static>(
    builder: &gtk::Builder,
    app_state: GuiStateHandle,
    add_curve: F,
) -> SignalBrowser {
    let model = gtk::TreeStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    setup_columns(builder);
    setup_filter_model(builder, &model);
    setup_drag_drop(builder);
    setup_activate(builder, add_curve);

    SignalBrowser {
        model,
        app_state,
        model_map: HashMap::new(),
    }
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
fn setup_drag_drop(builder: &gtk::Builder) {
    let tree_view: gtk::TreeView = builder.get_object("signal_tree_view").unwrap();

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
    tree_view.connect_drag_data_get(|w, _dc, data, info, _time| {
        let selected_names = get_selected_signal_names(w);
        // Join all signal names by ':'
        let r = data.set_text(&selected_names.join(":"));
        if !r {
            error!("Drag data get transfer failed");
        }
        debug!("GET DATA {} {}", info, r);
    });
}

fn get_selected_signal_names(w: &gtk::TreeView) -> Vec<String> {
    let selector = w.get_selection();
    let (selected_rows, tree_model) = selector.get_selected_rows();
    let mut selected_names: Vec<String> = vec![];
    for selected_row in selected_rows {
        if let Some(tree_iter) = tree_model.get_iter(&selected_row) {
            let value = tree_model
                .get_value(&tree_iter, 0)
                .get::<String>()
                .unwrap()
                .unwrap();
            // let signal_name = &value;
            selected_names.push(value);
        }
    }
    selected_names
}

fn setup_activate<F: Fn(&str) + 'static>(builder: &gtk::Builder, add_curve: F) {
    let tree_view: gtk::TreeView = builder.get_object("signal_tree_view").unwrap();

    tree_view.connect_row_activated(move |tv, path, _column| {
        let model = tv.get_model().unwrap();
        let iter = model.get_iter(path).unwrap();
        let value = model.get_value(&iter, 0).get::<String>().unwrap().unwrap();

        debug!("Signal activated: {}, adding to chart.", value);
        // Add activated signal to plot:
        add_curve(&value);
    });
}
