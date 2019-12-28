use super::Component;
use super::CoreSightResult;
use super::MemoryAccess;

pub const TPIU_PID: [u8; 8] = [0xA1, 0xB9, 0x0B, 0x0, 0x4, 0x0, 0x0, 0x0];

const REGISTER_OFFSET_TPIU_SSPSR: usize = 0x0;
const REGISTER_OFFSET_TPIU_CSPSR: usize = 0x4;
const REGISTER_OFFSET_TPIU_ACPR: usize = 0x10;
const REGISTER_OFFSET_TPIU_SPPR: usize = 0xF0;
const REGISTER_OFFSET_TPIU_FFCR: usize = 0x304;

/// TPIU unit
///
/// Trace port interface unit unit.
pub struct Tpiu<'m, M>
where
    M: MemoryAccess,
{
    component: Component<'m, M>,
}

impl<'m, M> Tpiu<'m, M>
where
    M: MemoryAccess,
{
    pub fn new(component: Component<'m, M>) -> Self {
        Tpiu { component }
    }

    pub fn set_port_size(&self, value: u32) -> CoreSightResult<()> {
        self.component
            .write_reg(REGISTER_OFFSET_TPIU_CSPSR, value)?;
        Ok(())
    }

    pub fn set_prescaler(&self, value: u32) -> CoreSightResult<()> {
        self.component.write_reg(REGISTER_OFFSET_TPIU_ACPR, value)?;
        Ok(())
    }

    /// Set protocol.
    /// 0 = sync trace mode
    /// 1 = async SWO (manchester)
    /// 2 = async SWO (NRZ)
    /// 3 = reserved
    pub fn set_pin_protocol(&self, value: u32) -> CoreSightResult<()> {
        self.component.write_reg(REGISTER_OFFSET_TPIU_SPPR, value)?;
        Ok(())
    }

    pub fn set_formatter(&self, value: u32) -> CoreSightResult<()> {
        self.component.write_reg(REGISTER_OFFSET_TPIU_FFCR, value)?;
        Ok(())
    }
}
