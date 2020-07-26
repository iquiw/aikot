use failure::Error;

use crate::env::password_store_file;
use crate::err::AikotError;
use crate::gpg::decrypt;

pub fn cmd_show(name: &str) -> Result<(), Error> {
    let file = password_store_file(name)?;
    if file.is_file() {
        let pass = decrypt(&file)?;
        for line in pass.lines().skip(1) {
            println!("{}", line);
        }
        Ok(())
    } else {
        Err(AikotError::PassNotFound {
            name: name.to_string(),
        })?
    }
}
