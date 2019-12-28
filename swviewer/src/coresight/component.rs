// use error::CoreSightError;
use super::{ComponentIdentification, CoreSightResult, MemoryAccess, MemoryAddress};

pub struct Component<'m, M>
where
    M: MemoryAccess,
{
    /// Memory access
    access: &'m M,

    /// The address where this debug block is located.
    address: MemoryAddress,

    identification: ComponentIdentification,
}

impl<'m, M> std::fmt::Debug for Component<'m, M>
where
    M: MemoryAccess,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Component @ 0x{:08X} id={:?}",
            self.address, self.identification
        )
    }
}

impl<'m, M> Component<'m, M>
where
    M: MemoryAccess,
{
    pub fn new(
        access: &'m M,
        address: MemoryAddress,
        identification: ComponentIdentification,
    ) -> Self {
        Component {
            access,
            address,
            identification,
        }
    }

    pub fn address(&self) -> MemoryAddress {
        self.address
    }

    pub fn is_peripheral(&self, peripheral_id: &[u8]) -> bool {
        self.identification.is_peripheral(peripheral_id)
    }

    pub fn read_reg(&self, offset: usize) -> CoreSightResult<u32> {
        let value = self.access.read_u32(self.address + offset as u32)?;
        Ok(value)
    }

    pub fn write_reg(&self, offset: usize, value: u32) -> CoreSightResult<()> {
        self.access.write_u32(self.address + offset as u32, value)?;
        Ok(())
    }
}
