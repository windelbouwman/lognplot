use quartzgui::widgets::{Element, Widget};
use quartzplot::Chart;

/// A graph visualization control!
pub struct GraphControl {
    chart: Chart,
    element: Element,
}

impl GraphControl {
    pub fn new(chart: Chart) -> Self {
        let element = Element::new();
        Self { chart, element }
    }
}

impl Widget for GraphControl {
    fn layout(&self) {}

    fn draw(&self) {
        // for trace in self.traces.iter() {
        // Draw!
        // }
    }

    fn element(&self) -> &Element {
        &self.element
    }
}
