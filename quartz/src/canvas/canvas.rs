use crate::geometry::Point;
use crate::style::Color;

/// A generic canvas trait. Implement this trait to
/// become a drawing canvas.
pub trait Canvas {
    fn set_pen(&mut self, color: Color);
    fn print_text(&mut self, p: &Point, text: &str);
    fn draw_line(&mut self, p1: &Point, p2: &Point);

    // fn draw(&mut self, item: &CanvasDrawAble) {
    // item.draw(self);
    // }
}

/// Implement this trait, to be able to be emitted
/// onto a canvas.
pub trait CanvasDrawAble {
    fn draw(&self, canvas: &mut dyn Canvas);
}
