use std::io::Write;
use std::net::TcpStream;

use super::payload::SampleBatch;

pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect(addr).unwrap();
        TcpClient { stream }
    }

    pub fn send_sample(&mut self, name: &str, timestamp: f64, value: f64) {
        // Encode data
        let payload = SampleBatch::new_sample(name.to_string(), timestamp, value);
        let data = serde_cbor::to_vec(&payload).unwrap();
        let mut header: [u8; 4] = [0; 4];
        let size: u32 = data.len() as u32;
        header[0] = ((size >> 24) & 0xff) as u8;
        header[1] = ((size >> 16) & 0xff) as u8;
        header[2] = ((size >> 8) & 0xff) as u8;
        header[3] = ((size) & 0xff) as u8;
        self.write(&header);
        self.write(&data);
    }

    fn write(&mut self, buffer: &[u8]) {
        self.stream.write(buffer).unwrap();
    }
}
