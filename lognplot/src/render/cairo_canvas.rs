use super::Canvas;
use crate::geometry::Point;
use crate::style::Color;

pub struct CairoCanvas<'a> {
    cr: &'a cairo::Context,
}

impl<'a> CairoCanvas<'a> {
    pub fn new(cr: &'a cairo::Context) -> Self {
        Self { cr }
    }
}

impl<'a> CairoCanvas<'a> {
    fn make_path(&self, points: &[Point]) {
        let (first, rest) = points.split_first().unwrap();
        self.cr.move_to(first.x(), first.y());
        for p in rest {
            self.cr.line_to(p.x(), p.y());
        }
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

    fn set_line_width(&mut self, width: f64) {
        self.cr.set_line_width(width);
    }

    fn print_text(&mut self, p: &Point, text: &str) {
        self.cr.move_to(p.x(), p.y());
        self.cr.show_text(text);
    }

    fn draw_line(&mut self, points: &[Point]) {
        if points.len() > 1 {
            self.make_path(points);
            self.cr.stroke();
        }
    }

    fn draw_polygon(&mut self, points: &[Point]) {
        if points.len() > 1 {
            self.make_path(points);
            self.cr.close_path();
            self.cr.stroke();
        }
    }

    fn fill_polygon(&mut self, points: &[Point]) {
        if points.len() > 1 {
            self.make_path(points);
            self.cr.close_path();
            self.cr.fill();
        }
    }
}
