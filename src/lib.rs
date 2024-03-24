#![deny(clippy::all)]

pub mod geometry;
pub mod display;

#[cfg(target_os = "windows")]
pub mod win;

