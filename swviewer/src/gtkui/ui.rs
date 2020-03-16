//! GUI using gtk-rs

// use gio::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;

use super::variable_panel::setup_elf_loading;
use crate::serial_wire_viewer::{data_thread, UiThreadCommand};
use std::sync::mpsc;

pub fn run_gtk_gui() {
    let application = Application::new(Some("com.github.windelbouwman.quartz"), Default::default())
        .expect("failed to initialize GTK application");

    let (cmd_tx, cmd_rx) = mpsc::channel();
    let join_handle = std::thread::spawn(move || {
        let lognplot_uri = "[::1]:12345";
        let core_freq = 16_000_000;
        if let Err(err) = data_thread(lognplot_uri, core_freq, cmd_rx) {
            error!("Error: {:?}", err);
        }
        info!("Data thread ended.");
    });

    let cmd_tx2 = cmd_tx.clone();
    application.connect_activate(move |app| build_ui(app, cmd_tx2.clone()));

    application.run(&[]);

    cmd_tx.send(UiThreadCommand::Stop);
    join_handle.join();
}

fn build_ui(app: &gtk::Application, cmd_tx: mpsc::Sender<UiThreadCommand>) {
    // First we get the file content.
    let glade_src = include_str!("gui.glade");

    // Then we call the Builder call.
    let builder = gtk::Builder::new_from_string(glade_src);

    // Connect the data set tree:
    setup_elf_loading(&builder);

    // Connect application to window:
    let window: gtk::Window = builder.get_object("top_unit").unwrap();

    window.set_application(Some(app));
    window.show_all();
}
