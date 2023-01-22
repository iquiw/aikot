use std::ffi::CString;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{anyhow, Error};
use arboard::Clipboard;

pub fn set_clip(text: &str) -> Result<(), Error> {
    unsafe {
        daemonize()?;
    };

    let mut cb = Clipboard::new()?;
    cb.set_text(text)?;

    sleep(Duration::from_secs(45));
    Ok(())
}

unsafe fn daemonize() -> Result<(), Error> {
    let child = libc::fork();
    if child == -1 {
        return Err(anyhow!("fork() failed"));
    } else if child > 0 {
        libc::_exit(0);
    }
    let result = libc::setsid();
    if result == -1 {
        exit(0);
    }
    let dev_null = CString::new("/dev/null")?;
    let fd = libc::open(dev_null.into_raw(), libc::O_RDWR, 0);
    if fd != -1 {
        libc::dup2(fd, libc::STDIN_FILENO);
        libc::dup2(fd, libc::STDOUT_FILENO);
        libc::dup2(fd, libc::STDERR_FILENO);
    }
    Ok(())
}
