use lognplot::net::TcpClient;
use regex::Regex;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    // Regex matching strace --timestamps=unix,ns style output:
    let re = Regex::new(r"^(\d+\.\d+) (.+)$").unwrap();

    let trace_name = "strace";
    let mut tcp_client = TcpClient::new("localhost:12345").unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let Some(m) = re.captures(&line) {
            let timestamp: f64 = f64::from_str(m.get(1).unwrap().as_str()).unwrap();
            let msg: String = m.get(2).unwrap().as_str().to_owned();
            tcp_client.send_text(trace_name, timestamp, msg).unwrap();
        } else {
            println!("Error in matching: {}", line);
        }
    }
}
