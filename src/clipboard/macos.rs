use clipboard_macos::Clipboard;
use failure::{err_msg, Error};

pub fn set_clip(text: &str) -> Result<(), Error> {
    let mut cb = Clipboard::new().map_err(|e| err_msg(e.to_string()))?;
    cb.write(text.to_string())
        .map_err(|e| err_msg(e.to_string()))?;
    Ok(())
}
