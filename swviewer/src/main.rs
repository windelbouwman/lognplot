#[macro_use]
extern crate log;

mod coresight;
mod serial_wire_viewer;
mod stlink;
mod symbolscanner;
mod trace_var;
mod ui;
mod ui_logger;
mod usbutil;

use serial_wire_viewer::do_magic;

fn main() {
    let matches = clap::App::new("swviewer")
        .arg(
            clap::Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            clap::Arg::with_name("elf")
                .required(true)
                .help("ELF file with debug info where to find variables."),
        )
        .arg(
            clap::Arg::with_name("lognplot-server")
                .short("L")
                .long("lognplot-server")
                .takes_value(true)
                .help("The lognplot GUI address to connect to. Defaults to [::1]:12345"),
        )
        .arg(
            clap::Arg::with_name("cpu-freq")
                .short("F")
                .long("cpu-frequency")
                .takes_value(true)
                .help("The cpu frequency in Hz. Defaults to 16000000. This is important to be correct for the tracing configuration prescaler. Set this to your application configured CPU frequency."),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => log::Level::Info,
        1 => log::Level::Debug,
        2 | _ => log::Level::Trace,
    };

    // simple_logger::init_with_level(log_level).unwrap();

    let elf_filename: String = matches
        .value_of("elf")
        .expect("Mandatory argument")
        .to_string();
    let lognplot_uri: String = matches
        .value_of("lognplot-server")
        .unwrap_or("[::1]:12345")
        .to_owned();
    let cpu_freq: u32 =
        match u32::from_str_radix(matches.value_of("cpu-freq").unwrap_or("16000000"), 10) {
            Err(e) => {
                error!("Invalid cpu freq argument: {}", e);
                return;
            }
            Ok(v) => v,
        };

    info!("Log level: {}", log_level);
    info!("Lognplot server: {}", lognplot_uri);
    info!("CPU freq in Hz: {:?}", cpu_freq);
    info!("rusb version: {:?}", rusb::version());

    if let Err(e) = do_magic(&elf_filename, lognplot_uri, cpu_freq) {
        error!("An error occurred: {:?}", e);
    }
}
