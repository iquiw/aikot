use anyhow::{anyhow, Error};

::windows::include_bindings!();

use Windows::ApplicationModel::DataTransfer::{Clipboard, ClipboardContentOptions, DataPackage};

pub fn set_clip(text: &str) -> std::result::Result<(), Error> {
    set_clip_win(text).map_err(|e| anyhow!("{}", e.message()))
}

fn set_clip_win(text: &str) -> windows::Result<()> {
    let cco = ClipboardContentOptions::new()?;
    cco.SetIsAllowedInHistory(false)?;
    cco.SetIsRoamable(false)?;
    let dp = DataPackage::new()?;
    dp.SetText(text)?;
    Clipboard::SetContentWithOptions(dp, cco)?;
    Clipboard::Flush()?;
    Ok(())
}
