use std::fmt;

/// A potential variable to trace.
///
/// This has a name, a memory address and all the other stuff required :)
#[derive(Clone, Debug)]
pub struct TraceVar {
    /// The name of the variable.
    pub name: String,

    /// 32 bits address (since we deal with arm 32 bits)
    pub address: u32,

    pub typ: VarType,
}

#[derive(Clone, Debug)]
pub enum VarType {
    Uint8,
    Uint16,
    Uint32,

    Int8,
    Int16,
    Int32,

    Float32,
}

impl fmt::Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarType::Float32 => write!(f, "float"),
            VarType::Int32 => write!(f, "int32_t"),
            VarType::Int16 => write!(f, "int16_t"),
            VarType::Int8 => write!(f, "int8_t"),
            VarType::Uint32 => write!(f, "uint32_t"),
            VarType::Uint16 => write!(f, "uint16_t"),
            VarType::Uint8 => write!(f, "uint8_t"),
        }
    }
}
