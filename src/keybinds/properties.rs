use std::fs;

use crate::structs;
use crate::structs::FOption;
use crate::structs::LMemory;
use structs::Interface;
use structs::Editor;
use structs::Mode;
use structs::KeyEvents;
use structs::FEOption;

use crate::utils;
use utils::terminal;
use terminal::clear;

use crossterm::event::KeyCode;

pub fn valid(editor: &mut Editor, press: KeyEvents) {
  let options: Vec<FOption> = FOption::properties();
  match press.code {
    KeyCode::Down  | KeyCode::Char('s') if editor.prop != options.len() - 1 => editor.prop += 1,
    KeyCode::Up    | KeyCode::Char('w') if editor.prop != 0 => editor.prop -= 1,
    KeyCode::Enter | KeyCode::Char('e') => {
      let selected: FOption = options[editor.prop].clone();
      match selected.fun {
        FEOption::Delete if editor.files.len() > 0 && editor.files[editor.file].is_folder => {
          fs::remove_dir_all(editor.files[editor.file].clone().path).unwrap();
        }
        FEOption::Delete if editor.files.len() > 0 && !editor.files[editor.file].is_folder => {
          fs::remove_file(editor.files[editor.file].clone().path).unwrap();
        }
        FEOption::Back => {},
        FEOption::New => {
          editor.mode = Mode::Listen;
          editor.listen = (LMemory::NFile, String::new());
        }
        FEOption::NewF => {
          editor.mode = Mode::Listen;
          editor.listen = (LMemory::NFolder, String::new());
        }
        FEOption::Rename if editor.files.len() > 0 => {
          editor.mode = Mode::Listen;
          editor.listen = (LMemory::Rename, String::new());
        }
        _ => {}
      }

      clear();
      editor.interface = Interface::Files;
    }
    _ => {}
  }
}