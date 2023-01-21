use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::Error;

use crate::env::AikotEnv;
use crate::err::AikotError;

pub fn decrypt<P>(aikot_env: &AikotEnv, path: P) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    let output = gpg_common(aikot_env.gpg_path())
        .arg("--decrypt")
        .arg(path.as_ref())
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(AikotError::CommandFail {
            stderr: String::from_utf8(output.stderr)?,
        }.into())
    }
}

pub fn encrypt<P>(aikot_env: &AikotEnv, path: P, contents: &str) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let result = encrypt_internal(aikot_env, path, contents);
    if result.is_err() {
        eprintln!("To-be-encrypted contents:\n-----\n{}-----", contents);
    }
    result
}

fn encrypt_internal<P>(aikot_env: &AikotEnv, path: P, contents: &str) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let recipients = aikot_env.get_recipients()?;
    let mut cmd = gpg_common(aikot_env.gpg_path());
    cmd.stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--encrypt")
        .arg("-o")
        .arg(path.as_ref());
    for recipient in &recipients {
        cmd.arg("-r").arg(recipient);
    }
    let mut child = cmd.spawn()?;
    child.stdin.take().unwrap().write_all(contents.as_bytes())?;
    let output = child.wait_with_output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(AikotError::CommandFail {
            stderr: String::from_utf8(output.stderr)?,
        }.into())
    }
}

fn gpg_common(gpg_path: &Path) -> Command {
    let mut cmd = Command::new(gpg_path);
    cmd.arg("--quiet")
        .arg("--yes")
        .arg("--compress-algo=none")
        .arg("--no-encrypt-to");
    cmd
}
