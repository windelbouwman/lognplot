#[macro_use]
extern crate log;

#[macro_use]
extern crate glib;

mod chart_widget;
mod error_dialog;

#[cfg(feature = "hdf5")]
mod io;

mod mainwindow;
mod mime_types;
mod resources;
mod session;
mod signal_repository;
mod state;

use lognplot::net::run_server;
use lognplot::tracer::AnyTracer;
use lognplot::tsdb::TsDb;
use std::sync::Arc;

pub use state::{GuiState, GuiStateHandle};

#[cfg(not(features = "hdf5"))]
mod io {
    use super::GuiStateHandle;

    pub fn save_data_as_hdf5(top_level: &gtk::Window, app_state: &GuiStateHandle) {
        unimplemented!();
    }

    pub fn load_data_from_hdf5(top_level: &gtk::Window, app_state: &GuiStateHandle) {
        unimplemented!();
    }
}

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
                .long("port")
                .help("Port to listen on")
                .default_value("12345"),
        )
        .arg(
            clap::Arg::with_name("meta-trace")
                .long("meta-trace")
                .help("Trace internal performance metrics in the plot tool itself."),
        )
        .arg(
            clap::Arg::with_name("meta-trace-remote")
                .long("--meta-trace-remote")
                .takes_value(true)
                .help("Trace internal performance metrics to the given address (host:port)."),
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

    let perf_tracer = if matches.is_present("meta-trace-remote") {
        let addr = matches.value_of("meta-trace-remote").unwrap();
        info!("Setting up meta tracing to remote: {:?}", addr);
        // let address = std::net::SocketAddr::from_str(addr);
        match lognplot::net::TcpClient::new(addr) {
            Ok(client) => Arc::new(AnyTracer::new_tcp(client)),
            Err(err) => {
                error!("Error setting up remote tracing: {:?}", err);
                Arc::new(AnyTracer::new_void())
            }
        }
    } else if matches.is_present("meta-trace") {
        info!("Setting up meta tracing");
        Arc::new(AnyTracer::new_db(db_handle.clone()))
    } else {
        Arc::new(AnyTracer::new_void())
    };

    let stop_token = run_server(db_handle.clone(), port, perf_tracer.clone());
    mainwindow::open_gui(db_handle, perf_tracer);
    stop_token.stop();
}
