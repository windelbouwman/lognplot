use super::Observation;

/// Less detailed summary, but easier to keep track of.
#[derive(Debug, Clone)]
pub struct QuickSummary<V> {
    pub count: usize,
    pub last: Observation<V>,
}

impl<V> QuickSummary<V> {
    pub fn new(count: usize, last: Observation<V>) -> Self {
        QuickSummary { count, last }
    }
}
