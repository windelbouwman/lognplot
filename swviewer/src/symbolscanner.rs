//! Parse ELF file in search of debug symbols.
//!
//!

use crate::trace_var::TraceVar;

pub fn parse_elf_file(elf_filename: &str) -> gimli::Result<Vec<TraceVar>> {
    info!("Parsing {}", elf_filename);

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
                if let Some(var) = analyze_variable_entry(entry, &dwarf, header.encoding())? {
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

    let address: Option<u64> =
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

    let optional_var = if let (Some(name), Some(address)) = (name, address) {
        Some(TraceVar {
            name,
            address: address as u32,
        })
    } else {
        None
    };

    Ok(optional_var)
}
