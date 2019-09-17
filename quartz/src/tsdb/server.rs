/// TCP based server for data
use std::io;
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    // ...
}

pub fn run_server() -> io::Result<()> {
    info!("Starting up server!");

    let listener = TcpListener::bind("127.0.0.1:12345")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
