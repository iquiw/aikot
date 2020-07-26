use std::path::{Path, PathBuf};

use failure::Error;

use crate::env::password_store_dir;

pub fn cmd_list() -> Result<(), Error> {
    let dir = password_store_dir()?;
    list_dir(&dir, None)
}

fn list_dir<P>(dir: P, prefix_opt: Option<&PathBuf>) -> Result<(), Error>
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
                    list_dir(&path, Some(&pbuf))?;
                }
            } else if let Some(ext) = path.extension() {
                if ext == "gpg" {
                    if let Some(name) = path.file_stem() {
                        if let Some(prefix) = prefix_opt {
                            println!("{}/{}", prefix.display(), name.to_string_lossy());
                        } else {
                            println!("{}", name.to_string_lossy());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
