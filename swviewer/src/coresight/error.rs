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
