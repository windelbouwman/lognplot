use super::canvas::Canvas;
use crate::geometry::Point;
use crate::style::Color;

use std::io::Write;

/// Output to SVG file format!
pub struct SvgOutput<'w> {
    file: &'w mut dyn Write,
    pen: Color,
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
        }
    }
}

/// Implement the canvas API for svg output!
impl<'w> Canvas for SvgOutput<'w> {
    fn set_pen(&mut self, color: Color) {
        self.pen = color;
    }

    fn print_text(&mut self, p: &Point, text: &str) {
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

    /// Draw a line between two points.
    fn draw_line(&mut self, p1: &Point, p2: &Point) {
        trace!("Line between {:?} and {:?}", p1, p2);
        let style = format!(
            r#"stroke:rgb({},{},{});stroke-width:2"#,
            self.pen.r(),
            self.pen.g(),
            self.pen.b()
        );
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
    }
}

/// Implement drop destructor so we can write the closing svg tag.
impl<'w> Drop for SvgOutput<'w> {
    fn drop(&mut self) {
        writeln!(self.file, "</svg>").unwrap();
    }
}
