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
        println!("{}", generate_one(&pwclass, length)?);
    }
    Ok(())
}

pub fn generate_one<C>(pwclass: &C, length: usize) -> Result<String, Error>
where
    C: PasswordClass + std::fmt::Display,
{
    if let Some(pass) = pwclass.try_generate(length) {
        Ok(pass)
    } else {
        Err(AikotError::GenerationFail {
            pwclass: format!("{}", pwclass).to_string(),
            length,
        })?
    }
}
