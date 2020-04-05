//! UI state
//!

use crate::serial_wire_viewer::{data_thread, UiThreadCommand};
use crate::trace_var::TraceVar;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct UiState {
    variables: Mutex<HashMap<String, TraceVar>>,

    serial_wire_viewer: Mutex<SerialViewerInnerState>,
}

pub type UiStateHandle = Arc<UiState>;

/// Implement all logic operations for the GUI.
///
/// This is a view model with logical actions such as:
/// - start tracing a variable
/// - connect to a board
impl UiState {
    pub fn new() -> UiStateHandle {
        Arc::new(UiState {
            variables: Mutex::new(Default::default()),
            serial_wire_viewer: Mutex::new(Default::default()),
        })
    }

    pub fn load_variables(&self, variables: Vec<TraceVar>) {
        info!("Loading {} variables", variables.len());
        let mut var_map = HashMap::new();
        for variable in variables {
            var_map.insert(variable.name.clone(), variable);
        }
        *self.variables.lock().unwrap() = var_map;
    }

    pub fn trace_var(&self, channel: usize, name: &str) {
        info!("Starting to trace {} on channel {}", name, channel);
        let var = self.variables.lock().unwrap().get(name).map(|v| v.clone());
        self.serial_wire_viewer
            .lock()
            .unwrap()
            .config_channel(channel, var);
    }

    pub fn connect(&self, core_freq: u32) {
        info!("Connect to chip with core clock @ {} Hz!", core_freq);
        self.serial_wire_viewer.lock().unwrap().start(core_freq);
    }

    pub fn disconnect(&self) {
        info!("Disconnecting chip!");
        self.serial_wire_viewer.lock().unwrap().stop();
    }
}

#[derive(Default)]
struct SerialViewerInnerState {
    cmd_tx: Option<mpsc::Sender<UiThreadCommand>>,

    join_handle: Option<std::thread::JoinHandle<()>>,
}

impl SerialViewerInnerState {
    fn start(&mut self, core_freq: u32) {
        if !self.is_running() {
            let (cmd_tx, cmd_rx) = mpsc::channel();
            let join_handle = std::thread::spawn(move || {
                let lognplot_uri = "[::1]:12345";
                if let Err(err) = data_thread(lognplot_uri, core_freq, cmd_rx) {
                    error!("Error: {:?}", err);
                }
                info!("Data thread ended.");
            });

            self.cmd_tx.replace(cmd_tx);
            self.join_handle.replace(join_handle);
        }
    }

    fn config_channel(&mut self, channel: usize, var: Option<TraceVar>) {
        let cmd = UiThreadCommand::ConfigChannel { channel, var };
        self.send_cmd(cmd);
    }

    fn stop(&mut self) {
        if self.is_running() {
            self.send_cmd(UiThreadCommand::Stop);
            self.join_thread();
            self.cmd_tx = None;
        }
    }

    fn join_thread(&mut self) {
        if self.join_handle.is_some() {
            if let Err(err) = self.join_handle.take().unwrap().join() {
                error!("Thread stopped with error: {:?}", err);
            }
        }
    }

    fn is_running(&self) -> bool {
        self.join_handle.is_some()
    }

    fn send_cmd(&mut self, cmd: UiThreadCommand) {
        if let Some(tx) = &self.cmd_tx {
            if let Err(err) = tx.send(cmd) {
                error!("Error sending command: {}", err);
                self.join_thread();
                self.cmd_tx = None;
            }
        } else {
            error!("No connection active, not sending command");
        }
    }
}
