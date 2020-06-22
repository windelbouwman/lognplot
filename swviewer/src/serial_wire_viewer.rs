//! Serial wire viewer application state.
//!

use crate::coresight::{
    CoreSightError, MemoryAccess, MemoryAddress, Target, TraceDataDecoder, TracePacket,
};
use crate::stlink::{get_stlink, StLink, StLinkError, StLinkMode};
use crate::trace_var::{TraceVar, VarType};
// use crate::ui_logger::UiLogger;
use crate::usbutil::lsusb;
use lognplot::net::TcpClient;
use scroll::{Pread, LE};
use std::collections::HashMap;
use std::sync::mpsc;

/// Serial wire viewer application state.
struct SerialWireViewer<'m> {
    /// Handle to st-link device:
    st_link: &'m StLink,

    target: Target<'m, StLink>,

    /// lognplot GUI client.
    lognplot_client: TcpClient,

    core_freq_hz: u32,

    trace_configuration: HashMap<usize, TraceVar>,

    timestamp: f64,

    decoder: TraceDataDecoder,
}

impl<'m> SerialWireViewer<'m> {
    fn new(
        // trace_vars: Vec<TraceVar>,
        st_link: &'m StLink,
        target: Target<'m, StLink>,
        lognplot_client: TcpClient,
        core_freq_hz: u32,
    ) -> Self {
        SerialWireViewer {
            st_link,
            target,
            // trace_vars,
            lognplot_client,
            core_freq_hz,
            trace_configuration: HashMap::new(),
            timestamp: 0.0,
            decoder: TraceDataDecoder::new(),
        }
    }

    /// Configure tracing on both ends of the SWO line.
    ///
    /// This means the st-link must be configured, and
    /// the ARM core as well.
    fn trace_enable(&self) -> SerialWireViewerResult<()> {
        // SWO data pin bit-rate frequency in Hz:
        let swo_trace_hz = 2_000_000;

        self.st_link.trace_enable(swo_trace_hz)?;

        self.target.setup_tracing(self.core_freq_hz, swo_trace_hz)?;

        // disable all channels:
        self.target.stop_all_tracing()?;

        Ok(())
    }

    fn enable_trace_channel(
        &mut self,
        trace_var: Option<TraceVar>,
        channel: usize,
    ) -> SerialWireViewerResult<()> {
        if let Some(var) = trace_var {
            self.target
                .start_trace_memory_address(var.address, channel)?;
            self.trace_configuration.insert(channel, var);
        } else {
            self.target.stop_trace(channel)?;
            self.trace_configuration.remove(&channel);
        }
        Ok(())
    }

    /// Perform a single poll action via USB.
    fn poll_trace_data(&mut self) -> SerialWireViewerResult<()> {
        let trace_byte_count = self.st_link.get_trace_count()?;
        if trace_byte_count > 0 {
            debug!("Trace bytes: {}", trace_byte_count);
            debug!("Reading trace data.");
            let trace_data = self.st_link.read_trace_data(trace_byte_count)?;
            debug!("Trace data: {:?}", trace_data);

            self.decoder.feed(trace_data);
            while let Some(packet) = self.decoder.pull() {
                // println!("Packet: {:?}", packet);
                // info!("Packet: {:?}", packet);
                self.process_packet(packet)?;
            }
        }

        Ok(())
    }

    /// Process a single trace packet
    fn process_packet(&mut self, packet: TracePacket) -> SerialWireViewerResult<()> {
        match packet {
            TracePacket::TimeStamp { tc, ts } => {
                debug!("Timestamp packet: tc={} ts={}", tc, ts);
                let mut time_delta: f64 = ts as f64;
                let core_freq_hz: f64 = self.core_freq_hz as f64;

                // Divide by core clock frequency to go from ticks to seconds.
                time_delta /= core_freq_hz;
                self.timestamp += time_delta;
                // println!("TIme: {}", timestamp);
            }
            TracePacket::DwtData { id, payload } => {
                // TODO: queue?
                debug!("Dwt: id={} payload={:?}", id, payload);
                // timestamp += 1.0;

                if id & 24 == 16 {
                    // ID16 to ID23 --> data trace!
                    let write_not_read: bool = id & 1 == 1;
                    let comparator: usize = (id & 6) >> 1;
                    // println!("write: {} comparator: {}", write_not_read, comparator);
                    // TODO: grab timestamp
                    // New memory value!

                    // only emit written values:
                    if write_not_read {
                        // Only transmit value when we have a corresponding variable configured:
                        if let Some(var) = self.trace_configuration.get(&comparator) {
                            let value = extract_value_from_payload(&payload, &var.typ);
                            trace!("VAr {} = {}", var.name, value);

                            self.lognplot_client
                                .send_sample(&var.name, self.timestamp, value)?;
                        }
                    }
                }
            }
            TracePacket::ItmData { id, payload } => {
                // Simply print all data from ITM as ascii
                use std::ascii::escape_default;
                let mut visible = String::new();
                for b in payload {
                    let part: Vec<u8> = escape_default(b).collect();
                    visible.push_str(&String::from_utf8(part).unwrap());
                }
                print!("{}", visible);

                // println!("")
            }
            _ => {
                info!("Trace packet: {:?}", packet);
            }
        }

        Ok(())
    }
}

fn extract_value_from_payload(payload: &[u8], typ: &VarType) -> f64 {
    match typ {
        VarType::Int32 => {
            let value: i32 = payload.pread_with(0, LE).unwrap();
            value as f64
        }
        VarType::Int16 => {
            let value: i16 = payload.pread_with(0, LE).unwrap();
            value as f64
        }
        VarType::Int8 => {
            let value: i8 = payload.pread_with(0, LE).unwrap();
            value as f64
        }
        VarType::Uint32 => {
            let value: u32 = payload.pread_with(0, LE).unwrap();
            value as f64
        }
        VarType::Uint16 => {
            let value: u16 = payload.pread_with(0, LE).unwrap();
            value as f64
        }
        VarType::Uint8 => {
            let value: u8 = payload.pread_with(0, LE).unwrap();
            value as f64
        }
        VarType::Float32 => {
            let value: f32 = payload.pread_with(0, LE).unwrap();
            value as f64
        }
    }
}

pub fn data_thread(
    lognplot_uri: &str,
    core_freq_hz: u32,
    cmd_rx: mpsc::Receiver<UiThreadCommand>,
) -> SerialWireViewerResult<()> {
    // get st link:
    lsusb()?;
    let st_link = get_stlink()?;
    initialize_st_link(&st_link)?;

    // Attach target upon st-link device:
    let mut target = Target::new(&st_link);
    target.read_debug_components()?;

    // connect to lognplot GUI:
    let lognplot_client = TcpClient::new(lognplot_uri)?;

    // Create app contraption:
    let mut app = SerialWireViewer::new(&st_link, target, lognplot_client, core_freq_hz);

    info!("All systems GO!");

    app.trace_enable()?;
    loop {
        app.poll_trace_data()?;
        if let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                UiThreadCommand::Stop => {
                    break;
                }
                UiThreadCommand::ConfigChannel { var, channel } => {
                    app.enable_trace_channel(var, channel)?;
                    info!("Configured channel {}", channel + 1);
                }
            }
        }
    }

    app.st_link.reset_core()?;
    app.st_link.exit_debug_mode()?;

    Ok(())
}

/// Initial initializations with the ST link
fn initialize_st_link(st_link: &StLink) -> SerialWireViewerResult<()> {
    let version = st_link.get_version()?;
    info!("ST-link Version: {:?}", version);

    enter_proper_mode(st_link)?;
    read_chip_id(st_link)?;

    Ok(())
}

fn enter_proper_mode(st_link: &StLink) -> SerialWireViewerResult<()> {
    let mut mode = st_link.get_mode()?;
    info!("Mode: {:?}", mode);
    if let StLinkMode::Dfu = mode {
        st_link.leave_dfu_mode()?;
        mode = st_link.get_mode()?;
        info!("Mode: {:?}", mode);
    }

    match mode {
        StLinkMode::Dfu | StLinkMode::Mass => {
            st_link.enter_debug_mode()?;
            mode = st_link.get_mode()?;
            info!("Mode: {:?}", mode);
        }
        _ => {}
    }

    Ok(())
}

fn read_chip_id(st_link: &StLink) -> SerialWireViewerResult<()> {
    let address = 0xE004_2000; // Chip ID
    let value = st_link.read_debug32(address)?;
    info!("Chip ID is 0x{:08X}", value);
    Ok(())
}

/// Command send from the UI to the processing thread.
pub enum UiThreadCommand {
    Stop,
    ConfigChannel {
        var: Option<TraceVar>,
        channel: usize,
    },
}

#[derive(Debug)]
pub enum SerialWireViewerError {
    Usb(rusb::Error),
    StLink(StLinkError),
    CoreSight(CoreSightError),
    Io(std::io::Error),
    Dwarf(gimli::Error), // Other(String),
}

impl From<rusb::Error> for SerialWireViewerError {
    fn from(error: rusb::Error) -> Self {
        SerialWireViewerError::Usb(error)
    }
}

impl From<StLinkError> for SerialWireViewerError {
    fn from(error: StLinkError) -> Self {
        SerialWireViewerError::StLink(error)
    }
}

impl From<std::io::Error> for SerialWireViewerError {
    fn from(error: std::io::Error) -> Self {
        SerialWireViewerError::Io(error)
    }
}

impl From<gimli::Error> for SerialWireViewerError {
    fn from(error: gimli::Error) -> Self {
        SerialWireViewerError::Dwarf(error)
    }
}

impl From<CoreSightError> for SerialWireViewerError {
    fn from(error: CoreSightError) -> Self {
        SerialWireViewerError::CoreSight(error)
    }
}

pub type SerialWireViewerResult<T> = Result<T, SerialWireViewerError>;

impl MemoryAccess for StLink {
    fn read_u32(&self, address: MemoryAddress) -> Result<u32, String> {
        self.read_debug32(address)
            .map_err(|e| format!("st-link error: {:?}", e))
    }

    fn write_u32(&self, address: MemoryAddress, value: u32) -> Result<(), String> {
        self.write_debug32(address, value)
            .map_err(|e| format!("st-link error: {:?}", e))
    }
}
