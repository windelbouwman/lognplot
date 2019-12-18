#[macro_use]
extern crate log;

mod coresight;
mod memory;
mod stlink;

use coresight::CoreSightResult;
use memory::{MemoryAccess, MemoryAddress};

use stlink::{StLink, StLinkError, StLinkMode, StLinkResult};

fn main() {
    simple_logger::init().unwrap();
    println!("Hello, world! {:?}", rusb::version());

    if let Err(e) = do_magic() {
        println!("An error occurred: {:?}", e);
    }
}

fn do_magic() -> StLinkResult<()> {
    for device_list in rusb::devices().iter() {
        println!("Device list:");
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
        println!("ST link found!");
        let x = stlink::open_st_link(st_link_device)?;
        interact(x)?;
    } else {
        println!("No ST link found, please connect it?");
    }

    Ok(())
}

fn enter_proper_mode(st_link: &StLink) -> StLinkResult<()> {
    let mut mode = st_link.get_mode()?;
    println!("Mode: {:?}", mode);
    match mode {
        StLinkMode::Dfu => {
            st_link.leave_dfu_mode()?;
            mode = st_link.get_mode()?;
            println!("Mode: {:?}", mode);
        }
        _ => {}
    }

    match mode {
        StLinkMode::Dfu | StLinkMode::Mass => {
            st_link.enter_debug_mode()?;
            mode = st_link.get_mode()?;
            println!("Mode: {:?}", mode);
        }
        _ => {}
    }

    Ok(())
}

fn interact(st_link: StLink) -> StLinkResult<()> {
    let version = st_link.get_version()?;
    println!("Version: {:?}", version);

    enter_proper_mode(&st_link)?;

    let address = 0xE0042000; // Chip ID
    let value = st_link.read_debug32(address)?;
    println!("Chip ID is 0x{:08X}", value);

    let address = 0xE00FF000; // Coresight thingy
    let value = st_link.read_debug32(address)?;
    println!("Value at address {} is {}", address, value);

    let component_id_rom_table = vec![0xD, 0x10, 0x5, 0xb1];

    // Rom table base.
    let rom_table_base: u32 = 0xE00F_F000;

    let rom_table_id = coresight::read_identification(&st_link, rom_table_base).unwrap();

    println!("ROM table id: {:?}", rom_table_id);
    if rom_table_id.is_component(&component_id_rom_table) {
        println!("ROM TABLE DETECTED!");
        coresight::read_rom_table(&st_link, rom_table_base);
    } else {
        println!("No ROM table found!");
    }

    Ok(())
}

fn old_detect(st_link: StLink) -> StLinkResult<()> {
    let rom_table_base: u32 = 0xE00F_F000;

    // ROM table base address: 0xE00F_F000
    // Identify Cortex-M4 ROM table:
    let addresses: Vec<u32> = vec![
        0xFD0, // peripheral ID4
        0xFE0, // peripheral ID0
        0xFE4, // peripheral ID1
        0xFE8, // peripheral ID2
        0xFEC, // peripheral ID3
        0xFF0, // component ID0 (0xD)
        0xFF4, // component ID1 (0x10)
        0xFF8, // component ID2 (0x5)
        0xFFC, // component ID3 (0xB1)
    ];
    println!("Identification table");
    for address in addresses {
        let value = st_link.read_debug32(rom_table_base + address)?;
        println!("Reg: 0x{:08X} = 0x{:08X}", address, value);
    }

    // TODO: identify properly!

    // ROM table components:
    let addresses: Vec<u32> = vec![
        0x000, // SCS
        0x004, // DWT
        0x008, // FPB
        0x00C, // ITM
        0x010, // TPIU
        0x014, // ETM
        0x018, // End marker (0)
    ];

    println!("Identification table");
    for address in addresses {
        let value = st_link.read_debug32(rom_table_base + address)?;
        println!("Reg: 0x{:08X} = 0x{:08X}", address, value);
        let component_address: u64 = rom_table_base as u64 + value as u64;
        println!("Component address: 0x{:08X}", component_address);
    }

    Ok(())
}

impl MemoryAccess for StLink {
    fn read_u32(&self, address: MemoryAddress) -> Result<u32, String> {
        self.read_debug32(address)
            .map_err(|e| format!("st-link error: {:?}", e))
    }
}
