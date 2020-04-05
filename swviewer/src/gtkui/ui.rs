//! GUI using gtk-rs

// use gio::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;

use super::chip_panel::setup_chip_config_panel;
use super::state::{UiState, UiStateHandle};
use super::variable_panel::setup_elf_loading;

pub fn run_gtk_gui() {
    let application = Application::new(
        Some("com.github.windelbouwman.swviewer"),
        gio::ApplicationFlags::NON_UNIQUE,
    )
    .expect("failed to initialize GTK application");

    let view_model = UiState::new();

    application.connect_activate(move |app| build_ui(app, view_model.clone()));

    application.run(&[]);
}

fn build_ui(app: &gtk::Application, view_model: UiStateHandle) {
    // First we get the file content.
    let glade_src = include_str!("swviewer_gui.glade");

    // Then we call the Builder call.
    let builder = gtk::Builder::new_from_string(glade_src);

    // Connect the data set tree:
    setup_elf_loading(&builder, view_model.clone());
    setup_chip_config_panel(&builder, view_model);

    // Connect application to window:
    let window: gtk::Window = builder.get_object("top_unit").unwrap();

    window.set_application(Some(app));
    window.show_all();
}
