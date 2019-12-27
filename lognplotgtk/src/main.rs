#[macro_use]
extern crate log;

#[macro_use]
extern crate glib;

mod chart_widget;
mod mainwindow;
mod mime_types;
mod io;
mod signal_repository;
mod state;

use lognplot::server::run_server;
use lognplot::tsdb::TsDb;

pub use state::{GuiState, GuiStateHandle};

/// Create database, start server, and open a GUI.
fn main() {
    env_logger::init();
    info!("BOOTING QUARTZ TOOL");

    let db = TsDb::default();
    let db_handle = db.into_handle();

    let stop_token = run_server(db_handle.clone());
    mainwindow::open_gui(db_handle);
    stop_token.stop();
}
