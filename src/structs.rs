use std::path::PathBuf;

use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;

#[derive(Debug, Clone)]
pub struct Editor {
  pub page_down: usize,
  pub file: usize,
  pub quit: bool,
  pub files: Vec<File>,
  pub interface: Interface,
  pub content: String,
  pub mode: Mode,
  pub cursor: (usize, usize)
}

impl Editor {
  pub fn new() -> Editor {
    Editor {
      page_down: 0,
      files: Vec::new(),
      file: 0,
      quit: false,
      interface: Interface::Files,
      content: String::new(),
      mode: Mode::Movement,
      cursor: (0,0)
    }
  }
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub is_folder: bool
}

#[derive(Debug, PartialEq, Clone)]
pub enum Interface {
  Files,
  Editor
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
  Movement,
  Insert
}

#[derive(Debug, Clone)]
pub struct KeyEvents {
    pub code: KeyCode,
    pub modifiers: KeyModifiers
}