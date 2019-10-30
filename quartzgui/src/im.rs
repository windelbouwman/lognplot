//! Immediate mode gui
//!

#[derive(Debug, Default)]
pub struct Context {
    hot: usize,
    active: usize,
}

pub fn begin() {}

pub fn end() {}

pub fn button(_ctx: &mut Context, _caption: &str) -> bool {
    // draw button

    // Check events
    return false;
}

pub fn text(_text: &str) {
    // Draw text
}
