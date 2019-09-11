#[macro_use]
extern crate log;

pub mod canvas;
mod geometry;
pub mod plot;
// mod gui;
mod time;
mod linalg;
mod tsdb;
pub mod render_gl;

#[derive(Debug, Default)]
pub struct Context {
    hot: usize,
    active: usize,
}


pub fn begin() {}

pub fn end() {}

pub fn button(ctx: &mut Context, caption: &str) -> bool {
    // draw button

    // Check events
    return false;
}

pub fn text(text: &str) {
    // Draw text
}
