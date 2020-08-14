use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

use failure::Error;

use crate::err::AikotError;
use crate::io::read_file;

pub fn editor_cmd() -> Result<OsString, Error> {
    if let Some(editor) = env::var_os("EDITOR") {
        Ok(editor)
    } else {
        Err(AikotError::InvalidEnv {
            name: "EDITOR".to_string(),
        })?
    }
}

pub fn get_recipients() -> Result<Vec<String>, Error> {
    let mut path = password_store_dir()?;
    path.push(".gpg-id");
    if path.is_file() {
        let recs = read_file(&path)?
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !recs.is_empty() {
            return Ok(recs);
        }
    }
    Err(AikotError::RecipientNotFound)?
}

pub fn password_store_dir() -> Result<PathBuf, Error> {
    if let Some(dir) = env::var_os("PASSWORD_STORE_DIR") {
        Ok(PathBuf::from(dir))
    } else if let Some(home) = env::var_os("HOME") {
        let mut pbuf = PathBuf::from(home);
        pbuf.push(".password-store");
        Ok(pbuf)
    } else {
        Err(AikotError::InvalidEnv {
            name: "HOME".to_string(),
        })?
    }
}

pub fn password_store_file(name: &str) -> Result<PathBuf, Error> {
    let mut pbuf = password_store_dir()?;
    let mut file = name.to_string();
    file.push_str(".gpg");
    pbuf.push(&file);
    Ok(pbuf)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn editor_when_env_set() {
        env::set_var("EDITOR", "emacs");
        let result = editor_cmd();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), OsString::from("emacs"));
    }

    #[test]
    fn editor_when_env_not_set() {
        env::remove_var("EDITOR");
        let result = editor_cmd();
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            "invalid environment: EDITOR"
        );
    }

    #[test]
    fn password_store_dir_when_env_set() {
        env::set_var("PASSWORD_STORE_DIR", "/tmp/password-store");
        let result = password_store_dir();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/tmp/password-store"));
    }

    #[test]
    fn password_store_dir_when_home_set() {
        env::remove_var("PASSWORD_STORE_DIR");
        env::set_var("HOME", "/home/foo");
        let result = password_store_dir();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/home/foo/.password-store"));
    }

    #[test]
    fn password_store_dir_error_when_home_not_set() {
        env::remove_var("PASSWORD_STORE_DIR");
        env::remove_var("HOME");
        let result = password_store_dir();
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            "invalid environment: HOME"
        );
    }

    #[test]
    fn password_store_file_example_com() {
        env::set_var("PASSWORD_STORE_DIR", "/tmp/password-store");
        let result = password_store_file("example.com");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            PathBuf::from("/tmp/password-store/example.com.gpg")
        );
    }
}
