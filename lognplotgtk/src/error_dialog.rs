use gtk::prelude::*;

/// Show an error dialog.
pub fn show_error(top_level: &gtk::Window, message: &str) {
    error!("{}", message);
    let error_dialog = gtk::MessageDialog::new(
        Some(top_level),
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Error,
        gtk::ButtonsType::Ok,
        &message,
    );
    error_dialog.run();
}
