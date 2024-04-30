use crossterm::terminal::size;
use std::{io::Write, process::Command};

pub fn dimensions() -> (u16, u16) {
    size().unwrap()
}

pub fn clear() {
    let is_windows = cfg!(target_os = "windows");
    if is_windows {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("_");
    } else {
        Command::new("sh")
            .args(&["-c", "clear"])
            .status()
            .expect("_");
    }
    std::io::stdout().flush().unwrap();
}
