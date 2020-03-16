mod ui;
mod ui_logger;

use crate::serial_wire_viewer::{data_thread, SerialWireViewerResult, UiThreadCommand};
use crate::symbolscanner::parse_elf_file;
use std::path::PathBuf;
use std::sync::mpsc;
use ui::{run_tui, UiInput};
use ui_logger::UiLogger;

pub fn do_magic(
    elf_filename: &PathBuf,
    lognplot_uri: String,
    core_freq_hz: u32,
) -> SerialWireViewerResult<()> {
    let (cmd_tx, cmd_rx) = mpsc::channel::<UiThreadCommand>();
    let (event_tx, event_rx) = mpsc::channel::<UiInput>();

    let tui_logger = UiLogger::new(event_tx.clone());
    log::set_boxed_logger(Box::new(tui_logger)).unwrap();
    log::set_max_level(log::Level::Info.to_level_filter());

    // Parse elf file:
    let trace_vars = parse_elf_file(elf_filename)?;

    let t1 = std::thread::spawn(move || {
        if let Err(err) = data_thread(&lognplot_uri, core_freq_hz, cmd_rx) {
            error!("ERROR: {:?}", err);
        }

        info!("Data thread finished");
    });

    run_tui(trace_vars, cmd_tx, event_rx)?;

    t1.join().unwrap();

    Ok(())
}
