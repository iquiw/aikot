use std::env::current_exe;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{anyhow, Error};

use windows::core::{Result, HSTRING};
use windows::ApplicationModel::DataTransfer::{Clipboard, ClipboardContentOptions, DataPackage};

pub fn set_clip(text: &str) -> std::result::Result<(), Error> {
    if let Ok(exe) = current_exe() {
        let _ = Command::new(exe).arg("unclip").spawn();
    }
    set_clip_win(text).map_err(|e| anyhow!("{}", e.message()))
}

fn set_clip_win(text: &str) -> Result<()> {
    let cco = ClipboardContentOptions::new()?;
    cco.SetIsAllowedInHistory(false)?;
    cco.SetIsRoamable(false)?;
    let dp = DataPackage::new()?;
    dp.SetText(&HSTRING::from(text))?;
    Clipboard::SetContentWithOptions(&dp, &cco)?;
    Clipboard::Flush()?;
    Ok(())
}

pub fn clear_clip() -> std::result::Result<(), Error> {
    sleep(Duration::from_secs(45));
    Clipboard::Clear().map_err(|e| anyhow!("{}", e.message()))
}
