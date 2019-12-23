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

    fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let path = make_rect(x, y, width, height);
        self.draw_polygon(&path);
    }

    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let path = make_rect(x, y, width, height);
        self.fill_polygon(&path);
    }
}

/// Create a path from a rectangle definition.
fn make_rect(x: f64, y: f64, width: f64, height: f64) -> Vec<Point> {
    let top_left = Point::new(x, y);
    let bottom_left = Point::new(x, y + height);
    let top_right = Point::new(x + width, y);
    let bottom_right = Point::new(x + width, y + height);

    vec![top_left, top_right, bottom_right, bottom_left]
}
