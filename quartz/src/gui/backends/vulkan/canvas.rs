use super::VulkanEngine;
use crate::canvas::Canvas;
use crate::geometry::Point;
use crate::style::Color;

/// Canvas implementation onto the vulkan engine!
impl Canvas for VulkanEngine {
    fn set_pen(&mut self, color: Color) {}

    fn print_text(&mut self, p: &Point, text: &str) {
        self.draw_text(p.x() as f32, p.y() as f32, text);
    }

    fn draw_line(&mut self, p1: &Point, p2: &Point) {
        // self.draw_line_inner(p1, p2);
    }
}
