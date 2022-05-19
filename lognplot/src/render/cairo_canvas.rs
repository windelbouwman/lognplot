use super::canvas::{HorizontalAnchor, VerticalAnchor};
use super::Canvas;
use crate::geometry::{Point, Size};
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
        let (first, rest) = points
            .split_first()
            .expect("At least two points to make a line");
        self.cr.move_to(first.x(), first.y());
        for p in rest {
            self.cr.line_to(p.x(), p.y());
        }
    }
}

impl<'a> Canvas for CairoCanvas<'a> {
    fn set_pen(&mut self, color: Color, alpha: f64) {
        self.cr.set_source_rgba(
            color.r() as f64 / 255.0,
            color.g() as f64 / 255.0,
            color.b() as f64 / 255.0,
            alpha,
        );
        // TODO: using a different filter might give better performance?
        // let pattern = self.cr.get_source();
        // println!("Filter {:?}", pattern.get_filter());
        // pattern.set_filter(cairo::Filter::Fast);
    }

    fn set_line_width(&mut self, width: f64) {
        self.cr.set_line_width(width);
    }

    fn draw_circle(&mut self, center: &Point, radius: f64) {
        self.cr.new_path();
        self.cr.arc(
            center.x(),
            center.y(),
            radius,
            0.0,
            2.0 * std::f64::consts::PI,
        );
        self.cr.stroke().unwrap();
    }

    fn text_size(&self, text: &str) -> Size {
        let extents = self.cr.text_extents(text).unwrap();
        Size::new(extents.width, extents.height)
    }

    fn print_text(
        &mut self,
        p: &Point,
        horizontal_anchor: HorizontalAnchor,
        vertical_anchor: VerticalAnchor,
        text: &str,
    ) {
        // Draw origin, for debugging
        // self.cr.arc(p.x(), p.y(), 1.0, 0.0, 6.28);
        // self.cr.stroke();

        // https://www.cairographics.org/manual/cairo-cairo-scaled-font-t.html#cairo-text-extents-t
        let extents = self.cr.text_extents(text).unwrap();

        let x_offset = match horizontal_anchor {
            HorizontalAnchor::Left => 0.0,
            HorizontalAnchor::Middle => extents.width * 0.5,
            HorizontalAnchor::Right => extents.width,
        };
        let y_offset = match vertical_anchor {
            VerticalAnchor::Top => 0.0,
            VerticalAnchor::Middle => extents.height * 0.5,
            VerticalAnchor::Bottom => extents.height,
        };
        let x = p.x() - extents.x_bearing - x_offset;
        let y = p.y() - extents.y_bearing - y_offset;

        self.cr.move_to(x, y);
        self.cr.show_text(text).unwrap();
    }

    fn draw_line(&mut self, points: &[Point]) {
        if points.len() > 1 {
            self.make_path(points);
            self.cr.stroke().unwrap();
        }
    }

    fn draw_polygon(&mut self, points: &[Point]) {
        if points.len() > 1 {
            self.make_path(points);
            self.cr.close_path();
            self.cr.stroke().unwrap();
        }
    }

    fn fill_polygon(&mut self, points: &[Point]) {
        if points.len() > 1 {
            self.make_path(points);
            self.cr.close_path();
            self.cr.fill().unwrap();
        }
    }
}
