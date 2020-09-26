use anyhow::{anyhow, Error};
use winrt::*;

import!(
    dependencies
        os
    types
        windows::application_model::data_transfer::*
);

use windows::application_model::data_transfer::{Clipboard, ClipboardContentOptions, DataPackage};

pub fn set_clip(text: &str) -> std::result::Result<(), Error> {
    set_clip_win(text).map_err(|e| anyhow!("{}", e.message()))
}

fn set_clip_win(text: &str) -> Result<()> {
    let cco = ClipboardContentOptions::new()?;
    cco.set_is_allowed_in_history(false)?;
    cco.set_is_roamable(false)?;
    let dp = DataPackage::new()?;
    dp.set_text(text)?;
    Clipboard::set_content_with_options(dp, cco)?;
    Clipboard::flush()?;
    Ok(())
}
