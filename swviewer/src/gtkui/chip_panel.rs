use super::dialogs::show_error;
use super::UiStateHandle;
use gtk::prelude::*;
use std::str::FromStr;

pub fn setup_chip_config_panel(builder: &gtk::Builder, view_model: UiStateHandle) {
    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();
    let core_freq_entry: gtk::Entry = builder.get_object("entry_core_frequency").unwrap();
    let connect_button: gtk::Button = builder.get_object("button_connect").unwrap();
    let disconnect_button: gtk::Button = builder.get_object("button_disconnect").unwrap();

    disconnect_button.connect_clicked(clone!(@strong view_model => move |_| {
        view_model.disconnect();
    }));

    connect_button.connect_clicked(move |_| {
        let value_text: String = core_freq_entry.get_text().unwrap().to_string();
        match u32::from_str(&value_text) {
            Ok(core_freq) => {
                view_model.connect(core_freq);
            }
            Err(err) => {
                let msg = format!("Invalid core frequency: {}", err);
                show_error(&top_level, &msg);
            }
        };
    });
}
