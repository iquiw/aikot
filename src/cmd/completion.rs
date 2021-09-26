use anyhow::Error;

use crate::env::{AikotEnv, ShellType};

pub fn cmd_completion(_aikot_env: &AikotEnv, shell: ShellType) -> Result<(), Error> {
    if shell == ShellType::Bash {
        println!("{}", include_str!("../../script/bash-completion.bash"));
    }
    Ok(())
}
