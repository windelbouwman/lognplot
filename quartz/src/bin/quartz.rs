/// Main executable

#[macro_use]
extern crate log;

use quartz::gui::run_gui;
use quartz::tsdb::TsDb;
use std::thread;

use quartz::tsdb::server::run_server;

fn main() {
    simple_logger::init().unwrap();
    info!("BOOTING QUARTZ TOOL");

    // Create datastore?
    let db = TsDb::new().into_handle();

    // Start server

    let mut threads = vec![];
    let start_server = false;
    if start_server {
        let server_db_handle = db.clone();
        let t = thread::spawn(move || {
            run_server(server_db_handle).unwrap();
        });
        threads.push(t);
    }

    run_gui(db);

    for t in threads {
        t.join().unwrap();
    }
}
