use std::ffi::{CString, OsStr};
use std::fmt;
use std::fs::File;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::io::FromRawFd;
use std::path::{Path, PathBuf};

use failure::{Error, Fail};
use libc::mkstemp;

use crate::tempfile::common::TempPath;

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

#[derive(Debug, Fail)]
struct UnixError {
    function: String,
    errno: i32,
}

impl fmt::Display for UnixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}() error, errno: {}", self.function, self.errno)
    }
}

pub fn create_temp_file(dir: &Path) -> Result<(TempPath, File), Error> {
    let mut temp_path_tmpl = dir.to_path_buf();
    temp_path_tmpl.push("aikot.XXXXXX");
    let temp_path_tmpl_cs = CString::new(temp_path_tmpl.into_os_string().into_vec())?;
    let temp_path_cptr = temp_path_tmpl_cs.into_raw();
    unsafe {
        let fd = mkstemp(temp_path_cptr);
        if fd == -1 {
            Err(UnixError {
                function: "mkstemp".to_string(),
                errno: *errno_location(),
            })?
        } else {
            let temp_path_cs = CString::from_raw(temp_path_cptr);
            let temp_path = PathBuf::from(OsStr::from_bytes(temp_path_cs.as_ref().to_bytes()));
            Ok((TempPath::new(temp_path), File::from_raw_fd(fd)))
        }
    }
}
