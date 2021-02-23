mod common;

#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

pub use common::*;

#[cfg(unix)]
pub use unix::create_directory;
#[cfg(windows)]
pub use self::windows::create_directory;
