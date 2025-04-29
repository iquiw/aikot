use std::io::{BufWriter, Write};

use std::env::temp_dir;

use anyhow::Error;

use crate::env::AikotEnv;
use crate::gpg::{decrypt, encrypt};
use crate::io::{open_editor, read_file};
use crate::password::{AlphanumSymbol, PasswordClass, PwGen};
use crate::tempfile::create_temp_file;

pub fn cmd_edit(aikot_env: &AikotEnv, name: &str, regenerate: bool) -> Result<(), Error> {
    let dir = temp_dir();

    let (temp_path, temp_file) = create_temp_file(&dir)?;
    let mut buf_write = BufWriter::new(temp_file);
    let pass_file = aikot_env.password_store_file(name)?;
    let contents = decrypt(aikot_env, &pass_file)?;

    if regenerate {
        let mut line_no = 0;
        for line in contents.lines() {
            line_no += 1;
            if line_no == 1 {
                let pwgen = PwGen::new(line.len(), AlphanumSymbol.verify(line))?;
                writeln!(buf_write, "{} # {}", pwgen.try_generate()?, line)?;
            } else {
                writeln!(buf_write, "{}", line)?;
            }
        }
    } else {
        buf_write.write_all(contents.as_bytes())?;
    }
    drop(buf_write);

    open_editor(temp_path.as_ref())?;

    let new_contents = read_file(temp_path.as_ref())?;
    if contents == new_contents {
        println!("{} unchanged", name);
        return Ok(());
    }
    encrypt(aikot_env, &pass_file, &new_contents)
}
