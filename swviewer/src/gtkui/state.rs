//! UI state
//!

use crate::serial_wire_viewer::{data_thread, UiThreadCommand};
use crate::trace_var::TraceVar;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct UiState {
    variables: Mutex<HashMap<String, TraceVar>>,

    cmd_tx: Mutex<Option<mpsc::Sender<UiThreadCommand>>>,
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
            cmd_tx: Mutex::new(None),
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

    pub fn trace_var(&self, name: &str) {
        info!("Starting to trace {}", name);

        let var = self.variables.lock().unwrap().get(name).map(|v| v.clone());

        let cmd = UiThreadCommand::ConfigChannel { channel: 1, var };
        self.send_cmd(cmd);
    }

    fn send_cmd(&self, cmd: UiThreadCommand) {
        let mut cmd_tx = self.cmd_tx.lock().unwrap();
        if let Some(tx) = &*cmd_tx {
            if let Err(err) = tx.send(cmd) {
                error!("Error sending command: {}", err);
                *cmd_tx = None;
            }
        } else {
            error!("No connection active, not sending command");
        }
    }

    pub fn connect(&self, core_freq: u32) {
        info!("Connect to chip with core clock @ {} Hz!", core_freq);
        self.start(core_freq);
    }

    pub fn disconnect(&self) {
        info!("Disconnecting chip!");
    }

    fn start(&self, core_freq: u32) {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let join_handle = std::thread::spawn(move || {
            let lognplot_uri = "[::1]:12345";
            if let Err(err) = data_thread(lognplot_uri, core_freq, cmd_rx) {
                error!("Error: {:?}", err);
            }
            info!("Data thread ended.");
        });

        self.cmd_tx.lock().unwrap().replace(cmd_tx);
    }

    fn stop(&self) {
        self.send_cmd(UiThreadCommand::Stop);

        // join_handle.join();
    }
}
