//! Interfacing with ST-link v2 device!

use bytes::{Buf, BufMut};
use std::io::Cursor;

pub fn find_st_link() -> rusb::Result<Option<rusb::Device<rusb::GlobalContext>>> {
    // ST-link v2 usb id's:
    let vid = 0x0483;
    let pid = 0x3748;

    let first_match = rusb::devices()?.iter().find(|d| {
        if let Ok(desc) = d.device_descriptor() {
            desc.vendor_id() == vid && desc.product_id() == pid
        } else {
            false
        }
    });

    Ok(first_match)
}

pub fn open_st_link(st_link_device: rusb::Device<rusb::GlobalContext>) -> StLinkResult<StLink> {
    let desc = st_link_device.device_descriptor()?;
    println!("Device description: {:?}", desc);
    println!("Num configs: {}", desc.num_configurations());

    let active_config = st_link_device.active_config_descriptor()?;
    println!("Active config: {:?}", active_config);

    let mut st_link_handle = st_link_device.open()?;
    st_link_handle.reset()?;
    st_link_handle.claim_interface(0)?;

    println!("St link opened!");

    let st_link = StLink::new(st_link_handle);

    // st_link_handle.close();

    Ok(st_link)
}

pub struct StLink {
    handle: rusb::DeviceHandle<rusb::GlobalContext>,
}

#[derive(Debug)]
pub enum StLinkError {
    Usb(rusb::Error),
    Other(String),
}

impl From<rusb::Error> for StLinkError {
    fn from(error: rusb::Error) -> Self {
        StLinkError::Usb(error)
    }
}

pub type StLinkResult<T> = Result<T, StLinkError>;

#[derive(Debug)]
pub struct StLinkVersion {
    stlink_version: u8,
    jtag_version: u8,
    swim_version: u8,
    pid: u16,
    vid: u16,
}

#[derive(Debug)]
pub enum StLinkMode {
    Dfu,
    Mass,
    Debug,
}

// Modes:
const MODE_DFU: u8 = 0;
const MODE_MASS: u8 = 1;
const MODE_DEBUG: u8 = 2;

// Commands:
const CMD_VERSION: u8 = 0xf1;
const CMD_DEBUG_COMMAND: u8 = 0xf2;
const CMD_DFU_COMMAND: u8 = 0xf3;
const CMD_GET_CURRENT_MODE: u8 = 0xf5;

// DFU commands:
const DFU_EXIT: u8 = 0x7;

// DEBUG COMMANDS:
const DEBUG_READ_U32: u8 = 0x7;
// const DEBUG_WRITE_U32: u8 = 0x8;
const DEBUG_ENTER: u8 = 0x20;
// const DEBUG_EXIT: u8 = 0x21;
const DEBUG_JTAG_WRITEDEBUG_32BIT: u8 = 0x35;
const DEBUG_JTAG_READDEBUG_32BIT: u8 = 0x36;

// debug mode enter parameters:
const DEBUG_ENTER_MODE_SWD: u8 = 0xa3;

impl StLink {
    fn new(handle: rusb::DeviceHandle<rusb::GlobalContext>) -> Self {
        StLink { handle }
    }

    /// Retrieve ST-link version.
    pub fn get_version(&self) -> StLinkResult<StLinkVersion> {
        let mut cmd: Vec<u8> = vec![0; 16];
        cmd[0] = CMD_VERSION;
        let res = self.xfer_cmd(&cmd, 6)?.expect("six bytes");

        // process version bytes:
        let stlink_version = res[0] >> 4;
        let jtag_version = ((res[0] & 0xf) << 2) | (res[1] >> 6);
        let swim_version = res[1] & 0x3f;

        let pid: u16 = ((res[3] as u16) << 8) | (res[2] as u16);
        let vid: u16 = ((res[5] as u16) << 8) | (res[4] as u16);

        Ok(StLinkVersion {
            stlink_version,
            jtag_version,
            swim_version,
            pid,
            vid,
        })
    }

    pub fn get_mode(&self) -> StLinkResult<StLinkMode> {
        debug!("Reading current mode");
        let mut cmd = [0; 16];
        cmd[0] = CMD_GET_CURRENT_MODE;
        let res = self.xfer_cmd(&cmd, 2)?.expect("2 bytes");
        let mode = res[0];

        let mode = match mode {
            MODE_MASS => StLinkMode::Mass,
            MODE_DFU => StLinkMode::Dfu,
            MODE_DEBUG => StLinkMode::Debug,
            x => panic!("Unknown mode: {}", x),
        };

        Ok(mode)
    }

    /// Execute leave DFU mode command
    pub fn leave_dfu_mode(&self) -> StLinkResult<()> {
        debug!("Leaving dfu mode");
        let mut cmd = [0; 16];
        cmd[0] = CMD_DFU_COMMAND;
        cmd[1] = DFU_EXIT;
        self.send_cmd(&cmd)?;
        Ok(())
    }

    /// Enter debug mode!
    pub fn enter_debug_mode(&self) -> StLinkResult<()> {
        debug!("Enter swo mode");
        let mut cmd = [0; 16];
        cmd[0] = CMD_DEBUG_COMMAND;
        cmd[1] = DEBUG_ENTER;
        cmd[2] = DEBUG_ENTER_MODE_SWD;
        self.send_cmd(&cmd)?;
        Ok(())
    }

    /// Method 2 for reading data.
    pub fn read_debug32(&self, address: u32) -> StLinkResult<u32> {
        trace!("Reading 32 bits via debug32 from address 0x{:08x}", address);
        let mut cmd = [0; 16];
        cmd[0] = CMD_DEBUG_COMMAND;
        cmd[1] = DEBUG_JTAG_READDEBUG_32BIT;
        put_u32(&mut cmd, 2, address);

        let res = self.xfer_cmd(&cmd, 8)?.expect("eight bytes");

        // TODO: what do the first 4 bytes mean?
        let value = get_u32(&res, 4);
        trace!(
            "Read response: value at address 0x{:08x} is 0x{:08x}",
            address,
            value
        );

        Ok(value)
    }

    /// Write a value via debug32 command.
    pub fn write_debug32(&self, address: u32, value: u32) -> StLinkResult<()> {
        trace!(
            "Writing u32 value 0x{:08X} via debug32 to address 0x{:08X}",
            value,
            address
        );
        let mut cmd = [0; 16];
        cmd[0] = CMD_DEBUG_COMMAND;
        cmd[1] = DEBUG_JTAG_WRITEDEBUG_32BIT;
        put_u32(&mut cmd, 2, address);
        put_u32(&mut cmd, 6, value);

        let _res = self.xfer_cmd(&cmd, 2)?;
        // TODO: what does this response mean?

        Ok(())
    }

    /// Read memory 32.
    pub fn read_mem32(&self, address: u32, data_length: usize) -> StLinkResult<Vec<u8>> {
        trace!(
            "Reading {} bytes from address 0x{:08x}",
            data_length,
            address
        );
        let mut cmd = [0; 16];
        cmd[0] = CMD_DEBUG_COMMAND;
        cmd[1] = DEBUG_READ_U32;
        assert!(data_length > 0);
        assert!(data_length < 0x1_0000);
        put_u32(&mut cmd, 2, address);
        put_u16(&mut cmd, 6, data_length as u16);

        let memory_contents = self
            .xfer_cmd(&cmd, data_length)?
            .expect("At least some bytes");

        trace!(
            "Read response: memory at address 0x{:08x} is {:?}",
            address,
            memory_contents
        );

        Ok(memory_contents)
    }

    /// Method for reading a u32 value
    pub fn read_u32_via_mem32(&self, address: u32) -> StLinkResult<u32> {
        let res = self.read_mem32(address, 4)?;

        // process version bytes:
        let value = get_u32(&res, 0);
        trace!(
            "Read response: u32 value at address 0x{:08x} is 0x{:08x}",
            address,
            value
        );

        Ok(value)
    }

    fn send_cmd(&self, cmd: &[u8]) -> StLinkResult<()> {
        self.xfer_cmd(cmd, 0)?;
        Ok(())
    }

    fn xfer_cmd(&self, cmd: &[u8], rxsize: usize) -> StLinkResult<Option<Vec<u8>>> {
        assert!(cmd.len() == 16);
        trace!("Sending command: {:?}", cmd);
        let timeout = std::time::Duration::from_millis(700);
        let bytes_written = self.handle.write_bulk(2, cmd, timeout)?;
        if bytes_written != cmd.len() {
            return Err(StLinkError::Other(format!(
                "Mismatch in written bytes! (wrote {}, but wanted to write {})",
                bytes_written,
                cmd.len()
            )));
        }

        if rxsize > 0 {
            let mut response_buffer = vec![0; rxsize];
            let bytes_received = self.handle.read_bulk(0x81, &mut response_buffer, timeout)?;
            if bytes_received != rxsize {
                return Err(StLinkError::Other(format!(
                    "Mismatch in received bytes! (read {} bytes, but expected {} bytes)",
                    bytes_received, rxsize
                )));
            }
            trace!("Got response: {:?}", response_buffer);
            Ok(Some(response_buffer))
        } else {
            Ok(None)
        }
    }
}

fn put_u32(cmd: &mut [u8], offset: usize, value: u32) {
    // Ugh, this should be possible in a different manner.
    let mut cmd2 = vec![];
    cmd2.put_u32_le(value);
    cmd[offset] = cmd2[0];
    cmd[offset + 1] = cmd2[1];
    cmd[offset + 2] = cmd2[2];
    cmd[offset + 3] = cmd2[3];
}

fn put_u16(cmd: &mut [u8], offset: usize, value: u16) {
    // Ugh, this should be possible in a different manner.
    let mut cmd2 = vec![];
    cmd2.put_u16_le(value);
    cmd[offset] = cmd2[0];
    cmd[offset + 1] = cmd2[1];
}

fn get_u32(buffer: &[u8], offset: usize) -> u32 {
    let mut cursor = Cursor::new(buffer);
    cursor.set_position(offset as u64);
    cursor.get_u32_le()
}
