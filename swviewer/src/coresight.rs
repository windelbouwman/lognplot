use super::memory::{MemoryAccess, MemoryAddress};

#[derive(Debug)]
pub struct ComponentIdentification {
    peripheral_id: Vec<u32>,
    component_id: Vec<u32>,
}

impl ComponentIdentification {
    /// Test if this component identification matches with the given
    /// component ID.
    pub fn is_component(&self, component_id: &[u32]) -> bool {
        self.component_id == component_id
    }
}

#[derive(Debug)]
pub enum CoreSightError {
    // Memory(M::MemoryError),
    Other(String),
}

impl From<String> for CoreSightError {
    fn from(e: String) -> Self {
        CoreSightError::Other(e)
    }
}

pub type CoreSightResult<T> = Result<T, CoreSightError>;

/// Read coresight block identification.
///
/// Given a coresight block, the identifation
/// in the shape of component and peripheral
/// id's are placed at the end of the block.
pub fn read_identification<M>(
    memory: &M,
    base: MemoryAddress,
) -> CoreSightResult<ComponentIdentification>
where
    M: MemoryAccess,
{
    let peripheral_id = {
        let peripheral_id_offsets: Vec<u32> = vec![
            0xFD0, // peripheral ID4
            0xFD4, // peripheral ID5
            0xFD8, // peripheral ID6
            0xFDC, // peripheral ID7
            0xFE0, // peripheral ID0
            0xFE4, // peripheral ID1
            0xFE8, // peripheral ID2
            0xFEC, // peripheral ID3
        ];

        let mut peripheral_id = vec![];
        for offset in peripheral_id_offsets {
            let value = memory.read_u32(base + offset)?;
            peripheral_id.push(value);
        }

        peripheral_id
    };

    let mut component_id = {
        let component_id_offsets: Vec<u32> = vec![
            0xFF0, // component ID0 (0xD)
            0xFF4, // component ID1 (0x10)
            0xFF8, // component ID2 (0x5)
            0xFFC, // component ID3 (0xB1)
        ];

        let mut component_id = vec![];
        for offset in component_id_offsets {
            let value = memory.read_u32(base + offset)?;
            component_id.push(value);
        }
        component_id
    };

    Ok(ComponentIdentification {
        peripheral_id,
        component_id,
    })
}

pub fn read_rom_table<M>(memory: &M,
    base: MemoryAddress) where
    M: MemoryAccess,
{
    // Read offsets until a zero is detected.
    let mut rom_table_offsets: Vec<u32> = vec![];
    let mut offset: u32 = 1;
    while offset > 0 {
        offset = 0;
    }
    
}

fn add_offset(base: MemoryAddress, offset: u32) -> MemoryAddress {
    base.wrapping_add(offset)
}

/// An Sbs component
struct SbsComponent<M>
where
    M: MemoryAccess,
{
    address: u32,
    accessor: M,
}
