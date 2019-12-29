use super::Dwt;
use super::MemoryAccess;
use super::{Itm, Tpiu};
use super::{DWT_PID, ITM_PID, TPIU_PID};

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

    tpiu: Option<Tpiu<'m, M>>,

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

            let components = rom_table.read_components(self.access)?;

            info!("Components: {}", components.len());
            for component in components {
                if component.is_peripheral(&DWT_PID) {
                    info!(" - Detected DWT component! {:?}", component);
                    // TODO: assumption about dwt location:
                    // self.access, 0xE000_1000
                    let dwt_expected_address = 0xE000_1000;
                    if component.address() != dwt_expected_address {
                        warn!(
                            "DWT located at 0x{:08X} (usual address is 0x{:08X}",
                            component.address(),
                            dwt_expected_address
                        )
                    }

                    if self.dwt.is_none() {
                        let dwt = Dwt::new(component);
                        dwt.info()?;
                        self.dwt = Some(dwt);
                    }
                } else if component.is_peripheral(&ITM_PID) {
                    info!(" - Detected ITM component! {:?}", component);

                    if self.itm.is_none() {
                        let itm = Itm::new(component);
                        self.itm = Some(itm);
                    }
                } else if component.is_peripheral(&TPIU_PID) {
                    info!(" - Detected TPIU component! {:?}", component);
                    let tpiu_expected_address = 0xE004_0000;
                    if component.address() != tpiu_expected_address {
                        warn!(
                            "TPIU located at 0x{:08X} (usual address is 0x{:08X}",
                            component.address(),
                            tpiu_expected_address
                        )
                    }

                    if self.tpiu.is_none() {
                        let tpiu = Tpiu::new(component);
                        self.tpiu = Some(tpiu);
                    }
                } else {
                    info!(" - Unknown {:?}", component);
                }
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
        // stm32 specific reg (DBGMCU_CR):
        self.access.write_u32(0xE004_2004, 0x27);

        // Config tpiu:
        let tpiu = self.grab_tpiu();
        tpiu.set_port_size(1)?;
        let uc_freq = 16; // MHz (HSI frequency)
        let swo_freq = 2; // MHz
        let prescaler = (uc_freq / swo_freq) - 1;
        tpiu.set_prescaler(prescaler)?;
        tpiu.set_pin_protocol(2)?;
        tpiu.set_formatter(0x100)?;

        // Config itm:
        let itm = self.grab_itm();
        itm.unlock()?;
        itm.tx_enable()?;

        // config dwt:
        let dwt = self.grab_dwt();
        // Future:
        dwt.enable_trace(addr)?;
        // dwt.disable_memory_watch()?;

        Ok(())
    }

    pub fn poll(&self) -> CoreSightResult<()> {
        self.grab_dwt().poll()
    }

    fn grab_dwt(&self) -> &Dwt<M> {
        self.dwt.as_ref().expect("DWT must be present.")
    }

    fn grab_itm(&self) -> &Itm<M> {
        self.itm.as_ref().expect("ITM must be present.")
    }

    fn grab_tpiu(&self) -> &Tpiu<M> {
        self.tpiu.as_ref().expect("TPIU must be present.")
    }
}
