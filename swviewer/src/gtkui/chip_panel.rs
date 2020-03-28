use super::dialogs::show_error;
use crate::symbolscanner::parse_elf_file;
use gio::prelude::*;
use gtk::prelude::*;

pub fn setup_chip_config_panel(builder: &gtk::Builder) {
    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();

    let connect_button: gtk::Button = builder.get_object("button_connect").unwrap();
    connect_button.connect_clicked(move |_| {
        info!("Connect to chip!");
    });
}
