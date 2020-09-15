use std::process::Command;

#[cfg(all(unix, not(target_os = "macos")))]
pub fn browser_command() -> Command {
    Command::new("xdg-open")
}

#[cfg(target_os = "macos")]
pub fn browser_command() -> Command {
    Command::new("open")
}

#[cfg(windows)]
pub fn browser_command() -> Command {
    let mut cmd = Command::new("cmd");
    cmd.arg("/c").arg("start");
    cmd
}


