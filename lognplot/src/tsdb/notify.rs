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
    ready: bool,
}

impl ChangeSubscriber {
    pub fn new(channel: mpsc::Sender<DataChangeEvent>) -> Self {
        ChangeSubscriber {
            channel,
            event: DataChangeEvent::new(),
            ready: false,
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

    /// Notification that all data was deleted.
    pub fn notify_delete_all(&mut self) {
        self.event.add_delete_all();
        self.emit_event();
    }

    pub fn poll_events(&mut self) {
        self.ready = true;
        if !self.event.is_empty() {
            self.emit_event();
        }
    }

    fn emit_event(&mut self) {
        if self.ready {
            // Try a single time to emit the event.
            // If this fails, do not try again, since this would invoke
            // the expensive clone on the event, which potentially
            // would have to clone a whole slew of signals in a hashmap.
            // This takes a serious effort.
            // This is safe, since we will call poll again when the queue
            // was processed on the other end.

            self.ready = false;
            if self.event.is_empty() {
                return;
            }

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
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataChangeEvent {
    pub new_signals: HashSet<String>,
    pub changed_signals: HashSet<String>,
    pub delete_all: bool,
}

impl DataChangeEvent {
    fn new() -> Self {
        DataChangeEvent {
            new_signals: HashSet::new(),
            changed_signals: HashSet::new(),
            delete_all: false,
        }
    }

    fn is_empty(&self) -> bool {
        self.new_signals.is_empty() && self.changed_signals.is_empty() && !self.delete_all
    }

    fn add_new_signal(&mut self, name: &str) {
        self.new_signals.insert(name.to_owned());
    }

    fn add_changed_signal(&mut self, name: &str) {
        self.changed_signals.insert(name.to_owned());
    }

    /// Add a delete all signals event
    fn add_delete_all(&mut self) {
        // Drop all signals added so far:
        self.new_signals.clear();
        self.changed_signals.clear();

        self.delete_all = true;
    }
}
