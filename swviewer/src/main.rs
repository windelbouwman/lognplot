#[macro_use]
extern crate log;

mod coresight;
mod stlink;

use coresight::{MemoryAccess, MemoryAddress, Target};
use stlink::{StLink, StLinkMode, StLinkResult};

fn main() {
    // simple_logger::init().unwrap();
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    info!("Hello, world! {:?}", rusb::version());

    if let Err(e) = do_magic() {
        error!("An error occurred: {:?}", e);
    }
}

fn do_magic() -> StLinkResult<()> {
    for device_list in rusb::devices().iter() {
        info!("Device list:");
        for device in device_list.iter() {
            let desc = device.device_descriptor()?;
            println!(
                "- Device: bus={:?} vendor:product = {:04X}:{:04X}",
                device.bus_number(),
                desc.vendor_id(),
                desc.product_id()
            );
        }
    }

    if let Some(st_link_device) = stlink::find_st_link()? {
        info!("ST link found!");
        let sl = stlink::open_st_link(st_link_device)?;
        sl.cmd_x40();
        interact(&sl)?;

        if let Err(e) = interact2(&sl) {
            error!("Error: {:?}", e);
        }

        // Enter trace capture:
        loop {
            std::thread::sleep_ms(60);

            let trace_byte_count = sl.get_trace_count()?;
            info!("Trace bytes: {}", trace_byte_count);
            if trace_byte_count > 0 {
                info!("Reading trace data.");
                let trace_data = sl.read_trace_data(trace_byte_count)?;
                println!("Trace data: {:?}", trace_data);
            }
        }
    } else {
        warn!("No ST link found, please connect it?");
    }

    Ok(())
}

fn enter_proper_mode(st_link: &StLink) -> StLinkResult<()> {
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

fn read_chip_id(st_link: &StLink) -> StLinkResult<()> {
    let address = 0xE004_2000; // Chip ID
    let value = st_link.read_debug32(address)?;
    info!("Chip ID is 0x{:08X}", value);
    Ok(())
}

fn interact(st_link: &StLink) -> StLinkResult<()> {
    let version = st_link.get_version()?;
    info!("ST-link Version: {:?}", version);

    enter_proper_mode(st_link)?;
    read_chip_id(st_link)?;

    Ok(())
}

fn interact2<M>(mem_access: &M) -> coresight::CoreSightResult<()>
where
    M: MemoryAccess,
{
    let mut target = Target::new(mem_access);

    target.read_debug_components()?;
    target.start_trace_memory_address(0x2000_0000)?;
    for _a in 1..10 {
        target.poll()?;
    }

    Ok(())
}

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
