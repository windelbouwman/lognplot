use super::canvas::{HorizontalAnchor, VerticalAnchor};
use super::Canvas;
use crate::geometry::Point;
use crate::style::Color;

use std::io::Write;

/// Output to SVG file format!
pub struct SvgOutput<'w> {
    file: &'w mut dyn Write,
    pen: Color,
    width: f64,
}

impl<'w> SvgOutput<'w> {
    pub fn new(file: &'w mut dyn Write) -> Self {
        let width = 1000;
        let height = 1000;
        writeln!(file, r#"<?xml version="1.0" encoding="UTF-8" ?>"#).unwrap();
        writeln!(
            file,
            r#"<svg width="{0}" height="{1}" viewBox="0 0 {0} {1}">"#,
            width, height
        )
        .unwrap();
        SvgOutput {
            file,
            pen: Color::black(),
            width: 1.0,
        }
    }

    fn get_stroke_style(&self) -> String {
        format!(
            r#"stroke:rgb({},{},{});stroke-width:2"#,
            self.pen.r(),
            self.pen.g(),
            self.pen.b()
        )
    }

    /// Convert array of points into SVG points string.
    fn points_to_string(points: &[Point]) -> String {
        let point_texts: Vec<String> = points
            .iter()
            .map(|p| format!("{},{}", p.x(), p.y()))
            .collect();
        point_texts.join(" ")
    }
}

/// Implement the canvas API for svg output!
impl<'w> Canvas for SvgOutput<'w> {
    fn set_pen(&mut self, color: Color, _alpha: f64) {
        self.pen = color;
    }

    fn set_line_width(&mut self, width: f64) {
        self.width = width;
    }

    fn print_text(
        &mut self,
        p: &Point,
        _horizontal_anchor: HorizontalAnchor,
        _vertical_anchor: VerticalAnchor,
        text: &str,
    ) {
        info!("Printing text! {}", text);
        writeln!(
            self.file,
            r#"   <text x="{}" y="{}">{}</text>"#,
            p.x(),
            p.y(),
            text
        )
        .unwrap();
    }

    /// Draw a line between points.
    fn draw_line(&mut self, points: &[Point]) {
        let style = self.get_stroke_style();

        if points.len() == 2 {
            let p1 = points[0];
            let p2 = points[1];
            trace!("Line between {:?} and {:?}", p1, p2);
            writeln!(
                self.file,
                r#"   <line x1="{}" y1="{}" x2="{}" y2="{}" style="{}" />"#,
                p1.x(),
                p1.y(),
                p2.x(),
                p2.y(),
                style
            )
            .unwrap();
        } else if points.len() > 2 {
            let point_text = Self::points_to_string(points);
            writeln!(
                self.file,
                r#"   <polyline points="{}" style="{}" />"#,
                point_text, style
            )
            .unwrap();
        }
    }

    fn draw_polygon(&mut self, points: &[Point]) {
        if points.len() > 2 {
            let style = self.get_stroke_style();

            let point_text = Self::points_to_string(points);
            writeln!(
                self.file,
                r#"   <polygon points="{}" style="{}" />"#,
                point_text, style
            )
            .unwrap();
        }
    }

    fn fill_polygon(&mut self, points: &[Point]) {
        if points.len() > 2 {
            let style = format!(
                r#"fill:rgb({},{},{});stroke-width:1"#,
                self.pen.r(),
                self.pen.g(),
                self.pen.b()
            );

            let point_text = Self::points_to_string(points);
            writeln!(
                self.file,
                r#"   <polygon points="{}" style="{}" />"#,
                point_text, style
            )
            .unwrap();
        }
    }

    fn draw_circle(&mut self, _center: &Point, _radius: f64) {
        // TODO!
    }
}

/// Implement drop destructor so we can write the closing svg tag.
impl<'w> Drop for SvgOutput<'w> {
    fn drop(&mut self) {
        writeln!(self.file, "</svg>").unwrap();
    }
}
