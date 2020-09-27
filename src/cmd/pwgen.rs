use anyhow::Error;

use crate::env::AikotEnv;
use crate::password::PwGen;

pub fn cmd_pwgen(_aikot_env: &AikotEnv, pwgen: &PwGen, count: u16) -> Result<(), Error> {
    for _i in 0..count {
        println!("{}", pwgen.try_generate()?);
    }
    Ok(())
}
