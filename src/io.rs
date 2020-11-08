mod common;

#[cfg(windows)]
pub mod windows;

pub use common::*;

#[cfg(windows)]
pub use windows::create_directory;
