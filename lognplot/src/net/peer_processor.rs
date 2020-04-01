//! Process events from remotely connected peers

use crate::tracer::{AnyTracer, Tracer};
use futures::channel::{mpsc, oneshot};
use futures::{FutureExt, StreamExt};
use std::sync::Arc;
use tokio::task::JoinHandle;

pub struct PeerEventProcessorHandle {
    kill_switch: oneshot::Sender<()>,
    join_handle: JoinHandle<()>,
}

impl PeerEventProcessorHandle {
    pub async fn stop(self) -> std::io::Result<()> {
        info!("Stopping peer processor");
        match self.kill_switch.send(()) {
            Err(_) => {
                info!("Peer event processor already stopped");
            }
            Ok(_) => {
                info!("Peer event processor stopped");
            }
        }
        self.join_handle.await?;
        Ok(())
    }
}

pub enum PeerEvent {
    BytesReceived(usize),
    // SamplesReceived(usize),
    // Finished,
}

pub fn start_peer_event_processor(
    peer_event_stream: mpsc::UnboundedReceiver<PeerEvent>,
    perf_tracer: Arc<AnyTracer>,
) -> PeerEventProcessorHandle {
    let (kill_switch, kill_switch_endpoint) = oneshot::channel::<()>();

    let join_handle = tokio::spawn(async {
        process_peer_events(kill_switch_endpoint, peer_event_stream, perf_tracer).await
    });

    PeerEventProcessorHandle {
        kill_switch,
        join_handle,
    }
}

async fn process_peer_events(
    kill_switch_endpoint: oneshot::Receiver<()>,
    mut peer_event_stream: mpsc::UnboundedReceiver<PeerEvent>,
    perf_tracer: Arc<AnyTracer>,
) {
    let mut kill_switch_endpoint = kill_switch_endpoint.fuse();

    let mut total_bytes = 0;
    // let mut total_samples = 0;

    perf_tracer.log_metric("total_bytes", std::time::Instant::now(), total_bytes as f64);
    // perf_tracer.log_metric(
    //     "total_samples",
    //     std::time::Instant::now(),
    //     total_samples as f64,
    // );

    loop {
        futures::select! {
            optional_peer_event = peer_event_stream.next() => {
                // println!("Event!");
                if let Some(peer_event) = optional_peer_event {
                    match peer_event {
                        PeerEvent::BytesReceived(amount) => {
                            total_bytes += amount;
                            perf_tracer.log_metric("total_bytes", std::time::Instant::now(), total_bytes as f64);
                        }
                        // PeerEvent::SamplesReceived(amount) => {
                        //     total_samples += amount;
                        //     perf_tracer.log_metric("total_samples", std::time::Instant::now(), total_samples as f64);
                        // }
                        // PeerEvent::Finished => {
                            // TODO: what to do?
                        // }
                    }
                } else {
                    // TODO: what to do in this case?
                }
            },
            x = kill_switch_endpoint => {
                info!("Killing peer event processing connection!");
                break;
            }
        }
    }
}
