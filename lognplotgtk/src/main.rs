#[macro_use]
extern crate log;

#[macro_use]
extern crate glib;

mod chart_widget;
mod io;
mod mainwindow;
mod mime_types;
mod signal_repository;
mod state;

use lognplot::server::run_server;
use lognplot::tsdb::TsDb;

pub use state::{GuiState, GuiStateHandle};

/// Create database, start server, and open a GUI.
fn main() {
    let matches = clap::App::new("lognplot GTK gui")
        .arg(
            clap::Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity."),
        )
        .get_matches();

    let verbosity = matches.occurrences_of("v");

    let log_level = match verbosity {
        0 => log::Level::Info,
        1 => log::Level::Debug,
        2 | _ => log::Level::Trace,
    };

    simple_logger::init_with_level(log_level).unwrap();

    info!("BOOTING QUARTZ TOOL");

    let db = TsDb::default();
    let db_handle = db.into_handle();

    let stop_token = run_server(db_handle.clone());
    mainwindow::open_gui(db_handle);
    stop_token.stop();
}
