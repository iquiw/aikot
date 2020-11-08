use std::ffi::CString;
use std::fmt;
use std::os::unix::ffi::OsStringExt;
use std::path::Path;

use anyhow::Error;

// Copied from src/util_libc.rs in https://github.com/rust-random/getrandom.
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
use libc::___errno as errno_location;
#[cfg(any(target_os = "netbsd", target_os = "openbsd", target_os = "android"))]
use libc::__errno as errno_location;
#[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "redox"))]
use libc::__errno_location as errno_location;
#[cfg(any(target_os = "macos", target_os = "freebsd"))]
use libc::__error as errno_location;
#[cfg(target_os = "haiku")]
use libc::_errnop as errno_location;

#[derive(Debug, thiserror::Error)]
pub struct UnixError {
    function: String,
    errno: i32,
}

impl UnixError {
    pub fn new(function: String) -> Self {
        unsafe {
            UnixError {
                function,
                errno: *errno_location(),
            }
        }
    }
}

impl fmt::Display for UnixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}() error, errno: {}", self.function, self.errno)
    }
}

pub fn create_directory(path: &Path) -> Result<(), Error> {
    let path_cs = CString::new(path.to_path_buf().into_os_string().into_vec())?;
    let result = unsafe { libc::mkdir(path_cs.into_raw(), 0o700) };
    if result == 0 {
        Ok(())
    } else {
        Err(UnixError::new("mkdir".to_string()).into())
    }
}
