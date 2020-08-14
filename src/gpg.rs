use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use failure::Error;

use crate::env::get_recipients;
use crate::err::AikotError;

pub fn decrypt<P>(path: P) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    let output = gpg_common().arg("--decrypt").arg(path.as_ref()).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(AikotError::CommandFail {
            stderr: String::from_utf8(output.stderr)?,
        })?
    }
}

pub fn encrypt<P>(path: P, pass: &str) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let recipients = get_recipients()?;
    let mut cmd = gpg_common();
    cmd.stdin(Stdio::piped())
        .arg("--encrypt")
        .arg("-o")
        .arg(path.as_ref());
    for recipient in &recipients {
        cmd.arg("-r").arg(recipient);
    }
    let child = cmd.spawn()?;
    Ok(child.stdin.unwrap().write_all(pass.as_bytes())?)
}

fn gpg_common() -> Command {
    let mut cmd = Command::new("gpg");
    cmd.arg("--quiet")
        .arg("--yes")
        .arg("--compress-algo=none")
        .arg("--no-encrypt-to");
    cmd
}
