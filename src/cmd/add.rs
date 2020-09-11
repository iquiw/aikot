use std::env::temp_dir;
use std::io::{BufWriter, Write};

use failure::Error;

use crate::env::AikotEnv;
use crate::err::AikotError;
use crate::gpg::encrypt;
use crate::io::{open_editor, read_file};
use crate::password::PwGen;
use crate::tempfile::create_temp_file;

pub fn cmd_add(aikot_env: &AikotEnv, name: &str, opwgen: Option<&PwGen>) -> Result<(), Error> {
    let pass_file = aikot_env.password_store_file(name)?;
    if pass_file.exists() {
        return Err(AikotError::PassAlreadyExists {
            name: name.to_string(),
        })?;
    }
    let dir = temp_dir();
    let (temp_path, temp_file) = create_temp_file(&dir)?;

    if let Some(pwgen) = opwgen {
        let pass = pwgen.try_generate()?;

        let mut buf_write = BufWriter::new(temp_file);
        buf_write.write_all(pass.as_bytes())?;
        buf_write.write_all(b"\n")?;
        drop(buf_write);
    }

    open_editor(temp_path.as_ref())?;

    let new_contents = read_file(temp_path.as_ref())?;
    if new_contents.is_empty() {
        return Err(AikotError::EmptyPassword {
            name: name.to_string(),
        })?;
    }
    // check again
    if pass_file.exists() {
        return Err(AikotError::PassAlreadyExists {
            name: name.to_string(),
        })?;
    }
    Ok(encrypt(aikot_env, &pass_file, &new_contents)?)
}
