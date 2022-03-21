use std::env::temp_dir;
use std::io::{BufWriter, Write};

use anyhow::Error;

use crate::env::AikotEnv;
use crate::err::AikotError;
use crate::gpg::encrypt;
use crate::io::{open_editor, read_file};
use crate::password::PwGen;
use crate::tempfile::create_temp_file;
use crate::template::PassTmpl;

pub fn cmd_add(aikot_env: &AikotEnv, name: &str, opwgen: Option<&PwGen>) -> Result<(), Error> {
    let pass_file = aikot_env.password_store_file(name)?;
    if pass_file.exists() {
        return Err(AikotError::PassAlreadyExists {
            name: name.to_string(),
        }
        .into());
    }
    let dir = temp_dir();
    let (temp_path, temp_file) = create_temp_file(&dir)?;

    let tmpl_path = aikot_env.template_file();
    let mut ptmpl = PassTmpl::new();
    if tmpl_path.is_file() {
        ptmpl.load(tmpl_path)?;
    } else {
        ptmpl.load_default()?;
    }
    let pass = if let Some(pwgen) = opwgen {
        pwgen.try_generate()?
    } else {
        "".to_string()
    };

    let mut buf_write = BufWriter::new(temp_file);
    write!(buf_write, "{}", ptmpl.render(&pass, name)?)?;
    drop(buf_write);

    open_editor(temp_path.as_ref())?;

    let new_contents = read_file(temp_path.as_ref())?;
    if new_contents.is_empty() {
        return Err(AikotError::EmptyPassword {
            name: name.to_string(),
        }
        .into());
    }
    // check again
    if pass_file.exists() {
        return Err(AikotError::PassAlreadyExists {
            name: name.to_string(),
        }
        .into());
    }
    encrypt(aikot_env, &pass_file, &new_contents)
}
