use anyhow::Error;

use crate::browser::browser_command;
use crate::env::AikotEnv;
use crate::err::AikotError;
use crate::gpg::decrypt;

pub fn cmd_browse(aikot_env: &AikotEnv, name: &str) -> Result<(), Error> {
    let file = aikot_env.password_store_file(name)?;
    if file.is_file() {
        let contents = decrypt(aikot_env, &file)?;
        for line in contents.lines().skip(1) {
            if let Some(val) = line.strip_prefix("url:") {
                let url = val.trim();
                browser_command().arg(url).spawn()?.wait()?;
                return Ok(());
            }
        }
        Err(AikotError::UrlNotFound {
            name: name.to_string(),
        }.into())
    } else {
        Err(AikotError::PassNotFound {
            name: name.to_string(),
        }.into())
    }
}
