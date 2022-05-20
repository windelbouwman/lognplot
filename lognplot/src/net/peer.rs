//! Handle a single peer via tcp socket.

use super::payload::SampleBatch;
use super::peer_processor::PeerEvent;
use crate::tsdb::TsDbHandle;
use futures::channel::{mpsc, oneshot};
use futures::{FutureExt, StreamExt};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

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
pub fn process_client(
    socket: TcpStream,
    db: TsDbHandle,
    peer_event_sink: mpsc::UnboundedSender<PeerEvent>,
) -> PeerHandle {
    info!("Got incoming socket! {:?}", socket);
    let (kill_switch, kill_switch_endpoint) = oneshot::channel::<()>();
    let join_handle = tokio::spawn(async {
        let res = peer_prog(db, socket, kill_switch_endpoint, peer_event_sink).await;
        if let Err(err) = res {
            error!("Error in peer: {:?}", err);
        }
    });
    PeerHandle {
        join_handle,
        kill_switch,
    }
}

async fn peer_prog(
    db: TsDbHandle,
    socket: TcpStream,
    kill_switch_endpoint: oneshot::Receiver<()>,
    peer_event_sink: mpsc::UnboundedSender<PeerEvent>,
) -> std::io::Result<()> {
    let mut framed_stream = Framed::new(socket, LengthDelimitedCodec::new()).fuse();
    let mut kill_switch_endpoint = kill_switch_endpoint.fuse();

    loop {
        futures::select! {
            optional_packet = framed_stream.next() => {
                if let Some(packet) = optional_packet {
                    let packet = packet?;
                    process_packet(&db, &packet, &peer_event_sink);
                } else {
                    info!("Client disconnect!");
                    break;
                }
            },
            _x = kill_switch_endpoint => {
                info!("Killing client connection!");
                break;
            }
        }
    }

    Ok(())
}

/// Process a single message.
fn process_packet(
    db: &TsDbHandle,
    packet: &[u8],
    peer_event_sink: &mpsc::UnboundedSender<PeerEvent>,
) {
    // debug!("Got: {:?}", &packet);

    peer_event_sink
        .unbounded_send(PeerEvent::BytesReceived(packet.len()))
        .unwrap();

    // try to decode cbor package:
    match serde_cbor::from_slice::<SampleBatch>(&packet) {
        Ok(batch) => {
            // let batch: SampleBatch =
            // println!("DAATAA: {:?}", batch.size());
            batch.to_db(db);
        }
        Err(err) => {
            error!("Error decoding packet: {:?}", err);
        }
    }
}
