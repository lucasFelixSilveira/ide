use crossterm::{
  execute,
  cursor::MoveTo,
  terminal::size
};

pub fn get_size() -> (u16, u16) { size().unwrap() }
pub fn back_to_zero() {
  execute!(std::io::stdout(), MoveTo(0,0)).expect("_");
}

pub fn clear() {
  let win = cfg!(target_os = "windows");

  if win {
    std::process::Command::new("cmd")
      .args(&["/C", "cls"])
      .status()
      .expect("_");
  } else {
    std::process::Command::new("sh")
      .args(&["-C", "clear"])
      .status()
      .expect("_");
  }
}