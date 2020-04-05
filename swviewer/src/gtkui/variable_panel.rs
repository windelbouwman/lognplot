use super::dialogs::show_error;
use super::UiStateHandle;
use crate::symbolscanner::parse_elf_file;
use gio::prelude::*;
use gtk::prelude::*;

pub fn setup_elf_loading(builder: &gtk::Builder, view_model: UiStateHandle) {
    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();

    let model = gtk::TreeStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    setup_search_filter(builder, &model);
    setup_columns(builder);
    let variables_tree_view: gtk::TreeView = builder.get_object("variables_tree_view").unwrap();
    setup_activated(&variables_tree_view, view_model.clone());
    setup_key_press_event(&variables_tree_view, view_model.clone());

    let load_button: gtk::Button = builder.get_object("button_load_elf_file").unwrap();
    load_button.connect_clicked(move |_| {
        load_elf_file(&top_level, &model, &view_model);
        info!("Clicke!");
    });
}

fn setup_activated(variables_tree_view: &gtk::TreeView, view_model: UiStateHandle) {
    variables_tree_view.connect_row_activated(move |tv, path, _| {
        let model = tv.get_model().unwrap();
        let iter = model.get_iter(path).unwrap();
        let value = get_variable_name(&model, &iter);
        info!("Signal activated: {}, starting to trace.", value);
        view_model.trace_var(0, &value);
    });
}

fn setup_key_press_event(variables_tree_view: &gtk::TreeView, view_model: UiStateHandle) {
    variables_tree_view.connect_key_press_event(move |tv, key| {
        let selected_signals = get_selected_variable_names(&tv);
        let channel_target = match key.get_keyval() {
            gdk::enums::key::_1 => Some(0),
            gdk::enums::key::_2 => Some(1),
            gdk::enums::key::_3 => Some(2),
            gdk::enums::key::_4 => Some(3),
            gdk::enums::key::_5 => Some(4),
            gdk::enums::key::_6 => Some(5),
            gdk::enums::key::_7 => Some(6),
            gdk::enums::key::_8 => Some(7),
            _ => None,
        };

        if let (Some(channel), Some(name)) = (channel_target, selected_signals.first()) {
            view_model.trace_var(channel, &name);
        }

        Inhibit(false)
    });
}

fn get_selected_variable_names(w: &gtk::TreeView) -> Vec<String> {
    let selector = w.get_selection();
    let (selected_rows, tree_model) = selector.get_selected_rows();
    let mut selected_names: Vec<String> = vec![];
    for selected_row in selected_rows {
        if let Some(tree_iter) = tree_model.get_iter(&selected_row) {
            let value = get_variable_name(&tree_model, &tree_iter);
            selected_names.push(value);
        }
    }
    selected_names
}

/// Given a model and an iterator get the signal name.
fn get_variable_name(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String {
    model.get_value(iter, 0).get::<String>().unwrap().unwrap()
}

fn setup_columns(builder: &gtk::Builder) {
    let name_column: gtk::TreeViewColumn = builder.get_object("col_name").unwrap();
    let address_column: gtk::TreeViewColumn = builder.get_object("col_address").unwrap();
    let type_column: gtk::TreeViewColumn = builder.get_object("col_type").unwrap();

    // treeview columns
    let cell = gtk::CellRendererText::new();
    name_column.pack_start(&cell, true);
    name_column.add_attribute(&cell, "text", 0);

    let cell = gtk::CellRendererText::new();
    address_column.pack_start(&cell, true);
    address_column.add_attribute(&cell, "text", 1);

    let cell = gtk::CellRendererText::new();
    type_column.pack_start(&cell, true);
    type_column.add_attribute(&cell, "text", 2);
}

fn setup_search_filter(builder: &gtk::Builder, model: &gtk::TreeStore) {
    let filter_model = gtk::TreeModelFilter::new(model, None);

    let search_bar: gtk::SearchEntry = builder.get_object("search_bar").unwrap();
    let variables_tree_view: gtk::TreeView = builder.get_object("variables_tree_view").unwrap();
    filter_model.set_visible_func(clone!(@strong search_bar => move |m, i| {
        let txt = search_bar.get_text().unwrap().to_string().to_lowercase();
        signal_filter_func(m, i, txt)
    }));

    variables_tree_view.set_model(Some(&filter_model));

    search_bar.connect_search_changed(move |_e| {
        filter_model.refilter();
    });
}

fn signal_filter_func(model: &gtk::TreeModel, iter: &gtk::TreeIter, filter_txt: String) -> bool {
    let optional_name = model.get_value(&iter, 0).get::<String>().unwrap();
    if let Some(name) = optional_name {
        filter_txt.is_empty() || name.to_lowercase().contains(&filter_txt)
    } else {
        true
    }
}

fn load_elf_file(top_level: &gtk::Window, model: &gtk::TreeStore, view_model: &UiStateHandle) {
    let dialog = gtk::FileChooserDialog::with_buttons(
        Some("Load ELF file with symbols"),
        Some(top_level),
        gtk::FileChooserAction::Open,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Open", gtk::ResponseType::Accept),
        ],
    );

    let res = dialog.run();
    let filename = dialog.get_filename();
    dialog.destroy();
    if let (gtk::ResponseType::Accept, Some(filename)) = (res, filename) {
        info!("Loading data from filename: {:?}", filename);
        match parse_elf_file(&filename) {
            Err(err) => {
                let error_message = format!("Error loading data from {:?}: {}", filename, err);
                show_error(top_level, &error_message);
            }
            Ok(variables) => {
                info!("Data loaded!");
                model.clear();

                for variable in &variables {
                    let iter = model.append(None);
                    model.set(
                        &iter,
                        &[0, 1, 2],
                        &[
                            &variable.name,
                            &format!("0x{:08X}", variable.address),
                            &variable.typ.to_string(),
                        ],
                    );
                }

                view_model.load_variables(variables);
            }
        }
    }
}
