use super::memory::{MemoryAccess, MemoryAddress};

mod dwt;
mod error;
mod identification;
mod itm;
mod romtable;
mod target;

use error::CoreSightError;
use identification::ComponentIdentification;

pub use dwt::Dwt;
pub use identification::read_identification;
pub use itm::Itm;
pub use romtable::read_rom_table;
pub use target::Target;

pub type CoreSightResult<T> = Result<T, CoreSightError>;

pub struct Component {
    address: MemoryAddress,
    identifation: ComponentIdentification,
}

impl std::fmt::Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Component @ 0x{:08X} id={:?}",
            self.address, self.identifation
        )
    }
}

fn add_offset(base: MemoryAddress, offset: u32) -> MemoryAddress {
    base.wrapping_add(offset)
}

// An Sbs component
/*
struct SbsComponent<M>
where
    M: MemoryAccess,
{
    address: u32,
    accessor: M,
}
*/
