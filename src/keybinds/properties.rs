use crate::structs;
use crate::structs::FOption;
use crate::structs::LMemory;
use colored::Colorize;
use structs::Interface;
use structs::Editor;
use structs::Mode;
use structs::KeyEvents;
use structs::FEOption;

use crate::utils;
use utils::terminal;
use terminal::clear;

use crossterm::event::KeyCode;

use colored::*;

pub fn valid(editor: &mut Editor, press: KeyEvents) {
  let options: Vec<FOption> = FOption::properties(editor);
  match press.code {
    KeyCode::Down  | KeyCode::Char('s') if editor.prop != options.len() - 1 => editor.prop += 1,
    KeyCode::Up    | KeyCode::Char('w') if editor.prop != 0 => editor.prop -= 1,
    KeyCode::Enter | KeyCode::Char('e') => {
      let selected: FOption = options[editor.prop].clone();
      match selected.fun {
        FEOption::Delete if editor.files.len() > 0 && editor.files[editor.file].is_folder => {
          editor.mode = Mode::Listen;
          editor.listen = (format!("Do you want delete the '{}' folder? {}", editor.files[editor.file].name, " [yes/ALT+X] ".on_yellow().black()), LMemory::DeleteFolder, String::new());
        }
        FEOption::Delete if editor.files.len() > 0 && !editor.files[editor.file].is_folder => {
          editor.mode = Mode::Listen;
          editor.listen = (format!("Do you want delete the '{}' file? {}", editor.files[editor.file].name, " [yes/ALT+X] ".on_yellow().black()), LMemory::DeleteFile, String::new());
        }
        FEOption::Back => {},
        FEOption::New => {
          editor.mode = Mode::Listen;
          editor.listen = (String::from("Name"), LMemory::NFile, String::new());
        }
        FEOption::NewF => {
          editor.mode = Mode::Listen;
          editor.listen = (String::from("Name"), LMemory::NFolder, String::new());
        }
        FEOption::Rename if editor.files.len() > 0 => {
          editor.mode = Mode::Listen;
          editor.listen = (String::from("New name"), LMemory::Rename, String::new());
        }
        _ => {}
      }

      clear();
      editor.interface = Interface::Files;
    }
    _ => {}
  }
}