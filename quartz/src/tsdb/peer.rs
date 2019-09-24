//! Handle a single peer via tcp socket.

use futures::sync::oneshot;
use tokio::codec::{Framed, LengthDelimitedCodec};
use tokio::net::TcpStream;
use tokio::prelude::*;

use super::db::TsDbHandle;
use super::sample::Sample;

/// A handle to a peer connection
pub struct PeerHandle {
    kill_switch: oneshot::Sender<()>,
}

impl PeerHandle {
    pub fn stop(self) {
        self.kill_switch.send(()).unwrap();
    }
}

/// Handle a single client
pub fn process_client(counter: usize, socket: TcpStream, db: TsDbHandle) -> PeerHandle {
    info!("Got incoming socket! {:?}", socket);

    let trace_name = format!("Client{}", counter);
    db.lock().unwrap().new_trace(&trace_name);

    let (framed_sink, framed_stream) = Framed::new(socket, LengthDelimitedCodec::new()).split();

    let client_task = framed_stream
        .for_each(move |packet| {
            // debug!("Got: {:?}", &packet);

            // try to decode cbor package:
            let pts: Vec<f64> = serde_cbor::from_slice(&packet).unwrap();
            println!("DAATAA: {:?}", pts.len());

            // Append the samples to the database:
            let samples: Vec<Sample> = pts.into_iter().map(|v| Sample::new(v)).collect();
            db.lock().unwrap().add_values(&trace_name, samples);
            Ok(())
        })
        .map_err(|err| println!("Failed: {:?}", err));

    // Create a kill switch for this client connection:
    let (kill_switch, c) = futures::sync::oneshot::channel::<()>();
    let c = c.map_err(|_| ());

    // Kill contraption:
    let task = client_task.select(c).map(|_| ()).map_err(|_| ());

    // Spawn of a task here:
    tokio::spawn(task);

    PeerHandle { kill_switch }
}
