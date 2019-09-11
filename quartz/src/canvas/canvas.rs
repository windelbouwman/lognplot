use super::color::Color;
use crate::geometry::Point;

pub trait Canvas {
    fn set_pen(&mut self, color: Color);
    fn print_text(&mut self, p: &Point, text: &str);
    fn draw_line(&mut self, p1: &Point, p2: &Point);
}
