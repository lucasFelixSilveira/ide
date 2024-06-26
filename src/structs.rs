use std::path::PathBuf;

use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;

#[derive(Debug, Clone)]
pub struct Editor {
  pub page_down: usize,
  pub file: usize,
  pub prop: usize,
  pub quit: bool,
  pub files: Vec<File>,
  pub interface: Interface,
  pub content: String,
  pub mode: Mode,
  pub cursor: (usize, usize),
  pub selection: ((usize, usize), (usize, usize)),
  pub listen: (String, LMemory, String)
}

impl Editor {
  pub fn new() -> Editor {
    Editor {
      page_down: 0,
      files: Vec::new(),
      file: 0,
      prop: 0,
      quit: false,
      interface: Interface::Files,
      content: String::new(),
      mode: Mode::Movement,
      cursor: (0,0),
      selection: ((0,0), (0,0)),
      listen: (String::new(), LMemory::Unknown, String::new())
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
  Editor,
  Properties
}

#[derive(Debug, PartialEq, Clone)]
pub enum LMemory {
  Unknown,
  Rename,
  DeleteFile,
  DeleteFolder,
  NFolder,
  NFile
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
  Movement,
  Insert,
  Listen,
  Overwrite,
  Selection,
  Clipboard
}

#[derive(Debug, Clone)]
pub struct KeyEvents {
  pub code: KeyCode,
  pub modifiers: KeyModifiers
}

#[derive(Debug, Clone)]
pub struct FOption {
  pub fun: FEOption,
  pub label: String,
  pub symbol: char
}

impl FOption {
  pub fn properties(editor: &mut Editor) -> Vec<FOption> {
    let mut options = vec![
      FOption {
        fun: FEOption::Back,
        label: String::from("Back"),
        symbol: '<'
      },
      FOption {
        fun: FEOption::Rename,
        label: String::from("Raname"),
        symbol: '#'
      },
      FOption {
        fun: FEOption::NewF,
        label: String::from("New folder"),
        symbol: '>'
      },
      FOption {
        fun: FEOption::New,
        label: String::from("New file"),
        symbol: '+'
      }
    ];

    if editor.files.len() > 0 {
      let file = editor.files[editor.file].clone();

      options.insert(1, FOption {
        fun: FEOption::Delete,
        label: format!("Delete the '{}' {}", file.name, if file.is_folder { "folder" } else { "file" }),
        symbol: 'x'
      });
    } 

    options
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FEOption {
  Back,
  Delete,
  Rename,
  NewF,
  New
}