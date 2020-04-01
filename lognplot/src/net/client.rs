use std::io::Write;
use std::net::TcpStream;

use super::payload::SampleBatch;

/// A TCP client to send logging events over TCP.
pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub fn new(addr: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let client = TcpClient { stream };
        Ok(client)
    }

    /// Close the connection gracefully.
    pub fn close(&self) -> std::io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }

    /// Transmit a single sample over tha wire.
    pub fn send_sample(&mut self, name: &str, timestamp: f64, value: f64) -> std::io::Result<()> {
        let payload = SampleBatch::new_sample(name.to_owned(), timestamp, value);
        self.write_sample_batch(payload)
    }

    /// Transmit a batch of samples.
    pub fn send_samples(&mut self, name: &str, samples: Vec<(f64, f64)>) -> std::io::Result<()> {
        let payload = SampleBatch::new_samples(name.to_owned(), samples);
        self.write_sample_batch(payload)
    }

    /// Send a batch equally spaced samples.
    ///
    /// This can be useful if samples as gathered in batched.
    pub fn send_sampled_samples(
        &mut self,
        name: &str,
        t0: f64,
        dt: f64,
        values: Vec<f64>,
    ) -> std::io::Result<()> {
        let payload = SampleBatch::new_sampled_data(name.to_owned(), t0, dt, values);
        self.write_sample_batch(payload)
    }

    /// Send a single text event
    pub fn send_text(&mut self, name: &str, timestamp: f64, text: String) -> std::io::Result<()> {
        let payload = SampleBatch::new_text(name.to_owned(), timestamp, text);
        self.write_sample_batch(payload)
    }

    fn write_sample_batch(&mut self, payload: SampleBatch) -> std::io::Result<()> {
        // Encode data
        let data = serde_cbor::to_vec(&payload).unwrap();
        self.write_blob(data)
    }

    /// Write a length prefixed blob of data.
    fn write_blob(&mut self, data: Vec<u8>) -> std::io::Result<()> {
        let mut header: [u8; 4] = [0; 4];
        let size: u32 = data.len() as u32;
        header[0] = ((size >> 24) & 0xff) as u8;
        header[1] = ((size >> 16) & 0xff) as u8;
        header[2] = ((size >> 8) & 0xff) as u8;
        header[3] = ((size) & 0xff) as u8;
        self.write(&header)?;
        self.write(&data)
    }

    fn write(&mut self, buffer: &[u8]) -> std::io::Result<()> {
        self.stream.write_all(buffer)
    }
}
