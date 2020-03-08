#[macro_use]
extern crate log;

#[macro_use]
extern crate glib;

mod chart_widget;
mod error_dialog;
mod io;
mod mainwindow;
mod meta_metrics;
mod mime_types;
mod resources;
mod session;
mod signal_repository;
mod state;

use lognplot::net::run_server;
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
        .arg(
            clap::Arg::with_name("port")
                .short("p")
                .help("Port to listen on")
                .default_value("12345"),
        )
        .get_matches();

    let verbosity = matches.occurrences_of("v");

    let log_level = match verbosity {
        0 => log::Level::Info,
        1 => log::Level::Debug,
        2 | _ => log::Level::Trace,
    };

    use std::str::FromStr;
    let port = u16::from_str(
        matches
            .value_of("port")
            .expect("port value must be present"),
    )
    .unwrap_or(12345);

    simple_logger::init_with_level(log_level).unwrap();

    info!("Starting lognplot GUI tool");

    let db = TsDb::default();
    let db_handle = db.into_handle();

    let stop_token = run_server(db_handle.clone(), port);
    mainwindow::open_gui(db_handle);
    stop_token.stop();
}
