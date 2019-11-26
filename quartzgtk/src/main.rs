#[macro_use]
extern crate log;

mod mainwindow;
mod state;

use lognplot::server::run_server;
use lognplot::tsdb::TsDb;

pub use state::{GuiState, GuiStateHandle};

/// Create database, start server, and open a GUI.
fn main() {
    simple_logger::init().unwrap();
    info!("BOOTING QUARTZ TOOL");

    let db = TsDb::default();
    let db_handle = db.into_handle();

    let stop_token = run_server(db_handle.clone());
    mainwindow::open_gui(db_handle);
    stop_token.stop();
}
