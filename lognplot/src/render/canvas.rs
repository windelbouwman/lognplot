use crate::geometry::Point;
use crate::style::Color;

/// A generic canvas trait. Implement this trait to
/// become a drawing canvas.
pub trait Canvas {
    fn set_pen(&mut self, color: Color);
    fn set_line_width(&mut self, width: f64);
    fn print_text(&mut self, p: &Point, text: &str);
    fn draw_line(&mut self, points: &[Point]);
    fn draw_polygon(&mut self, points: &[Point]);
    fn fill_polygon(&mut self, points: &[Point]);
}
