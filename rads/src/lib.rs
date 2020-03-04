//! BECKHOFF ADS protocol client implementation
//!
//!

use scroll::{Pread, Pwrite, LE};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::FromStr;

pub fn connect(addr: std::net::SocketAddr) -> AdsResult<Ads> {
    let s = TcpStream::connect(addr)?;

    println!("s = ");

    Ok(Ads::new(s))
}

/// ADS client.
///
/// Implements all ADS commands as described
/// [here](https://infosys.beckhoff.com/english.php?content=../content/1033/tc3_ads_intro/115844363.html&id=1036147080957164813)
///
pub struct Ads {
    socket: TcpStream,
    invoke_id_counter: u32,
}

#[derive(Debug)]
pub enum AdsError {
    IoError(std::io::Error),
    Other(String),
}

type AdsResult<T> = Result<T, AdsError>;

impl From<std::io::Error> for AdsError {
    fn from(e: std::io::Error) -> Self {
        AdsError::IoError(e)
    }
}

impl Ads {
    fn new(socket: TcpStream) -> Self {
        Ads {
            socket,
            invoke_id_counter: 0,
        }
    }

    /// ADS Read Device info
    ///
    /// Read name and version for the ADS device.
    pub fn ads_read_device_info(&mut self) -> AdsResult<AdsDeviceInfo> {
        // TODO: figure out sensible ams net ID's:
        let source = AmsAddress::new(AmsNetId::from_str("127.0.0.1.2.3").unwrap(), 1337);
        let target = AmsAddress::new(AmsNetId::from_str("127.0.0.1.2.3").unwrap(), 1337);

        // empty request payload:
        let data = vec![];

        let request = AmsRequest::new(source, target, ADS_READ_DEVICE_INFO, data);
        let response_data = self.send_request(request)?;

        self.check_response(&response_data)?;

        assert!(response_data.len() == 24);

        let info = {
            let major_version = response_data.pread_with::<u8>(4, LE).expect("Works");
            let minor_version = response_data.pread_with::<u8>(5, LE).expect("Works");
            let version_build = response_data.pread_with::<u16>(6, LE).expect("Works");

            // TODO:
            // let name = packet.data[8..].to_vec();
            let device_name = "Dummy!".to_owned();

            AdsDeviceInfo {
                major_version,
                minor_version,
                version_build,
                device_name,
            }
        };

        Ok(info)
    }

    /// ADS Read
    ///
    /// Read some data from an ADS device. Data is addressed by index group and index offset.
    pub fn ads_read(
        &mut self,
        index_group: u32,
        index_offset: u32,
        length: u32,
    ) -> AdsResult<Vec<u8>> {
        // TODO: figure out sensible ams net ID's:
        let source = AmsAddress::new(AmsNetId::from_str("127.0.0.1.2.3").unwrap(), 1337);
        let target = AmsAddress::new(AmsNetId::from_str("127.0.0.1.2.3").unwrap(), 1337);

        let mut request_data = vec![0; 12];
        request_data
            .pwrite_with::<u32>(index_group, 0, LE)
            .expect("Works fine.");
        request_data
            .pwrite_with::<u32>(index_offset, 4, LE)
            .expect("Works fine.");
        request_data
            .pwrite_with::<u32>(length, 8, LE)
            .expect("Works fine.");

        let request = AmsRequest::new(source, target, ADS_READ, request_data);
        let response_data = self.send_request(request)?;

        self.check_response(&response_data)?;

        assert!(response_data.len() >= 8);

        let data_length = response_data.pread_with::<u32>(4, LE).expect("Works");
        let data = response_data[8..].to_vec();
        assert!(data.len() == data_length as usize);

        Ok(data)
    }

    /// ADS Write
    ///
    /// Write data to an ADS device at a certain index group and index offset.
    pub fn ads_write(
        &mut self,
        index_group: u32,
        index_offset: u32,
        data: Vec<u8>,
    ) -> AdsResult<()> {
        // TODO: figure out sensible ams net ID's:
        let source = AmsAddress::new(AmsNetId::from_str("127.0.0.1.2.3").unwrap(), 1337);
        let target = AmsAddress::new(AmsNetId::from_str("127.0.0.1.2.3").unwrap(), 1337);

        // pack data:
        let mut request_data = vec![0; 12];
        request_data
            .pwrite_with::<u32>(index_group, 0, LE)
            .expect("Works fine.");
        request_data
            .pwrite_with::<u32>(index_offset, 4, LE)
            .expect("Works fine.");
        request_data
            .pwrite_with::<u32>(data.len() as u32, 8, LE)
            .expect("Works fine.");
        request_data.append(&mut data.clone());

        let request = AmsRequest::new(source, target, ADS_WRITE, request_data);
        let response_data = self.send_request(request)?;

        self.check_response(&response_data)?;

        Ok(())
    }

    /// Given some response data, extract the result field and check it.
    fn check_response(&self, response_data: &[u8]) -> AdsResult<()> {
        let result = response_data.pread_with::<u32>(0, LE).expect("Works");
        assert!(result == 0);
        Ok(())
    }

    pub fn ads_read_state(&self) {
        unimplemented!("TODO!");
    }

    /// Generate a new ID for a request invocation.
    fn get_invoke_id(&mut self) -> u32 {
        let invoke_id = self.invoke_id_counter;
        self.invoke_id_counter += 1;
        invoke_id
    }

    /// Send a request and receive a corresponding response.
    fn send_request(&mut self, request: AmsRequest) -> AdsResult<Vec<u8>> {
        let invoke_id = self.get_invoke_id();

        // Bit 7 marks TCP(0) or UDP(1)
        // Bit 3 marks ADS command
        // bit 0 marks request(0) or response(1)
        let state_flags = 0x4; // 0x4 = ADS command request over TCP

        // 0 = no error :)
        let error_code = 0;

        let packet = AmsPacket::new(
            request.target,
            request.source,
            request.command_id,
            state_flags,
            error_code,
            invoke_id,
            request.data,
        );
        self.send_ams_packet(packet)?;

        // Wait for response now.
        let packet = self.receive_ams_packet()?;

        // verify invoke ID:
        if packet.invoke_id != invoke_id {
            panic!("invoke id is incorrect");
        }

        if packet.command_id != request.command_id {
            panic!("Command id mismatch!");
        }

        let response_data = packet.data;
        Ok(response_data)
    }

    /// Receive a single AMS packet.
    fn receive_ams_packet(&mut self) -> AdsResult<AmsPacket> {
        let frame = self.receive_ams_tcp_frame()?;
        let packet = AmsPacket::from_frame(frame);
        Ok(packet)
    }

    /// Receive a single AMS/TCP frame.
    fn receive_ams_tcp_frame(&mut self) -> AdsResult<Vec<u8>> {
        let mut header = vec![0; 6];
        self.socket.read_exact(&mut header)?;
        let _zero = header.pread_with::<u16>(0, LE).expect("Works!");
        // TODO: check zero == 0?
        let frame_size = header.pread_with::<u32>(2, LE).expect("Works!");

        let mut frame = vec![0; frame_size as usize];
        self.socket.read_exact(&mut frame)?;
        Ok(frame)
    }

    /// Send an AMS request.
    fn send_ams_packet(&mut self, packet: AmsPacket) -> AdsResult<()> {
        // Contrapt a frame with the packet:
        let frame: Vec<u8> = packet.into_frame();
        self.write_ams_tcp_frame(frame)?;
        Ok(())
    }

    /// Write a length prefixed frame onto tha wire.
    fn write_ams_tcp_frame(&mut self, mut data: Vec<u8>) -> AdsResult<()> {
        let mut buf: Vec<u8> = Vec::with_capacity(data.len() + 6);

        // Attach AMS/TCP header!
        // 2 bytes reserved, 4 bytes little endian length
        buf.resize(6, 0);
        buf.pwrite_with::<u16>(0, 0, LE).expect("must work");
        buf.pwrite_with::<u32>(data.len() as u32, 2, LE)
            .expect("must work");
        buf.append(&mut data);

        self.socket.write_all(&buf)?;
        Ok(())
    }
}

/// Commands:
const ADS_READ_DEVICE_INFO: u16 = 1;
const ADS_READ: u16 = 2;
const ADS_WRITE: u16 = 3;
const ADS_READ_STATE: u16 = 4;
const ADS_WRITE_CONTROL: u16 = 5;

const ADS_READ_WRITE: u16 = 9;

struct AmsRequest {
    source: AmsAddress,
    target: AmsAddress,
    command_id: u16,
    data: Vec<u8>,
}

impl AmsRequest {
    fn new(source: AmsAddress, target: AmsAddress, command_id: u16, data: Vec<u8>) -> Self {
        AmsRequest {
            source,
            target,
            command_id,
            data,
        }
    }
}

/// AMS packet
#[derive(Clone, Debug, PartialEq)]
struct AmsPacket {
    /// The destination AMS address
    target: AmsAddress,

    /// The source AMS address
    source: AmsAddress,

    command_id: u16,
    state_flags: u16,
    error_code: u32,
    invoke_id: u32,

    /// Payload data of the packet.
    data: Vec<u8>,
}

/// AMS net id.
///
/// Usually something like 127.0.0.1.2.3
/// Unrelated to IP address, but still very alike :)
#[derive(Clone, Debug, PartialEq)]
struct AmsNetId {
    digits: Vec<u8>,
}

impl AmsNetId {
    fn new(digits: Vec<u8>) -> Self {
        AmsNetId { digits }
    }

    // fn write(&)
}

#[derive(Debug)]
enum ParseAmsNetIdError {
    // id: String
    IntError(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for ParseAmsNetIdError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseAmsNetIdError::IntError(err)
    }
}

impl FromStr for AmsNetId {
    type Err = ParseAmsNetIdError;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let mut digits = vec![];
        for digit in id.split('.') {
            let d = u8::from_str(digit)?;
            digits.push(d);
        }

        Ok(AmsNetId { digits })
    }
}

/// A unique AMS address.
///
/// This consists of a net id and a port.
#[derive(Clone, Debug, PartialEq)]
struct AmsAddress {
    net_id: AmsNetId,
    port: u16,
}

impl AmsAddress {
    fn new(net_id: AmsNetId, port: u16) -> Self {
        AmsAddress { net_id, port }
    }

    fn write_at(&self, frame: &mut [u8], offset: usize) {
        for i in 0..6 {
            frame[offset + i] = self.net_id.digits[i];
        }
        frame
            .pwrite_with::<u16>(self.port, offset + 6, LE)
            .expect("Works");
    }

    /// Parse an AMS address from some byte array.
    fn read_at(frame: &[u8], offset: usize) -> Self {
        let digits = frame[offset..offset + 6].to_vec();
        let net_id = AmsNetId::new(digits);
        let port = frame.pread_with::<u16>(offset + 6, LE).expect("Works");
        AmsAddress { net_id, port }
    }
}

impl AmsPacket {
    /// Construct a new AMS packet
    fn new(
        target: AmsAddress,
        source: AmsAddress,
        command_id: u16,
        state_flags: u16,
        error_code: u32,
        invoke_id: u32,
        data: Vec<u8>,
    ) -> Self {
        AmsPacket {
            target,
            source,
            command_id,
            state_flags,
            error_code,
            invoke_id,
            data,
        }
    }

    /// Turn an AMS packet into a frame with a header before it.
    fn into_frame(mut self) -> Vec<u8> {
        let data_length = self.data.len();

        let mut frame: Vec<u8> = Vec::with_capacity(data_length + 32);

        // Fill AMS header:
        frame.resize(32, 0);

        self.target.write_at(&mut frame, 0);
        self.source.write_at(&mut frame, 8);

        frame
            .pwrite_with::<u16>(self.command_id, 16, LE)
            .expect("Works");
        frame
            .pwrite_with::<u16>(self.state_flags, 18, LE)
            .expect("Works");
        frame
            .pwrite_with::<u32>(data_length as u32, 20, LE)
            .expect("Works");

        frame
            .pwrite_with::<u32>(self.error_code, 24, LE)
            .expect("Works");
        frame
            .pwrite_with::<u32>(self.invoke_id, 28, LE)
            .expect("Works");

        // Add payload data:
        frame.append(&mut self.data);
        frame
    }

    /// Turn a single frame into an AMS packet.
    fn from_frame(frame: Vec<u8>) -> Self {
        let (header, data) = frame.split_at(32);
        assert!(header.len() == 32);

        let target = AmsAddress::read_at(&frame, 0);
        let source = AmsAddress::read_at(&frame, 8);

        let command_id = frame.pread_with::<u16>(16, LE).expect("Works");
        let state_flags = frame.pread_with::<u16>(18, LE).expect("Works");
        let data_length = frame.pread_with::<u32>(20, LE).expect("Works");

        let error_code = frame.pread_with::<u32>(24, LE).expect("Works");
        let invoke_id = frame.pread_with::<u32>(28, LE).expect("Works");

        // TODO: change assert into error?
        assert!(data_length + 32 == frame.len() as u32);
        assert!(data_length == data.len() as u32);

        AmsPacket {
            target,
            source,
            command_id,
            state_flags,
            error_code,
            invoke_id,
            data: data.to_vec(),
        }
    }
}

#[derive(Debug)]
pub struct AdsDeviceInfo {
    major_version: u8,
    minor_version: u8,
    version_build: u16,
    device_name: String,
}

struct AmsReadRequest {
    index_group: u32,
    index_offset: u32,
    length: u32,
}

struct AmsReadResponse {
    result: u32,
    length: u32,
    data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::{AmsAddress, AmsNetId, AmsPacket};
    use std::str::FromStr;

    #[test]
    fn ams_address_serialization_roundtrips() {
        let addr1 = AmsAddress::new(AmsNetId::from_str("5.6.7.8.9.10").unwrap(), 1337);
        let addr2 = AmsAddress::new(AmsNetId::from_str("5.6.7.8.9.10").unwrap(), 1337);
        let mut buf = vec![0; 8];
        addr1.write_at(&mut buf, 0);
        let addr3 = AmsAddress::read_at(&buf, 0);
        assert_eq!(addr1, addr2);
        assert_eq!(addr2, addr3);
    }

    #[test]
    fn ams_packet_serialization_roundtrips() {
        let target = AmsAddress::new(AmsNetId::from_str("5.6.7.8.9.10").unwrap(), 1337);
        let source = AmsAddress::new(AmsNetId::from_str("1.2.3.4.11.13").unwrap(), 442);
        let command_id = 53;
        let state_flags = 54;
        let error_code = 99;
        let data = vec![80, 82, 33];
        let invoke_id = 104;

        let packet1 = AmsPacket::new(
            target,
            source,
            command_id,
            state_flags,
            error_code,
            invoke_id,
            data,
        );
        let buf = packet1.clone().into_frame();
        let packet2 = AmsPacket::from_frame(buf);
        assert_eq!(packet1, packet2);
    }
}
