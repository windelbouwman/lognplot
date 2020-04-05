//! Parse ELF file in search of debug symbols.
//!
//!

use crate::trace_var::{TraceVar, VarType};
use std::path::PathBuf;

pub fn parse_elf_file(elf_filename: &PathBuf) -> gimli::Result<Vec<TraceVar>> {
    info!("Parsing {}", elf_filename.display());

    use object::Object;
    let data = std::fs::read(elf_filename)?;
    let obj = object::File::parse(&data).unwrap();

    // Copied from: https://github.com/gimli-rs/gimli/blob/master/examples/simple.rs
    info!(
        "Parsed obj file. Has debug syms: {:?}",
        obj.has_debug_symbols()
    );

    let endian = if obj.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };

    // Define some helper closures:
    let load_section = |id: gimli::SectionId| -> Result<std::borrow::Cow<[u8]>, gimli::Error> {
        Ok(obj
            .section_data_by_name(id.name())
            .unwrap_or(std::borrow::Cow::Borrowed(&[][..])))
    };

    let load_section_sup = |_| Ok(std::borrow::Cow::Borrowed(&[][..]));

    let dwarf_cow = gimli::Dwarf::load(&load_section, &load_section_sup)?;

    let borrow_section: &dyn for<'a> Fn(
        &'a std::borrow::Cow<[u8]>,
    ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(&*section, endian);

    // Create `EndianSlice`s for all of the sections.
    let dwarf = dwarf_cow.borrow(&borrow_section);

    let mut trace_vars = vec![];
    let mut iter = dwarf.units();
    while let Some(header) = iter.next()? {
        let unit = dwarf.unit(header)?;
        // if let Some(name) = unit.name {
        //     println!("Unit: {:?}", name.to_string());
        // }

        let mut entries = unit.entries();
        // entries.next_dfs().unwrap();
        while let Some((_depth, entry)) = entries.next_dfs()? {
            // while let Some(entry) = entries.next_sibling().unwrap() {
            let tag = entry.tag();
            // println!("  - entry: depth={}, tag={:?}", depth, entry.tag());
            if tag == gimli::DW_TAG_variable {
                // println!("   -- it is a variable!");
                if let Some(var) = analyze_variable_entry(&unit, entry, &dwarf, header.encoding())?
                {
                    // println!("Var: {} @ 0x{:08X}", var.name, var.address);
                    trace_vars.push(var);
                }
            }
        }
    }

    // Sort variables by name:
    trace_vars.sort_by_key(|v| v.name.clone());

    println!("Detected {} variables", trace_vars.len());
    for var in &trace_vars {
        println!("-> Var: {} @ 0x{:08X}", var.name, var.address);
    }

    Ok(trace_vars)
}

fn analyze_variable_entry<E>(
    unit: &gimli::Unit<gimli::EndianSlice<E>>,
    entry: &gimli::DebuggingInformationEntry<gimli::EndianSlice<E>>,
    dwarf: &gimli::Dwarf<gimli::EndianSlice<E>>,
    encoding: gimli::Encoding,
) -> gimli::Result<Option<TraceVar>>
where
    E: gimli::Endianity,
{
    // let mut attrs = entry.attrs();
    // while let Some(attr) = attrs.next()? {
    //     println!("    --- attr name: {:?}", attr.name());
    //     println!("    --- attr value: {:?}", attr.value());
    // }

    let name: Option<String> =
        if let Some(value) = entry.attr_value(gimli::constants::DW_AT_name)? {
            match value {
                gimli::AttributeValue::String(x) => {
                    let name: String = x.to_string().unwrap().to_owned();
                    Some(name)
                }
                gimli::AttributeValue::DebugStrRef(str_ref) => {
                    if let Ok(s) = dwarf.debug_str.get_str(str_ref) {
                        let name: String = s.to_string().unwrap().to_owned();
                        Some(name)
                    } else {
                        // str_ref
                        None
                    }
                }
                x => {
                    // "??".to_owned()
                    println!("Name: {:?}", x);
                    None
                }
            }

        // println!("Name = {}", name);
        } else {
            None
        };

    let typ: Option<VarType> = extract_type(unit, entry)?;
    let address: Option<u64> = extract_address(encoding, entry)?;

    let optional_var = if let (Some(name), Some(address), Some(typ)) = (name, address, typ) {
        Some(TraceVar {
            name,
            address: address as u32,
            typ,
        })
    } else {
        None
    };

    Ok(optional_var)
}

/// Take a debug entry and examine it's DW_AT_type attribute closely
fn extract_type<E>(
    unit: &gimli::Unit<gimli::EndianSlice<E>>,
    entry: &gimli::DebuggingInformationEntry<gimli::EndianSlice<E>>,
) -> gimli::Result<Option<VarType>>
where
    E: gimli::Endianity,
{
    let optional_typ = if let Some(value) = entry.attr_value(gimli::constants::DW_AT_type)? {
        match value {
            gimli::AttributeValue::UnitRef(offset) => {
                let mut type_die_cursor = unit.entries_at_offset(offset)?;
                type_die_cursor.next_entry()?;
                if let Some(type_die) = type_die_cursor.current() {
                    // println!("type die: {:?}", type_die);
                    match type_die.tag() {
                        gimli::DW_TAG_volatile_type => {
                            // println!("Volatile!");
                            extract_type(unit, type_die)?
                        }
                        gimli::DW_TAG_base_type => {
                            let byte_size =
                                type_die.attr_value(gimli::constants::DW_AT_byte_size)?;
                            let encoding = type_die.attr_value(gimli::constants::DW_AT_encoding)?;
                            // println!(
                            //     "Base type! byte_size={:?}, encoding={:?}",
                            //     byte_size, encoding
                            // );
                            if let (
                                Some(gimli::AttributeValue::Encoding(ate)),
                                Some(gimli::AttributeValue::Udata(byte_size)),
                            ) = (encoding, byte_size)
                            {
                                match ate {
                                    gimli::constants::DW_ATE_float => match byte_size {
                                        4 => Some(VarType::Float32),
                                        8 => None,
                                        x => {
                                            println!("Invalid float size.. {}", x);
                                            None
                                        }
                                    },
                                    gimli::constants::DW_ATE_signed => match byte_size {
                                        0 => None,
                                        1 => Some(VarType::Int8),
                                        2 => Some(VarType::Int16),
                                        4 => Some(VarType::Int32),
                                        8 => None,
                                        x => {
                                            println!("Invalid signed size.. {}", x);
                                            None
                                        }
                                    },
                                    gimli::constants::DW_ATE_unsigned => match byte_size {
                                        0 => None,
                                        1 => Some(VarType::Uint8),
                                        2 => Some(VarType::Uint16),
                                        4 => Some(VarType::Uint32),
                                        8 => None,
                                        x => {
                                            println!("Invalid unsigned size.. {}", x);
                                            None
                                        }
                                    },
                                    gimli::constants::DW_ATE_unsigned_char => match byte_size {
                                        1 => Some(VarType::Int8),
                                        _x => None,
                                    },
                                    gimli::constants::DW_ATE_boolean => {
                                        // Hmmz, what is a boolean?
                                        None
                                    }
                                    x => {
                                        println!("?????????? {}", x);
                                        None
                                    }
                                }
                            } else {
                                None
                            }
                        }
                        gimli::DW_TAG_pointer_type => {
                            // Hmmz, pointer, what to do?
                            None
                        }
                        gimli::DW_TAG_structure_type => {
                            // Structure type, cool stuff!
                            // TODO: add .field to the name, and recurse
                            None
                        }
                        gimli::DW_TAG_array_type => {
                            // TODO: handle this
                            None
                        }
                        gimli::DW_TAG_enumeration_type => {
                            // TODO: can we handle this?
                            None
                        }
                        gimli::DW_TAG_union_type => {
                            // TODO: handle this ?
                            None
                        }
                        x => {
                            println!("Other type of type: {:?}", x);
                            None
                        }
                    }
                } else {
                    None
                }
            }
            x => {
                println!("Other value: {:?}", x);
                None
            }
        }
    } else {
        None
    };

    Ok(optional_typ)
}

/// Try to determine the absolute address by inspecting the DW_AT_location attribute.
fn extract_address<E>(
    encoding: gimli::Encoding,
    entry: &gimli::DebuggingInformationEntry<gimli::EndianSlice<E>>,
) -> gimli::Result<Option<u64>>
where
    E: gimli::Endianity,
{
    let optional_address =
        if let Some(value) = entry.attr_value(gimli::constants::DW_AT_location)? {
            if let Some(loc) = value.exprloc_value() {
                let mut eval = loc.evaluation(encoding);
                let mut res = eval.evaluate()?;
                while res != gimli::EvaluationResult::Complete {
                    match res {
                        gimli::EvaluationResult::RequiresRelocatedAddress(addr) => {
                            // TODO: relocate address?
                            res = eval.resume_with_relocated_address(addr)?;
                        }
                        _ => {
                            break;
                        }
                    }
                }

                match res {
                    gimli::EvaluationResult::Complete => {
                        let x = eval.result();
                        // println!("Location = {:?}", x);
                        if x.len() == 1 {
                            let piece = x.first().unwrap();
                            match piece.location {
                                gimli::Location::Address { address } => Some(address),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => {
                        // println!("Location = {:?}", res);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

    Ok(optional_address)
}
