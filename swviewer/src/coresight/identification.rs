//! ROM table identification.
//!
//! Each ROM table element is identified by peripheral and component id's.
//! The component ID consists of CID0 up to CID3, the peripheral ID
//! consists of PID0 up to PID7.
//!

use super::{CoreSightResult, MemoryAccess, MemoryAddress};

pub struct ComponentIdentification {
    peripheral_id: Vec<u8>,
    component_id: Vec<u8>,
    component_class: Option<u8>,
}

impl std::fmt::Debug for ComponentIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let component_text = if let Some(component_class) = self.component_class {
            format!("{}", component_class)
        } else {
            format!("{:?}", self.component_id)
        };

        write!(
            f,
            "Id(component={}, pid={:?})",
            component_text, self.peripheral_id
        )
    }
}

impl ComponentIdentification {
    /// Check if this component identification makes sense.
    pub fn is_valid(&self) -> bool {
        self.component_class.is_some()
    }

    /// Test if this component identification matches with the given
    /// component ID.
    pub fn is_component(&self, component_id: &[u8]) -> bool {
        self.component_id == component_id
    }

    pub fn is_peripheral(&self, peripheral_id: &[u8]) -> bool {
        self.peripheral_id == peripheral_id
    }
}

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
    let component_id = read_component_id(memory, base)?;
    let component_class = parse_component_id(&component_id);

    let peripheral_id = read_peripheral_id(memory, base)?;
    parse_pid(&peripheral_id);

    Ok(ComponentIdentification {
        peripheral_id,
        component_id,
        component_class,
    })
}

fn parse_component_id(component_id: &[u8]) -> Option<u8> {
    // Check component pre-amble:
    if component_id[0] == 0xD && component_id[2] == 0x5 && component_id[3] == 0xB1 {
        info!("Pre-amble is OK.");
        let component_class: u8 = component_id[1] >> 4;
        info!("Component class: {}", component_class);
        Some(component_class)
    } else {
        // Pre-amble is bad.
        warn!("Component pre-amble is wrong.");
        None
    }
}

fn parse_pid(pid: &[u8]) {
    let part_number: u16 = ((pid[1] as u16 & 0xF) << 8) | (pid[0] as u16);
    let jep106_id_code: u8 = ((pid[2] & 0x7) << 4) | ((pid[1] & 0xF0) >> 4);
    info!(
        "part number: {}, jep106_id_code: {}",
        part_number, jep106_id_code
    );
}

// Component classes (table 9-3 in ARM debug interface architecture specification ADIv5.0)
// 1 = rom table
// 9 = debug component
// 14 = generic IP component

fn read_component_id<M>(memory: &M, base: MemoryAddress) -> CoreSightResult<Vec<u8>>
where
    M: MemoryAccess,
{
    // only the lowest byte of each id is in use.
    // Component ID 0: preamble 0 -> fixed to 0xD (13)
    // Component ID 1: bits 4..7 are the class, bits 0..3 are zero
    // Component ID 2: preamble 2 -> fixed to 0x5 (5)
    // Component ID 3: preamble 3 -> fixed to 0xB1 (177)
    let component_id_offsets: Vec<u32> = vec![
        0xFF0, // component ID0 (0xD)
        0xFF4, // component ID1 (0x10)
        0xFF8, // component ID2 (0x5)
        0xFFC, // component ID3 (0xB1)
    ];

    let mut component_id = vec![];
    for offset in component_id_offsets {
        let value = memory.read_u32(base + offset)?;
        component_id.push((value & 0xff) as u8);
    }

    Ok(component_id)
}

fn read_peripheral_id<M>(memory: &M, base: MemoryAddress) -> CoreSightResult<Vec<u8>>
where
    M: MemoryAccess,
{
    let peripheral_id_offsets: Vec<u32> = vec![
        0xFE0, // peripheral ID0
        0xFE4, // peripheral ID1
        0xFE8, // peripheral ID2
        0xFEC, // peripheral ID3
        0xFD0, // peripheral ID4
        0xFD4, // peripheral ID5
        0xFD8, // peripheral ID6
        0xFDC, // peripheral ID7
    ];

    let mut peripheral_id = vec![];
    for offset in peripheral_id_offsets {
        let value = memory.read_u32(base + offset)?;
        peripheral_id.push((value & 0xFF) as u8);
    }

    Ok(peripheral_id)
}
