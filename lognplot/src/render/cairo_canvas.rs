use super::Canvas;
use crate::style::Color;
use crate::geometry::Point;

pub struct CairoCanvas<'a> {
    cr: &'a cairo::Context,
}

impl<'a> CairoCanvas<'a> {
    pub fn new(cr: &'a cairo::Context) -> Self {
        Self { cr }
    }
}

impl<'a> Canvas for CairoCanvas<'a> {
    fn set_pen(&mut self, color: Color) {
        self.cr.set_source_rgb(
            color.r() as f64 / 255.0,
            color.g() as f64 / 255.0,
            color.b() as f64 / 255.0,
        );
    }

    fn print_text(&mut self, p: &Point, text: &str) {
        self.cr.move_to(p.x(), p.y());
        self.cr.show_text(text);
    }

    fn draw_line(&mut self, p1: &Point, p2: &Point) {
        self.cr.set_line_width(3.0);
        self.cr.move_to(p1.x(), p1.y());
        self.cr.line_to(p2.x(), p2.y());
        self.cr.stroke();
    }
}
