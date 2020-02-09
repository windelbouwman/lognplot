//! Module for using the ITM.
//!
//! ITM = Instrumentation Trace Macrocell

use super::Component;
use super::CoreSightResult;
use super::MemoryAccess;

pub const ITM_PID: [u8; 8] = [0x1, 0xB0, 0x3b, 0x0, 0x4, 0x0, 0x0, 0x0];

pub struct Itm<'m, M>
where
    M: MemoryAccess,
{
    component: Component<'m, M>,
}

const REGISTER_OFFSET_ITM_TER: usize = 0xE00;
const REGISTER_OFFSET_ITM_TPR: usize = 0xE40;
const REGISTER_OFFSET_ITM_TCR: usize = 0xE80;
const REGISTER_OFFSET_ACCESS: usize = 0xFB0;

impl<'m, M> Itm<'m, M>
where
    M: MemoryAccess,
{
    pub fn new(component: Component<'m, M>) -> Self {
        Itm { component }
    }

    pub fn unlock(&self) -> CoreSightResult<()> {
        self.component
            .write_reg(REGISTER_OFFSET_ACCESS, 0xC5AC_CE55)?;

        Ok(())
    }

    pub fn tx_enable(&self) -> CoreSightResult<()> {
        let mut value = self.component.read_reg(REGISTER_OFFSET_ITM_TCR)?;
        info!("ITM_TCR Before: 0x{:08X}", value);

        value |= 1; // itm enable
        value |= 1 << 1; // timestamp enable
        value |= 1 << 2; // Enable sync pulses, note DWT_CTRL.SYNCTAP must be configured.
        value |= 1 << 3; // tx enable (for DWT)
        value |= 13 << 16; // 7 bits trace bus ID
        self.component.write_reg(REGISTER_OFFSET_ITM_TCR, value)?;

        let value = self.component.read_reg(REGISTER_OFFSET_ITM_TCR)?;
        info!("ITM_TCR After: 0x{:08X}", value);

        // Enable 32 channels:
        self.component
            .write_reg(REGISTER_OFFSET_ITM_TER, 0xFFFF_FFFF)?;

        Ok(())
    }
}
