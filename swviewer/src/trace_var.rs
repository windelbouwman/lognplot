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
    Int32,

    Float32,

    Int8,
}

impl fmt::Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarType::Float32 => write!(f, "float"),
            VarType::Int32 => write!(f, "int32_t"),
            VarType::Int8 => write!(f, "int8_t"),
        }
    }
}
