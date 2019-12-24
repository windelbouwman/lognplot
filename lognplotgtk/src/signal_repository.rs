use gtk::prelude::*;

use super::GuiStateHandle;

/// Prepare a widget with a list of available signals.
pub fn setup_signal_repository(
    tree_view: gtk::TreeView,
    filter_edit: gtk::SearchEntry,
    name_column: gtk::TreeViewColumn,
    app_state: GuiStateHandle,
) {
    let model = gtk::TreeStore::new(&[String::static_type()]);

    let cell = gtk::CellRendererText::new();
    // name_column.set_renderer(&renderer);
    name_column.pack_start(&cell, true);
    name_column.add_attribute(&cell, "text", 0);

    // Filter model:
    // Sort model:
    let sort_model = gtk::TreeModelSort::new(&model);
    sort_model.set_sort_column_id(gtk::SortColumn::Index(0), gtk::SortType::Ascending);
    let filter_model = gtk::TreeModelFilter::new(&sort_model, None);

    filter_model.set_visible_func(clone!(@strong filter_edit => move |m, i| {
        let txt = filter_edit.get_text().unwrap().to_string();
        // let txt = "FUU".to_string();
        my_filter_func(m, i, txt)
    }));

    tree_view.set_model(Some(&filter_model));

    filter_edit.connect_search_changed(move |_e| {
        filter_model.refilter();
    });

    // tree_view.set_enable_search(true);
    // tree_view.set_search_column(0);

    // Connect drag signal.
    let targets = vec![gtk::TargetEntry::new(
        "text/plain",
        gtk::TargetFlags::empty(),
        0,
    )];
    tree_view.drag_source_set(
        gdk::ModifierType::BUTTON1_MASK,
        &targets,
        gdk::DragAction::COPY,
    );
    tree_view.connect_drag_data_get(|w, _dc, data, info, _time| {
        let selector = w.get_selection();
        let (tree_model, tree_iter) = selector.get_selected().expect("At least some selection");
        let value = tree_model
            .get_value(&tree_iter, 0)
            .get::<String>()
            .unwrap()
            .unwrap();
        // let txt: String = value.downcast().expect("Must work").get_some();
        let signal_name = &value;
        let r = data.set_text(signal_name);
        debug!("GET DATA {} {}", info, r);
    });

    // Refresh model once in a while

    // Refreshing timer!
    let tick = move || {
        let new_signal_names = app_state.borrow_mut().get_new_signal_names();

        for signal_name in new_signal_names {
            let iter = model.append(None);
            model.set(&iter, &[0], &[&signal_name]);
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
