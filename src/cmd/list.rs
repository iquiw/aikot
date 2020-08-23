use std::path::{Path, PathBuf};

use failure::Error;

use crate::env::AikotEnv;

pub fn cmd_list(aikot_env: &AikotEnv, pattern: Option<&str>) -> Result<(), Error> {
    list_dir(aikot_env.base_dir(), pattern, None)
}

fn list_dir<P>(dir: P, pattern: Option<&str>, prefix_opt: Option<&PathBuf>) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    for result in dir.as_ref().read_dir()? {
        if let Ok(entry) = result {
            let path = entry.path();
            if path.is_dir() {
                if let Some(p) = path.file_name() {
                    let mut pbuf = if let Some(prefix) = prefix_opt {
                        prefix.to_path_buf()
                    } else {
                        PathBuf::new()
                    };
                    pbuf.push(p);
                    list_dir(&path, pattern, Some(&pbuf))?;
                }
            } else if let Some(ext) = path.extension() {
                if ext == "gpg" {
                    if let Some(name) = path.file_stem() {
                        let secret = if let Some(prefix) = prefix_opt {
                            format!("{}/{}", prefix.display(), name.to_string_lossy())
                        } else {
                            format!("{}", name.to_string_lossy())
                        };
                        if pattern.is_none() || secret.contains(pattern.unwrap()) {
                            println!("{}", secret);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
