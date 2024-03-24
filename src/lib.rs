#![deny(clippy::all)]

pub mod geometry;
pub mod screen;
pub mod utils;

#[cfg(target_os = "windows")]
pub mod win;

