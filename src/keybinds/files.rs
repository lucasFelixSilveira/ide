use crate::structs;
use structs::Interface;
use structs::Editor;
use structs::KeyEvents;
use structs::File;

use crate::utils;
use utils::terminal;
use terminal::clear;

use crossterm::event::KeyCode;

use colored::*;

pub fn valid(editor: &mut Editor, press: KeyEvents) {
  match press.code {
    KeyCode::Down | KeyCode::Char('s') if editor.file != editor.files.len() - 1 => editor.file += 1,
    KeyCode::Up | KeyCode::Char('w') if editor.file != 0 => editor.file -= 1,
    KeyCode::Enter | KeyCode::Char('e') => {
      let file: File = editor.files[editor.file].clone();
      match file.is_folder {
        false => {
          editor.content = std::fs::read_to_string(file.path).unwrap();
          editor.interface = Interface::Editor;
        }
        true => {
          clear();
          let name: String = file.name.clone();
          std::env::set_current_dir(format!("{}{}{name}", std::env::current_dir().unwrap().display(), std::path::MAIN_SEPARATOR)).unwrap();
        }
      }
    }
    KeyCode::Backspace | KeyCode::Char('q') => {
      clear();
      std::env::set_current_dir(
        format!("{}", std::env::current_dir().unwrap().display()).rsplit_once(std::path::MAIN_SEPARATOR).unwrap().0  
      ).unwrap();
    }
    KeyCode::Char('x') => {
      clear();      
      println!("{}", "Process killed.".red());
      editor.quit = true
    }
    _ => {}
  }
}