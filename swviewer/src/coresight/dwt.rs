//! Interface with the DWT (data watchpoint and trace) unit.
//!
//! This unit can monitor specific memory locations for write / read
//! access, this could be handy to debug a system :).
//!
//! See ARMv7-M architecture reference manual C1.8 for some additional
//! info about this stuff.

use super::CoreSightResult;
use super::MemoryAccess;

/// A struct representing a DWT unit on target.
pub struct Dwt<'m, M>
where
    M: MemoryAccess,
{
    /// The address where this debug block is located.
    address: u32,

    /// Memory access
    access: &'m M,
}

const REG_OFFSET_DWT_CTRL: usize = 0;

impl<'m, M> Dwt<'m, M>
where
    M: MemoryAccess,
{
    pub fn new(access: &'m M, address: u32) -> Self {
        Dwt { address, access }
    }

    pub fn info(&self) -> CoreSightResult<()> {
        let ctrl = self.read_reg(REG_OFFSET_DWT_CTRL)?;

        let num_comparators_available: u8 = ((ctrl >> 28) & 0xf) as u8;
        let has_trace_sampling_support = ctrl & (1 << 27) == 0;
        let has_compare_match_support = ctrl & (1 << 26) == 0;
        let has_cyccnt_support = ctrl & (1 << 25) == 0;
        let has_perf_counter_support = ctrl & (1 << 24) == 0;

        info!("DWT info:");
        info!(
            " number of comparators available: {}",
            num_comparators_available
        );
        info!(" trace sampling support: {}", has_trace_sampling_support);
        info!(" compare match support: {}", has_compare_match_support);
        info!(" cyccnt support: {}", has_cyccnt_support);
        info!(" performance counter support: {}", has_perf_counter_support);
        Ok(())
    }

    /// Enable data monitor on a given user variable at some address
    pub fn enable_trace(&self, var_address: u32) -> CoreSightResult<()> {
        let mask = 0; // size of the ignore mask, ignore nothing!
        let function = 3; // sample PC and data

        // entry 0:
        self.write_reg(0x20, var_address)?; // COMp value
        self.write_reg(0x24, mask)?; // mask
        self.write_reg(0x28, function)?; // function
        Ok(())
    }

    pub fn poll(&self) -> CoreSightResult<()> {
        let status = self.read_reg(0x28)?;
        let matched = status & (1 << 24) > 0;
        info!("DWT function0 State: matched={}", matched);
        Ok(())
    }

    fn read_reg(&self, offset: usize) -> CoreSightResult<u32> {
        let value = self.access.read_u32(self.address + offset as u32)?;
        Ok(value)
    }

    fn write_reg(&self, offset: usize, value: u32) -> CoreSightResult<()> {
        self.access.write_u32(self.address + offset as u32, value)?;
        Ok(())
    }
}
