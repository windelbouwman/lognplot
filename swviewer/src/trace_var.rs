/// A potential variable to trace.
///
/// This has a name, a memory address and all the other stuff required :)
#[derive(Clone, Debug)]
pub struct TraceVar {
    /// The name of the variable.
    pub name: String,

    /// 32 bits address (since we deal with arm 32 bits)
    pub address: u32,
    // type?
}
