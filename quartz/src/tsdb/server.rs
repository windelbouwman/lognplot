/// TCP based server for data
use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

use super::db::TsDbHandle;
use super::sample::Sample;

fn handle_client(counter: usize, mut stream: TcpStream, db: TsDbHandle) {
    info!("New client connected!");

    let trace_name = format!("Client{}", counter);
    db.lock().unwrap().new_trace(&trace_name);

    loop {
        let res = read_packet(&mut stream);

        match res {
            Ok(msg) => {
                let cursor = std::io::Cursor::new(msg);
                let pts: Vec<f64> = serde_cbor::from_reader(cursor).unwrap();
                println!("DAATAA: {:?}", pts.len());

                let samples: Vec<Sample> = pts.into_iter().map(|v| Sample::new(v)).collect();
                db.lock().unwrap().add_values(&trace_name, samples);

                // let f: f64 = 3.3;
            }
            Err(err) => {
                error!("Error in read: {:?}", err);
                break;
            }
        }
    }
}

/// Parse a little endian uint32 as lenght, and then use this length for a packet.
fn read_packet(stream: &mut dyn Read) -> std::io::Result<Vec<u8>> {
    let mut buf1: [u8; 4] = [0; 4];
    stream.read_exact(&mut buf1)?;
    let mut cursor = std::io::Cursor::new(buf1);
    // let b = bytes::
    use bytes::Buf;
    let length = cursor.get_uint_le(4) as usize;
    let mut buf2 = vec![0u8; length];
    stream.read_exact(&mut buf2)?;
    Ok(buf2)
}

pub fn run_server(db: TsDbHandle) -> io::Result<()> {
    let port = 12345;
    info!(
        "Starting up server at port {} with db {}!",
        port,
        db.lock().unwrap()
    );

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    let mut clients = vec![];
    // accept connections and process them serially
    for (counter, stream) in listener.incoming().enumerate() {
        let stream = stream?;
        let client_db_handle = db.clone();
        let t = thread::spawn(move || {
            handle_client(counter, stream, client_db_handle);
        });
        clients.push(t);
    }

    Ok(())
}
