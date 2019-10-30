use super::element::Element;

pub trait Widget {
    fn layout(&self);
    fn draw(&self);

    /// Retrieve the element for this widget.
    fn element(&self) -> &Element;
}
