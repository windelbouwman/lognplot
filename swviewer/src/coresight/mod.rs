mod component;
mod dwt;
mod error;
mod identification;
mod itm;
mod memory;
mod romtable;
mod target;
mod tpiu;
mod trace_protocol;

// Public API:
pub use memory::{MemoryAccess, MemoryAddress};
pub use target::Target;

use error::CoreSightError;
use identification::ComponentIdentification;

use component::Component;
use dwt::Dwt;
use dwt::DWT_PID;
use identification::read_identification;
use itm::Itm;
use itm::ITM_PID;
use romtable::read_rom_table;
use tpiu::{Tpiu, TPIU_PID};
pub use trace_protocol::Decoder;

pub type CoreSightResult<T> = Result<T, CoreSightError>;

fn add_offset(base: MemoryAddress, offset: u32) -> MemoryAddress {
    base.wrapping_add(offset)
}
