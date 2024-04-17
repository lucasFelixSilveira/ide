use crossterm::terminal::size;
use std::process::Command;

pub fn dimensions() -> (u16, u16) {
    size().unwrap()
}

pub fn clear() {
    print!("\033[2J\033[1;1H");
    std::io::stdout().flush().expect("failed to flush stdout");
}
