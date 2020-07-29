use failure::Error;

use crate::clipboard::set_clip;
use crate::env::password_store_file;
use crate::err::AikotError;
use crate::gpg::decrypt;

pub fn cmd_clip(name: &str) -> Result<(), Error> {
    let file = password_store_file(name)?;
    if file.is_file() {
        let pass = decrypt(&file)?;
        if let Some(password) = pass.lines().next() {
            set_clip(&password)
        } else {
            Err(AikotError::EmptyPassword {
                name: name.to_string(),
            })?
        }
    } else {
        Err(AikotError::PassNotFound {
            name: name.to_string(),
        })?
    }
}
