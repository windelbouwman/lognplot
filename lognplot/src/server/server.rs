//! TCP based server for data

use futures::sync::oneshot;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::net::TcpListener;
use tokio::prelude::*;

use super::peer::{process_client, PeerHandle};
use crate::tsdb::TsDbHandle;

/// This is a handle to a started TCP server.
/// You can use this handle to stop the server.
pub struct ServerHandle {
    // This is the thread in which the server will block
    thread: thread::JoinHandle<()>,

    // This switch can be used to trigger shutdown of the server.
    kill_switch: oneshot::Sender<()>,

    // A list of active peers
    peers: Arc<Mutex<Vec<PeerHandle>>>,
}

impl ServerHandle {
    pub fn stop(self) {
        self.kill_switch.send(()).unwrap();

        for peer in self.peers.lock().unwrap().drain(..) {
            peer.stop();
        }

        self.thread.join().unwrap();
    }
}

pub fn run_server(db: TsDbHandle) -> ServerHandle {
    let port = 12345;
    info!("Starting up server at port {} with db {:?}!", port, db);

    let addr = format!("127.0.0.1:{}", port);
    let addr = addr.parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let peers: Arc<Mutex<Vec<PeerHandle>>> = Arc::new(Mutex::new(vec![]));

    let server_peers = peers.clone();
    let server_task = listener
        .incoming()
        .for_each(move |socket| {
            let peer = process_client(0, socket, db.clone());
            server_peers.lock().unwrap().push(peer);
            Ok(())
        })
        .map_err(|err| {
            println!("Error in accept: {:?}", err);
        });

    // Construct contraption which enabled the graceful quit of the server.
    let (kill_switch, c) = futures::sync::oneshot::channel::<()>();
    let c = c.map_err(|_| ());

    // let oneshot::
    let task = server_task.select(c).map(|_| ()).map_err(|_| ());

    let thread = thread::spawn(move || {
        info!("Server thread begun!!!");
        tokio::run(task);
        info!("Server finished!!!");
    });

    info!("Server listening on {:?}", addr);

    ServerHandle {
        thread,
        kill_switch,
        peers,
    }
}
