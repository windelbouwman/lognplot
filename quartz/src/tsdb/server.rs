/// TCP based server for data
use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    info!("New client connected!");

    let mut buf: [u8; 4] = [0; 4];

    loop {
        let res = stream.read_exact(&mut buf);

        match res {
            Ok(data) => {
                // println!("DAATAA: {:?}", buf);
                let f: f64 = 3.3;
            }
            Err(err) => {
                error!("Error in read: {:?}", err);
                break;
            }
        }
    }
}

// fn read_packet(&mut r: Read) -> std::io::Result<Vec<u8>, std::io::Error> {
//     Ok(vec![])
// }

pub fn run_server() -> io::Result<()> {
    let port = 12345;
    info!("Starting up server at port {}!", port);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    let mut clients = vec![];
    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream?;
        let t = thread::spawn(move || {
            handle_client(stream);
        });
        clients.push(t);
    }

    Ok(())
}
