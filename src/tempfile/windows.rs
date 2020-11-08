use std::fs::File;
use std::os::windows::io::FromRawHandle;
use std::path::{Path, PathBuf};

use anyhow::Error;

use crate::io::windows::create_file_handle;
use crate::rand::gen_random_alphanum;
use crate::tempfile::common::TempPath;

pub fn create_temp_file(dir: &Path) -> Result<(TempPath, File), Error> {
    let temp_path = create_temp_path(dir)?;
    let handle = create_file_handle(&temp_path)?;
    Ok((TempPath::new(temp_path), unsafe {
        File::from_raw_handle(handle.cast())
    }))
}

fn create_temp_path(dir: &Path) -> Result<PathBuf, Error> {
    let mut pb = dir.to_path_buf();
    let mut name = String::from("aikot-");
    name.push_str(&gen_random_alphanum(6));
    pb.push(name);
    Ok(pb)
}
