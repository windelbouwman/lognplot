use super::Dwt;
use super::Itm;
use super::MemoryAccess;

use super::read_identification;
use super::read_rom_table;
use super::CoreSightResult;

/// A struct representing a specific target device.
pub struct Target<'m, M>
where
    M: MemoryAccess,
{
    /// The target might have a DWT unit.
    dwt: Option<Dwt<'m, M>>,

    // etm:
    itm: Option<Itm<'m, M>>,

    tpiu: Option<i32>,

    /// Memory accessor
    access: &'m M,
}

impl<'m, M> Target<'m, M>
where
    M: MemoryAccess,
{
    pub fn new(access: &'m M) -> Self {
        Target {
            dwt: None,
            itm: None,
            tpiu: None,
            access,
        }
    }

    /// Scan rom table for debug components available.
    pub fn read_debug_components(&mut self) -> CoreSightResult<()> {
        let component_id_rom_table = vec![0xD, 0x10, 0x5, 0xb1];

        // Rom table base.
        let rom_table_base: u32 = 0xE00F_F000;

        // Trace enable, this is crucial for the detection of DWT from the ROM table, otherwise bogus values will be
        // read from PID and CID registers.
        self.trace_enable()?;

        let rom_table_id = read_identification(self.access, rom_table_base)?;

        info!("ROM table id: {:?}", rom_table_id);
        if rom_table_id.is_component(&component_id_rom_table) {
            info!("ROM TABLE DETECTED!");
            let rom_table = read_rom_table(self.access, rom_table_base)?;
            info!("Rom table: {:?}", rom_table);
            for entry in &rom_table.entries {
                info!("Entry: {:?}", entry);
            }

            let components = rom_table.read_components(self.access)?;
            // TODO: assumption about dwt location:
            let dwt = Dwt::new(self.access, 0xE000_1000);
            dwt.info()?;
            self.dwt = Some(dwt);

            println!("Components: {}", components.len());
            for component in components {
                println!(" - {:?}", component);
            }
        } else {
            println!("No ROM table found!");
        }

        Ok(())
    }

    fn trace_enable(&self) -> CoreSightResult<()> {
        // TODO?
        self.access.write_u32(0xE000_EDFC, 0x0100_0000)?;
        Ok(())
    }

    pub fn start_trace_memory_address(&self, addr: u32) -> CoreSightResult<()> {
        self.grab_dwt().enable_trace(addr)
    }

    pub fn poll(&self) -> CoreSightResult<()> {
        self.grab_dwt().poll()
    }

    fn grab_dwt(&self) -> &Dwt<M> {
        self.dwt
            .as_ref()
            .expect("DWT must be present before invoking this function.")
    }
}
