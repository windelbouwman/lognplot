#[macro_use]
extern crate log;

pub mod canvas;
pub mod geometry;
pub mod gui;
mod layout;
pub mod plot;
mod style;
mod time;
pub mod tsdb;
mod widgets;

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
