//! Handle a single peer via tcp socket.

use futures::sync::oneshot;
use serde::Deserialize;
use tokio::codec::{Framed, LengthDelimitedCodec};
use tokio::net::TcpStream;
use tokio::prelude::*;

use crate::time::TimeStamp;
use crate::tsdb::{Observation, Sample, TsDbHandle};

/// A handle to a peer connection
pub struct PeerHandle {
    kill_switch: oneshot::Sender<()>,
}

impl PeerHandle {
    pub fn stop(self) {
        self.kill_switch.send(()).unwrap();
    }
}

/// A chunk of data at fixed sample rate.
#[derive(Deserialize, Debug)]
struct SampleBatch {
    /// The name of the signal.
    name: String,

    /// Timestamp of the first sample
    t0: f64,

    /// Spacing in time of the samples.
    dt: f64,

    /// The data points
    data: Vec<f64>,
}

impl SampleBatch {
    /// Convert a batch of samples received over the wire to
    /// a vector of samples
    fn to_samples(&self) -> Vec<Observation<Sample>> {
        // let start_time = self.t0;
        self.data
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let t = self.t0 + self.dt * index as f64;
                let timestamp = TimeStamp::new(t);
                Observation::new(timestamp, Sample::new(*value))
            })
            .collect()
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

/// Handle a single client
pub fn process_client(_counter: usize, socket: TcpStream, db: TsDbHandle) -> PeerHandle {
    info!("Got incoming socket! {:?}", socket);

    let (_framed_sink, framed_stream) = Framed::new(socket, LengthDelimitedCodec::new()).split();
    // TODO: use two way communication to give feedback?

    let client_task = framed_stream
        .for_each(move |packet| {
            // debug!("Got: {:?}", &packet);

            // try to decode cbor package:
            let batch: SampleBatch = serde_cbor::from_slice(&packet).unwrap();
            println!("DAATAA: {:?}", batch.size());

            // Append the samples to the database:
            // let samples: Vec<Sample> = pts.into_iter().map(|v| {
            // to_samples
            // }).collect();
            // TODO: instead of direct database access
            // get access to a queue which is processed elsewhere into the database.
            db.add_values(&batch.name, batch.to_samples());
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
