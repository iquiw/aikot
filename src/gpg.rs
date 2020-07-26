use std::path::Path;
use std::process::Command;

use failure::Error;

use crate::err::AikotError;

pub fn decrypt<P>(path: P) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    let output = Command::new("gpg")
        .arg("--decrypt")
        .arg(path.as_ref())
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(AikotError::CommandFail {
            stderr: String::from_utf8(output.stderr)?,
        })?
    }
}
