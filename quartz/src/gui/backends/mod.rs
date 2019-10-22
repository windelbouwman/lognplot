//! Idea: backends can draw stuff, and the rest of the application
//! is relatively unaware of the drawing tech used.
//!
//! For example, one can draw with openGL or vulkan, using a different
//! backend.

mod gl;
pub mod vulkan;
