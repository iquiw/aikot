#[cfg(windows)]
fn main() {
    windows::build! {
        Windows::ApplicationModel::DataTransfer::{Clipboard, ClipboardContentOptions, DataPackage}
    }
}

#[cfg(not(windows))]
fn main() {
}
