use crate::structs;
use structs::Editor;
use structs::LMemory;

use crate::utils;
use utils::terminal;
use terminal::clear;

use colored::*;

pub fn valid(editor: &mut Editor) {
  match editor.listen.0 {
    LMemory::NFile => {
      std::fs::write(
        format!(
          "{}{}{}", 
          // arguments
            std::env::current_dir()
              .unwrap()
              .display(), 
            std::path::MAIN_SEPARATOR,
            editor.listen.1
        ), String::new()
      ).unwrap();
      
      clear();
      println!("{}", "Created file.".green());
      std::thread::sleep(std::time::Duration::new(1, 0));
      clear();
    }
    LMemory::NFolder => {
      std::fs::create_dir(
        format!(
          "{}{}{}", 
          // arguments
            std::env::current_dir()
              .unwrap()
              .display(), 
            std::path::MAIN_SEPARATOR,
            editor.listen.1
        )
      ).unwrap();
      
      clear();
      println!("{}", "Created folder.".green());
      std::thread::sleep(std::time::Duration::new(1, 0));
      clear();
    },
    LMemory::Rename => {
      std::fs::rename(
          format!(
          "{}{}{}", 
          // arguments
            std::env::current_dir()
              .unwrap()
              .display(), 
            std::path::MAIN_SEPARATOR,
            editor.files[editor.file].name
        ), 
        format!(
          "{}{}{}", 
          // arguments
            std::env::current_dir()
              .unwrap()
              .display(), 
            std::path::MAIN_SEPARATOR,
            editor.listen.1
        )
      ).unwrap()
    }
    _ => {}
  }
}