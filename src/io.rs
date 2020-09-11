use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use failure::Error;

use crate::env::editor_cmd;
use crate::err::AikotError;

pub fn open_editor(path: &Path) -> Result<(), Error> {
    let editor = editor_cmd()?;
    let status = Command::new(&editor).arg(path).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(AikotError::CommandFail {
            stderr: format!("{} exits with status: {}", editor.to_string_lossy(), status),
        }.into())
    }
}

pub fn read_file(path: &Path) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}
