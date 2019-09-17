/// Main executable

#[macro_use]
extern crate log;

use quartz::gui::run_gui;

use quartz::tsdb::server::run_server;

fn main() {
    simple_logger::init().unwrap();
    info!("BOOTING QUARTZ TOOL");

    // Create datastore?

    // Start server

    run_server().unwrap();
    run_gui();
}
