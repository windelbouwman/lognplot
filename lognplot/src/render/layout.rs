use super::ChartOptions;
use crate::geometry::Size;

/// Chart layout in pixels.
///
/// This struct has the various elements where parts of the chart are located.
pub struct ChartLayout {
    pub width: f64,
    pub height: f64,
    pub plot_top: f64,
    pub plot_left: f64,
    pub plot_bottom: f64,
    pub plot_right: f64,
    pub plot_width: f64,
    pub plot_height: f64,
}

impl ChartLayout {
    pub fn new(size: Size) -> Self {
        ChartLayout {
            // TODO: casowary?
            width: size.width,
            height: size.height,
            plot_top: 0.0,
            plot_left: 0.0,
            plot_bottom: 0.0,
            plot_right: 0.0,
            plot_width: 0.0,
            plot_height: 0.0,
        }
    }

    pub fn layout(&mut self, options: &ChartOptions) {
        self.plot_top = options.padding;
        self.plot_left = 140.0;
        self.plot_bottom = self.height - 60.0;
        self.plot_right = self.width - options.padding;
        self.plot_height = self.plot_bottom - self.plot_top;
        self.plot_width = self.plot_right - self.plot_left;
    }
}
