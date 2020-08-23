use std::io::{BufWriter, Write};

use std::env::temp_dir;

use failure::Error;

use crate::env::AikotEnv;
use crate::gpg::{decrypt, encrypt};
use crate::io::{open_editor, read_file};
use crate::tempfile::create_temp_file;

pub fn cmd_edit(aikot_env: &AikotEnv, name: &str) -> Result<(), Error> {
    let dir = temp_dir();

    let (temp_path, temp_file) = create_temp_file(&dir)?;
    let mut buf_write = BufWriter::new(temp_file);
    let pass_file = aikot_env.password_store_file(name)?;
    let pass = decrypt(&pass_file)?;
    buf_write.write(pass.as_bytes())?;
    drop(buf_write);

    open_editor(temp_path.as_ref())?;

    let new_pass = read_file(temp_path.as_ref())?;
    if pass == new_pass {
        println!("{} unchanged", name);
    }
    Ok(encrypt(aikot_env, &pass_file, &new_pass)?)
}
