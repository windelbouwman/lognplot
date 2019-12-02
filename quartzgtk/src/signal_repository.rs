use gtk::prelude::*;

use super::GuiStateHandle;

/// Prepare a widget with a list of available signals.
pub fn setup_signal_repository(
    tree_view: gtk::TreeView,
    name_column: gtk::TreeViewColumn,
    app_state: GuiStateHandle,
) {
    let model = gtk::TreeStore::new(&[
        String::static_type(),
        // String::static_type(),
    ]);

    let nodex = model.insert_with_values(None, None, &[0], &[&"Trace0".to_string()]);
    model.insert_with_values(Some(&nodex), None, &[0], &[&"WOOT3".to_string()]);

    // FIXME: this is a fixed list of signals. Grab from tsdb dynamically.
    for i in 1..10 {
        model.insert_with_values(None, None, &[0], &[&format!("Trace{}", i)]);
    }

    let cell = gtk::CellRendererText::new();
    // name_column.set_renderer(&renderer);
    name_column.pack_start(&cell, true);
    name_column.add_attribute(&cell, "text", 0);

    tree_view.set_model(Some(&model));

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
        let value = tree_model.get_value(&tree_iter, 0).get::<String>().unwrap();
        // let txt: String = value.downcast().expect("Must work").get_some();
        let signal_name = &value;
        let r = data.set_text(signal_name);
        println!("GET DATA {} {}", info, r);
    });

    // Refresh model once in a while

    // Refreshing timer!
    let tick = move || {
        // println!("TICK!!!");
        // println!("App state: {}", app_state.borrow());
        // TODO: load traces from database.
        // app_state.borrow()
        gtk::Continue(true)
    };
    gtk::timeout_add(1000, tick);
}
