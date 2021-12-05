use std::fs::File;
use std::os::windows::io::{FromRawHandle, RawHandle};
use std::path::{Path, PathBuf};

use anyhow::Error;
use windows::Win32::Foundation::HANDLE;

use crate::io::windows::create_file_handle;
use crate::rand::gen_random_alphanum;
use crate::tempfile::common::TempPath;

pub fn create_temp_file(dir: &Path) -> Result<(TempPath, File), Error> {
    let temp_path = create_temp_path(dir);
    let mut handle = create_file_handle(&temp_path)?;
    let raw_handle = (&mut handle as *mut HANDLE).cast::<RawHandle>();
    Ok((TempPath::new(temp_path), unsafe {
        File::from_raw_handle(*raw_handle)
    }))
}

fn create_temp_path(dir: &Path) -> PathBuf {
    let mut pb = dir.to_path_buf();
    let mut name = String::from("aikot-");
    name.push_str(&gen_random_alphanum(6));
    pb.push(name);
    pb
}
