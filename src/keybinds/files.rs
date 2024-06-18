use crate::structs;
use structs::Interface;
use structs::Editor;
use structs::KeyEvents;
use structs::Mode;
use structs::File;
use structs::LMemory;

use crate::utils;
use utils::terminal;
use terminal::get_size;
use terminal::clear;

use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::cursor::MoveTo;
use crossterm::execute;

use crate::keybinds;
use keybinds::modules;
use modules::properties;

use colored::*;

pub fn valid(editor: &mut Editor, press: KeyEvents) {
  let (_, height) = get_size();

  match editor.mode {
    Mode::Listen => {
      match press.modifiers {
        KeyModifiers::ALT if press.code == KeyCode::Char('x') => {
          editor.mode = Mode::Movement;
          editor.listen = (LMemory::Unknown, String::new());
          clear();
        }
        _ => match press.code {
          KeyCode::Char(c) => {
            editor.listen.1.push(c);
          }
          KeyCode::Backspace if editor.listen.1.len() > 0 => {
            editor.listen.1.remove(editor.listen.1.len()-1);
            
            execute!(std::io::stdout(), MoveTo(( 2 + editor.listen.1.len() ) as u16,height-2)).expect("_");
            print!(" ");
          },
          KeyCode::Enter => {
            properties::valid(editor);
            editor.mode = Mode::Movement;
            editor.listen = (LMemory::Unknown, String::new());
            clear();
          }
          _ => {}
        }
      }
    }
    _ => match press.code {
      KeyCode::Down  | KeyCode::Char('s') if editor.file != editor.files.len() - 1 => editor.file += 1,
      KeyCode::Up    | KeyCode::Char('w') if editor.file != 0 => editor.file -= 1,
      KeyCode::Enter | KeyCode::Char('e') => {
        let file: File = editor.files[editor.file].clone();
        match file.is_folder {
          false => {
            editor.content = std::fs::read_to_string(file.path).unwrap();
            editor.interface = Interface::Editor;
            editor.cursor = (0,0);
            clear();
          }
          true => {
            clear();
            let name: String = file.name.clone();
            std::env::set_current_dir(format!("{}{}{name}", std::env::current_dir().unwrap().display(), std::path::MAIN_SEPARATOR)).unwrap();
            editor.file = 0;
          }
        }
      }
      KeyCode::Backspace | KeyCode::Char('q') => {
        clear();
        std::env::set_current_dir(
          format!("{}", std::env::current_dir().unwrap().display()).rsplit_once(std::path::MAIN_SEPARATOR).unwrap().0  
        ).unwrap();
        editor.file = 0;
      }
      KeyCode::Char('x') => {
        clear();      
        println!("{}", "Process killed.".red());
        editor.quit = true
      }
      KeyCode::Char('f') => {
        clear();     
        editor.interface = Interface::Properties; 
        editor.prop = 0;
      }
      _ => {}
    }
  }
}