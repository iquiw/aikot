use failure::Error;

use crate::env::AikotEnv;
use crate::err::AikotError;
use crate::gpg::decrypt;

pub fn cmd_show(aikot_env: &AikotEnv, name: &str) -> Result<(), Error> {
    let file = aikot_env.password_store_file(name)?;
    if file.is_file() {
        let contents = decrypt(aikot_env, &file)?;
        for line in contents.lines().skip(1) {
            println!("{}", line);
        }
        Ok(())
    } else {
        Err(AikotError::PassNotFound {
            name: name.to_string(),
        }.into())
    }
}
