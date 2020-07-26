use std::env;
use std::path::PathBuf;

use failure::Error;

use crate::err::AikotError;

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

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

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
        assert_eq!(format!("{}", result.unwrap_err()), "invalid environment: HOME");
    }
}
