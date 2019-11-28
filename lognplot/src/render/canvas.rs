use crate::geometry::Point;
use crate::style::Color;

pub enum VerticalAnchor {
    Top,
    Middle,
    Bottom,
}

pub enum HorizontalAnchor {
    Left,
    Middle,
    Right,
}

/// A generic canvas trait. Implement this trait to
/// become a drawing canvas.
pub trait Canvas {
    fn set_pen(&mut self, color: Color, alpha: f64);
    fn set_line_width(&mut self, width: f64);
    fn print_text(
        &mut self,
        p: &Point,
        horizontal_anchor: HorizontalAnchor,
        vertical_anchor: VerticalAnchor,
        text: &str,
    );
    fn draw_line(&mut self, points: &[Point]);
    fn draw_polygon(&mut self, points: &[Point]);
    fn draw_circle(&mut self, center: &Point, radius: f64);
    fn fill_polygon(&mut self, points: &[Point]);
}
