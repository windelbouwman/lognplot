//! Notification support for changes in the database.
//!
//! This module is used by the GUI to respond to changes
//! which happen in the database.

use futures::channel::mpsc;
use std::collections::HashSet;

#[derive(Debug)]
pub struct ChangeSubscriber {
    channel: mpsc::Sender<DataChangeEvent>,
    event: DataChangeEvent,
}

impl ChangeSubscriber {
    pub fn new(channel: mpsc::Sender<DataChangeEvent>) -> Self {
        ChangeSubscriber {
            channel,
            event: DataChangeEvent::new(),
        }
    }

    /// Notification of a newly added signal
    pub fn notify_signal_added(&mut self, name: &str) {
        self.event.add_new_signal(name);
        self.emit_event();
    }

    /// Notification of a change on a data signal
    pub fn notify_signal_changed(&mut self, name: &str) {
        self.event.add_changed_signal(name);
        self.emit_event();
    }

    pub fn poll_events(&mut self) {
        if !self.event.is_empty() {
            self.emit_event();
        }
    }

    fn emit_event(&mut self) {
        if self.event.is_empty() {
            return;
        }

        // TODO: if ready?
        let ready = true;
        if ready {
            match self.channel.try_send(self.event.clone()) {
                Ok(()) => {
                    self.event = DataChangeEvent::new();
                }
                Err(err) => {
                    if err.is_full() {
                        // No worries, we still have the event :).
                    }
                }
            }
            // println!("Send res: {:?}", res);
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataChangeEvent {
    pub new_signals: HashSet<String>,
    pub changed_signals: HashSet<String>,
}

impl DataChangeEvent {
    fn new() -> Self {
        DataChangeEvent {
            new_signals: HashSet::new(),
            changed_signals: HashSet::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.new_signals.is_empty() && self.changed_signals.is_empty()
    }

    fn add_new_signal(&mut self, name: &str) {
        self.new_signals.insert(name.to_owned());
    }

    fn add_changed_signal(&mut self, name: &str) {
        self.changed_signals.insert(name.to_owned());
    }
}
