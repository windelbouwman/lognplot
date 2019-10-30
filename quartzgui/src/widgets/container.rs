use super::{Element, Widget};

/// Widget which contains other widgets
/// Children are layed out vertically
pub struct Container {
    children: Vec<Box<dyn Widget>>,
    element: Element,
}

impl Container {
    pub fn new() -> Self {
        let element = Element::new();
        Self {
            children: vec![],
            element,
        }
    }

    pub fn add_child<W>(&mut self, widget: W)
    where
        W: Widget + 'static,
    {
        self.element.align_left(widget.element());
        self.element.align_right(widget.element());
        self.children.push(Box::new(widget));
    }
}

impl Widget for Container {
    fn layout(&self) {}

    fn draw(&self) {
        for child in &self.children {
            child.draw();
        }
    }

    fn element(&self) -> &Element {
        &self.element
    }
}
