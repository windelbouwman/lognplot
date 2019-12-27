use super::CoreSightResult;
use super::MemoryAccess;

pub struct Itm<'m, M>
where
    M: MemoryAccess,
{
    /// The address where this debug block is located.
    address: u32,

    /// Memory access
    access: &'m M,
}

impl<'m, M> Itm<'m, M>
where
    M: MemoryAccess,
{
    pub fn new(access: &'m M, address: u32) -> Self {
        Itm { address, access }
    }

    pub fn tx_enable(&self) {}
}
