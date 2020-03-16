use super::dialogs::show_error;
use crate::symbolscanner::parse_elf_file;
use gio::prelude::*;
use gtk::prelude::*;

pub fn setup_elf_loading(builder: &gtk::Builder) {
    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();

    let model = gtk::TreeStore::new(&[
        String::static_type(),
        String::static_type(),
        String::static_type(),
    ]);

    setup_search_filter(builder, &model);
    setup_columns(builder);
    let variables_tree_view: gtk::TreeView = builder.get_object("variables_tree_view").unwrap();
    setup_activated(&variables_tree_view);

    let load_button: gtk::Button = builder.get_object("button_load_elf_file").unwrap();
    load_button.connect_clicked(move |_| {
        load_elf_file(&top_level, &model);
        info!("Clicke!");
    });
}

fn setup_activated(variables_tree_view: &gtk::TreeView) {
    variables_tree_view.connect_row_activated(move |tv, path, _| {
        let model = tv.get_model().unwrap();
        let iter = model.get_iter(path).unwrap();
        let value = get_variable_name(&model, &iter);

        info!("Signal activated: {}, starting to trace.", value);
        // Add activated signal to plot:
        // app_state.borrow().add_curve(&value, None);
    });
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

fn load_elf_file(top_level: &gtk::Window, model: &gtk::TreeStore) {
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

                for variable in variables {
                    let iter = model.append(None);
                    model.set(
                        &iter,
                        &[0, 1, 2],
                        &[&variable.name, &format!("0x{:08X}", variable.address), &"-"],
                    );
                }
            }
        }
    }
}
