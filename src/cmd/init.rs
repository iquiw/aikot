use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Error;

use crate::env::AikotEnv;
use crate::err::AikotError;
use crate::io::create_directory;

pub fn cmd_init(aikot_env: &AikotEnv, gpg_ids: &[String]) -> Result<(), Error> {
    if gpg_ids.is_empty() {
        return Err(AikotError::GpgIdRequired.into());
    }
    let base_dir = aikot_env.base_dir();
    let path = aikot_env.gpg_id_path();
    if path.exists() {
        return Err(AikotError::AlreadyInitialized {
            path: format!("{}", base_dir.display()),
        }
        .into());
    }
    if !base_dir.is_dir() {
        create_directory(base_dir)?;
    }

    let mut w = BufWriter::new(File::create(path)?);
    for gpg_id in gpg_ids {
        writeln!(w, "{}", gpg_id)?;
    }
    drop(w);
    println!("Password store initialized: {}", base_dir.display());
    Ok(())
}
