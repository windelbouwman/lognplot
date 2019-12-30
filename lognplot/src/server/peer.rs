//! Handle a single peer via tcp socket.

use futures::channel::oneshot;
use futures::{FutureExt, StreamExt};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::payload::SampleBatch;
use crate::tsdb::TsDbHandle;

/// A handle to a peer connection
pub struct PeerHandle {
    kill_switch: oneshot::Sender<()>,
    join_handle: JoinHandle<()>,
}

impl PeerHandle {
    pub async fn stop(self) -> std::io::Result<()> {
        info!("Stopping peer");
        match self.kill_switch.send(()) {
            Err(_) => {
                info!("Peer already disconnected");
            }
            Ok(_) => {
                info!("Peer stopped");
            }
        }
        self.join_handle.await?;
        Ok(())
    }
}

/// Handle a single client
pub fn process_client(socket: TcpStream, db: TsDbHandle) -> PeerHandle {
    info!("Got incoming socket! {:?}", socket);
    let (kill_switch, kill_switch_endpoint) = oneshot::channel::<()>();
    let join_handle = tokio::spawn(async { peer_prog(db, socket, kill_switch_endpoint).await });
    PeerHandle {
        join_handle,
        kill_switch,
    }
}

async fn peer_prog(db: TsDbHandle, socket: TcpStream, kill_switch_endpoint: oneshot::Receiver<()>) {
    let mut framed_stream = Framed::new(socket, LengthDelimitedCodec::new()).fuse();
    let mut kill_switch_endpoint = kill_switch_endpoint.fuse();

    loop {
        futures::select! {
            optional_packet = framed_stream.next() => {
                if let Some(packet) = optional_packet {
                    let packet = packet.unwrap();
                    process_packet(&db, &packet);
                } else {
                    info!("Client disconnect!");
                    break;
                }
            },
            x = kill_switch_endpoint => {
                info!("Killing client connection!");
                break;
            }
        }
    }
}

/// Process a single message.
fn process_packet(db: &TsDbHandle, packet: &[u8]) {
    // debug!("Got: {:?}", &packet);

    // try to decode cbor package:
    let batch: SampleBatch = serde_cbor::from_slice(&packet).unwrap();
    // println!("DAATAA: {:?}", batch.size());

    // Append the samples to the database:
    // TODO: instead of direct database access
    // get access to a queue which is processed elsewhere into the database.
    db.add_values(batch.name(), batch.to_samples());
}
