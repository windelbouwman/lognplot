use super::Text;

/// Implement this for specific observations.
pub trait Metrics<V> {
    // TODO: we might merge update and include into a single function?
    fn update(&mut self, sample: &V);

    /// Include other metrics into this metrics.
    fn include(&mut self, metrics: &Self);
}

/// The most simple metric which works always: just count the observations.
#[derive(Clone, Debug)]
pub struct CountMetrics {
    pub count: usize,
}

impl<V> Metrics<V> for CountMetrics {
    /// Integrate a single sample into tha metrics.
    fn update(&mut self, _sample: &V) {
        self.count += 1;
    }

    fn include(&mut self, metrics: &CountMetrics) {
        self.count += metrics.count;
    }
}

// This is a bit lame, but impl<V> From<V> for CountMetrics conflicts with a builtin From implementation.
impl From<Text> for CountMetrics {
    fn from(_observation: Text) -> Self {
        CountMetrics { count: 1 }
    }
}
