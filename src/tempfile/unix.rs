use std::ffi::{CString, OsStr};
use std::fs::File;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::io::FromRawFd;
use std::path::{Path, PathBuf};

use anyhow::Error;
use libc::mkstemp;

use crate::io::unix::UnixError;
use crate::tempfile::common::TempPath;

pub fn create_temp_file(dir: &Path) -> Result<(TempPath, File), Error> {
    let mut temp_path_tmpl = dir.to_path_buf();
    temp_path_tmpl.push("aikot.XXXXXX");
    let temp_path_tmpl_cs = CString::new(temp_path_tmpl.into_os_string().into_vec())?;
    let temp_path_cptr = temp_path_tmpl_cs.into_raw();
    unsafe {
        let fd = mkstemp(temp_path_cptr);
        if fd == -1 {
            Err(UnixError::new("mkstemp".to_string()).into())
        } else {
            let temp_path_cs = CString::from_raw(temp_path_cptr);
            let temp_path = PathBuf::from(OsStr::from_bytes(temp_path_cs.as_ref().to_bytes()));
            Ok((TempPath::new(temp_path), File::from_raw_fd(fd)))
        }
    }
}
