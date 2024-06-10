use crate::structs;
use structs::Editor;
use structs::File;

use crate::utils;
use utils::terminal;
use terminal::get_size;

use colored::*;

pub fn assemble(editor: &mut Editor, files: Vec<File>) {

  let (width, height) = get_size();

  let selected: usize = editor.file;
  let division: String = "-".repeat(usize::from(width));
  println!("File explorer\n{division}");
  for (index, file) in files.iter().enumerate() {
    
    if editor.page_down > index { continue; }    
    if files.len() > usize::from(height) && (index - editor.page_down - 3) == usize::from(height) { break; } 
    
    let mut name: String = file.name.clone();
    let prefix: char = if file.is_folder { '+' } else { '-' };

    if name.len() > usize::from(width - 2) {
      name = format!("{}...", name[0..(usize::from(width-5))].to_string());
    }

    let text: String = format!(" {} {}", prefix, name);

    if selected == index {
      println!("{}", text.purple());
    } else {
      println!("{text}");
    }
  }
}