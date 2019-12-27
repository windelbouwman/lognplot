pub type MemoryAddress = u32;

/// Implement this trait to provide access to a memory map.
///
/// This trait will for example be implemented by the st-link-v2 device.
pub trait MemoryAccess {
    fn read_u32(&self, address: MemoryAddress) -> Result<u32, String>;
    fn write_u32(&self, address: MemoryAddress, value: u32) -> Result<(), String>;
}
