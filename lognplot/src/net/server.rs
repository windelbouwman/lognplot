//! TCP based server for data

use futures::channel::oneshot;
use futures::{FutureExt, StreamExt};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::net::TcpListener;

use super::peer::{process_client, PeerHandle};
use crate::tsdb::TsDbHandle;

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
        self.kill_switch.send(()).unwrap();
        self.thread.join().unwrap();
    }
}

pub fn run_server(db: TsDbHandle) -> ServerHandle {
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
            server_prog(db, kill_switch_receiver).await.unwrap();
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
    kill_switch_receiver: oneshot::Receiver<()>,
) -> std::io::Result<()> {
    let peers: Arc<Mutex<Vec<PeerHandle>>> = Arc::new(Mutex::new(vec![]));
    let port: u16 = 12345;
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

    loop {
        futures::select! {
            x = kill_switch_receiver => {
                info!("Server shutdown by kill switch.");
                break;
            },
            optional_new_client = incoming.next() => {
                if let Some(new_client) = optional_new_client {
                    let peer_socket = new_client?;
                    info!("Client connected!");
                    let peer = process_client(peer_socket, db.clone());
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

    Ok(())
}
