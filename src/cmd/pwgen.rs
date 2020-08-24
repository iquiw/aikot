use failure::Error;

use crate::env::AikotEnv;
use crate::err::AikotError;
use crate::password::{Alphanum, PasswordClass};

pub fn cmd_pwgen(_aikot_env: &AikotEnv, length: usize, count: u16) -> Result<(), Error> {
    let pwclass = Alphanum;
    if length < pwclass.minimum_length() {
        return Err(AikotError::MinimumLength {
            pwclass: format!("{}", pwclass).to_string(),
            length: pwclass.minimum_length(),
        })?;
    }
    for _i in 0..count {
        if let Some(pass) = pwclass.try_generate(length) {
            println!("{}", pass);
        } else {
            Err(AikotError::GenerationFail {
                pwclass: format!("{}", pwclass).to_string(),
                length,
            })?
        }
    }
    Ok(())
}
