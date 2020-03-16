// IDEA: route log messages to event queue

use super::ui::UiInput;
use std::sync::mpsc;
use std::sync::Mutex;

pub struct UiLogger {
    ui_queue: Mutex<mpsc::Sender<UiInput>>,
}

impl UiLogger {
    pub fn new(tx: mpsc::Sender<UiInput>) -> Self {
        UiLogger {
            ui_queue: Mutex::new(tx),
        }
    }
}

impl log::Log for UiLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if let Err(_err) = self.ui_queue.lock().unwrap().send(UiInput::Log(format!(
            "{} - {}",
            record.level(),
            record.args()
        ))) {
            // Error during logging..
            // What to do? Log a message?
            // TODO: figure something legit?
        }
    }

    fn flush(&self) {}
}
