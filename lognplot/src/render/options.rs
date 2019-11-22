pub struct ChartOptions {
    pub tick_size: f64,
    pub padding: f64,
}

impl Default for ChartOptions {
    fn default() -> Self {
        ChartOptions {
            tick_size: 7.0,
            padding: 10.0,
        }
    }
}
