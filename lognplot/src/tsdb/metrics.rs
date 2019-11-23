/// Implement this for specific observations.
pub trait Metrics<V> {
    // TODO: we might merge update and include into a single function?
    fn update(&mut self, sample: &V);
    fn include(&mut self, metrics: &Self);
}
