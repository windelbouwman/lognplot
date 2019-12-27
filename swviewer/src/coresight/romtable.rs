use super::super::memory::{MemoryAccess, MemoryAddress};
use super::identification::read_identification;
use super::CoreSightResult;
use super::{add_offset, Component};

pub struct RomTable {
    base: MemoryAddress,
    pub entries: Vec<RomTableEntry>,
}

impl std::fmt::Debug for RomTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Rom table @ 0x{:08X} with {} entries",
            self.base,
            self.entries.len()
        )
    }
}

pub struct RomTableEntry {
    /// Two complement offset of rom table base.
    offset: u32,

    /// Indicates if this entry is present or not.
    present: bool,
}

impl std::fmt::Debug for RomTableEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let present_text = if self.present { "present" } else { "missing" };
        write!(f, "Entry {} @ offset 0x{:08X}", present_text, self.offset)
    }
}

impl RomTable {
    pub fn read_components<M>(&self, memory: &M) -> CoreSightResult<Vec<Component>>
    where
        M: MemoryAccess,
    {
        let mut components = vec![];
        for entry in &self.entries {
            if entry.present {
                let address: MemoryAddress = add_offset(self.base, entry.offset);

                let identifation = read_identification(memory, address)?;

                let component = Component {
                    address,
                    identifation,
                };

                info!("Component! offset=0x{:08X} {:?}", entry.offset, component);

                components.push(component);
            } else {
                info!("Component not present!")
            }
        }

        Ok(components)
    }
}

pub fn read_rom_table<M>(memory: &M, base: MemoryAddress) -> CoreSightResult<RomTable>
where
    M: MemoryAccess,
{
    // Read offsets until a zero is detected.
    let mut rom_table_entries: Vec<RomTableEntry> = vec![];
    let mut offset = 0;
    let mut entry_value: u32 = memory.read_u32(base + offset)?;
    while entry_value > 0 {
        // Extract bit fields:
        let entry_offset = entry_value & 0xFFFF_F000;
        let entry_present: bool = (entry_value & 1) == 1;

        let table_entry = RomTableEntry {
            offset: entry_offset,
            present: entry_present,
        };
        rom_table_entries.push(table_entry);

        // Read next record:
        offset += 4;
        entry_value = memory.read_u32(base + offset)?;
    }

    Ok(RomTable {
        base,
        entries: rom_table_entries,
    })
}
