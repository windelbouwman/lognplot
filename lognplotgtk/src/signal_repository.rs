use gtk::prelude::*;

use super::GuiStateHandle;

/// Prepare a widget with a list of available signals.
pub fn setup_signal_repository(builder: &gtk::Builder, app_state: GuiStateHandle) {
    let tree_view: gtk::TreeView = builder.get_object("signal_tree_view").unwrap();
    let filter_edit: gtk::SearchEntry = builder.get_object("signal_search_entry").unwrap();
    let name_column: gtk::TreeViewColumn = builder.get_object("column_name").unwrap();
    let size_column: gtk::TreeViewColumn = builder.get_object("column_size").unwrap();
    let last_value_column: gtk::TreeViewColumn = builder.get_object("column_last_value").unwrap();

    let model = gtk::TreeStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    let cell = gtk::CellRendererText::new();
    name_column.pack_start(&cell, true);
    name_column.add_attribute(&cell, "text", 0);

    let cell = gtk::CellRendererText::new();
    size_column.pack_start(&cell, true);
    size_column.add_attribute(&cell, "text", 1);

    let cell = gtk::CellRendererText::new();
    last_value_column.pack_start(&cell, true);
    last_value_column.add_attribute(&cell, "text", 2);

    // Filter model:
    // Sort model:
    let sort_model = gtk::TreeModelSort::new(&model);
    sort_model.set_sort_column_id(gtk::SortColumn::Index(0), gtk::SortType::Ascending);
    let filter_model = gtk::TreeModelFilter::new(&sort_model, None);

    filter_model.set_visible_func(clone!(@strong filter_edit => move |m, i| {
        let txt = filter_edit.get_text().unwrap().to_string();
        my_filter_func(m, i, txt)
    }));

    tree_view.set_model(Some(&filter_model));

    filter_edit.connect_search_changed(move |_e| {
        filter_model.refilter();
    });

    let selection = tree_view.get_selection();
    selection.set_mode(gtk::SelectionMode::Multiple);

    // Connect drag signal.
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

    // Refresh model once in a while

    // Refreshing timer!
    let tick = move || {
        let new_signal_names = app_state.borrow_mut().get_new_signal_names();

        for signal_name in new_signal_names {
            let iter = model.append(None);
            model.set(&iter, &[0, 1, 2], &[&signal_name, &"-", &"-"]);
        }

        // Uhm, this is a bit lame:
        let iter = model.get_iter_first();

        if let Some(iter2) = iter {
            for (signal_name, signal_size) in app_state.borrow().get_signal_sizes() {
                let model_name_value = model.get_value(&iter2, 0).get::<String>().unwrap().unwrap();
                if model_name_value == signal_name {
                    model.set_value(&iter2, 1, &signal_size.to_string().to_value());
                }

                if !model.iter_next(&iter2) {
                    break;
                }
            }
        }

        gtk::prelude::Continue(true)
    };
    gtk::timeout_add(1000, tick);
}

fn my_filter_func(model: &gtk::TreeModel, iter: &gtk::TreeIter, filter_txt: String) -> bool {
    let optional_name = model.get_value(&iter, 0).get::<String>().unwrap();
    // let filter_text = filter_edit;
    if let Some(name) = optional_name {
        // println!("FILTER {:?} with {}", name, filter_txt);
        if filter_txt.is_empty() || name.contains(&filter_txt) {
            true
        } else {
            false
        }
    } else {
        true
    }
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
