use anyhow::{anyhow, Error};
use clipboard_macos::Clipboard;

pub fn set_clip(text: &str) -> Result<(), Error> {
    let mut cb = Clipboard::new().map_err(|e| anyhow!("{}", e.to_string()))?;
    cb.write(text.to_string()).map_err(|e| anyhow!("{}", e.to_string()))?;
    Ok(())
}
