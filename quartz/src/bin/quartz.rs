/// Main executable

#[macro_use]
extern crate log;

fn main() {
    simple_logger::init().unwrap();

    info!("Starting gui!!");
}
