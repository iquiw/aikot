#[cfg(windows)]
fn main() {
    windows::build!(
        windows::application_model::data_transfer::{Clipboard, ClipboardContentOptions, DataPackage}
    );
}

#[cfg(not(windows))]
fn main() {
}
