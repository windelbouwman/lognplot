/// Axis options
#[derive(Clone)]
pub struct AxisOptions {
    /// Draw major tick markers
    pub major_ticks: bool,

    /// Draw minor tick markers
    pub minor_ticks: bool,
}

/// Implement sensible default axis options.
impl Default for AxisOptions {
    fn default() -> Self {
        AxisOptions {
            major_ticks: true,
            minor_ticks: false,
        }
    }
}
