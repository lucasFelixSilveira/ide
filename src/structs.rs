use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interface {
    Files,
    Editor
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Insert,
    Movement,
    Command,
    Watching
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub is_folder: bool
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub file: usize
}

impl Cursor {
    pub fn default() -> Cursor {
        Cursor { x: 0, y: 0, z: 0, file: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub waiting: bool,
    pub context: usize,
    pub current: String
}

impl Command {
    pub fn default() -> Command {
        Command {
            waiting: false,
            context: 0,
            current: String::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoopPerson {
    pub color: usize,
    pub name: String,
    pub id: usize
}

#[derive(Debug, Clone)]
pub struct CoopCoding {
    pub members: Vec<CoopPerson>,
    pub possession: usize,
    pub enable: bool,
    pub started: bool
}

#[derive(Debug, Clone)]
pub struct Editor {
    pub interface: Interface,
    pub force_updates: bool,
    pub cursor: Cursor,
    pub files: Vec<File>,
    pub stopped: bool,
    pub before_mode: Mode,
    pub mode: Mode,
    pub file_tmp: String,
    pub input_command: String,
    pub output: String,
    pub file_lines: usize,
    pub file_lines_vec: Vec<String>,
    pub command: Command,
    pub force_quant: usize,
    pub updated: usize,
    pub coop: CoopCoding
}

impl Editor {
    pub fn default() -> Editor {
        Editor {
            interface: Interface::Files,
            force_updates: false,
            cursor: Cursor::default(),
            files: Vec::new(),
            stopped: false,
            mode: Mode::Movement,
            before_mode: Mode::Movement,
            file_tmp: String::new(),
            input_command: String::new(),
            output: String::from("Command output"),
            file_lines: 0,
            file_lines_vec: Vec::new(),
            command: Command::default(),
            force_quant: 0,
            coop: CoopCoding {
                members: vec![
                    CoopPerson {
                        color: 0,
                        id: 0,
                        name: if cfg!(windows) {
                            std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
                        } else {
                            std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
                        }
                    }
                ],
                possession: 0,
                enable: false,
                started: false
            },
            updated: 0
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyEvents {
    pub code: KeyCode,
    pub modifiers: KeyModifiers
}