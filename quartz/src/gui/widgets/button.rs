use super::Element;
use super::Widget;

pub struct Button {
    element: Element,
}

impl Button {
    pub fn new() -> Self {
        let mut element = Element::new();
        element.min_height(20.0);
        element.min_width(100.0);

        Self { element }
    }
}

impl Widget for Button {
    fn layout(&self) {}

    fn draw(&self) {
        // TODO
    }

    fn element(&self) -> &Element {
        &self.element
    }
}
