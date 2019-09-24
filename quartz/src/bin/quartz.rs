/// Main executable

#[macro_use]
extern crate log;

use quartz::gui::run_gui;
use quartz::tsdb::TsDb;

use quartz::tsdb::server::run_server;

fn main() {
    simple_logger::init().unwrap();
    info!("BOOTING QUARTZ TOOL");

    // Create datastore:
    let db = TsDb::new().into_handle();

    // Start server
    let server = run_server(db.clone());
    run_gui(db);
    server.stop();
}
