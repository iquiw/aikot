use failure::Error;

use crate::clipboard::set_clip;
use crate::env::AikotEnv;
use crate::err::AikotError;
use crate::gpg::decrypt;

pub fn cmd_clip(aikot_env: &AikotEnv, name: &str) -> Result<(), Error> {
    let file = aikot_env.password_store_file(name)?;
    if file.is_file() {
        let contents = decrypt(aikot_env, &file)?;
        if let Some(pass) = contents.lines().next() {
            set_clip(&pass)
        } else {
            Err(AikotError::EmptyPassword {
                name: name.to_string(),
            }.into())
        }
    } else {
        Err(AikotError::PassNotFound {
            name: name.to_string(),
        }.into())
    }
}
