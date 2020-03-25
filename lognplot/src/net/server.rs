//! TCP based server for data

use super::peer::{process_client, PeerHandle};
use super::peer_processor::start_peer_event_processor;
use crate::tracer::{AnyTracer, Tracer};
use crate::tsdb::TsDbHandle;
use futures::channel::{mpsc, oneshot};
use futures::{FutureExt, StreamExt};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::net::TcpListener;

/// This is a handle to a started TCP server.
/// You can use this handle to stop the server.
pub struct ServerHandle {
    // This is the thread in which the server will block
    thread: thread::JoinHandle<()>,

    // This switch can be used to trigger shutdown of the server.
    kill_switch: oneshot::Sender<()>,
}

impl ServerHandle {
    pub fn stop(self) {
        match self.kill_switch.send(()) {
            Err(_) => {
                debug!("Server thread already stopped");
            }
            Ok(_) => {
                debug!("Server thread stopped.");
            }
        }
        self.thread.join().unwrap();
    }
}

pub fn run_server(db: TsDbHandle, port: u16, perf_tracer: Arc<AnyTracer>) -> ServerHandle {
    let (kill_switch, kill_switch_receiver) = oneshot::channel::<()>();
    let thread = thread::spawn(move || {
        info!("Server thread begun!!!");
        let mut runtime = tokio::runtime::Builder::new()
            .basic_scheduler()
            .enable_all()
            .thread_name("Tokio-server-thread")
            .build()
            .unwrap();

        runtime.block_on(async {
            if let Err(err) = server_prog(db, port, perf_tracer, kill_switch_receiver).await {
                error!("Server stopped with error: {}", err);
            }
        });

        info!("Server finished!!!");
    });

    ServerHandle {
        thread,
        kill_switch,
    }
}

async fn server_prog(
    db: TsDbHandle,
    port: u16,
    perf_tracer: Arc<AnyTracer>,
    kill_switch_receiver: oneshot::Receiver<()>,
) -> std::io::Result<()> {
    let peers: Arc<Mutex<Vec<PeerHandle>>> = Arc::new(Mutex::new(vec![]));
    info!("Starting up server at port {}!", port);
    // let addr = format!("127.0.0.1:{}", port);
    // let addr: std::net::SocketAddr = addr.parse().unwrap();
    let ip = std::net::Ipv6Addr::UNSPECIFIED;
    let addr = std::net::SocketAddrV6::new(ip, port, 0, 0);
    let std_listener = std::net::TcpListener::bind(addr)?;
    // info!("a: only v6={}", std_listener.only_v6()?);
    // let mut listener = TcpListener::bind(&addr).await?;
    let mut listener = TcpListener::from_std(std_listener)?;
    info!("Server listening on {:?}", addr);
    let mut kill_switch_receiver = kill_switch_receiver.fuse();
    let mut incoming = listener.incoming().fuse();

    let (peer_event_sink, peer_event_rx) = mpsc::unbounded();
    let peer_processor_handle = start_peer_event_processor(peer_event_rx, perf_tracer.clone());

    loop {
        perf_tracer.log_metric(
            "peers",
            std::time::Instant::now(),
            peers.lock().unwrap().len() as f64,
        );

        futures::select! {
            x = kill_switch_receiver => {
                info!("Server shutdown by kill switch.");
                break;
            },
            optional_new_client = incoming.next() => {
                if let Some(new_client) = optional_new_client {
                    let peer_socket = new_client?;
                    info!("Client connected!");
                    let peer = process_client(peer_socket, db.clone(), peer_event_sink.clone());
                    peers.lock().unwrap().push(peer);
                } else {
                    info!("No more incoming connections.");
                    break;
                }
            },
        };
    }

    info!("Shutting down peer connections");

    for peer in peers.lock().unwrap().drain(..) {
        peer.stop().await?;
    }

    peer_processor_handle.stop().await?;

    Ok(())
}
