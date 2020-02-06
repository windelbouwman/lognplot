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

    pub fn notify(&mut self, name: &str) {
        self.event.add_name(name);

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
    pub names: HashSet<String>,
}

impl DataChangeEvent {
    fn new() -> Self {
        DataChangeEvent {
            names: HashSet::new(),
        }
    }

    fn add_name(&mut self, name: &str) {
        self.names.insert(name.to_owned());
    }
}
