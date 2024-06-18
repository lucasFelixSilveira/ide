use crate::structs;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use structs::Editor;
use structs::File;

use crate::utils;
use utils::terminal;
use terminal::get_size;

use colored::*;

pub fn assemble(editor: &mut Editor) {
  let file: File = editor.files[editor.file].clone();
  let name: String = file.name;
  
  let (width, height) = get_size();

  let (x, y) = editor.cursor;

  let local: String = format!("col: {} line: {} scrolled: {}", (x+1), (y+1), editor.page_down);

  let division: String = "-".repeat((width as usize) - local.len() - 3);
  println!("Editing '{name}' [{}] in {} mode.                    \n{division} {local}", editor.file, format!(" {:?} ", editor.mode).black().on_purple());
  let spaces: String = " ".repeat((width as usize)-1);
  print!("{}", format!("~{spaces}\n").repeat(usize::from(height-3)).purple());
  execute!(std::io::stdout(), MoveTo(0, 2)).expect("_");

  let mut code: String = String::new();
  let mut lines: Vec<&str> = editor.content.lines().collect();
  let mut i: usize = 0;
  while i < editor.page_down {
    lines.remove(0);
    i += 1;
  }
  
  i = 0;
  while i < (height-3) as usize && i < lines.len() {
    code.push_str(&format!("{}\n", lines[i]));
    i += 1;
  }

  println!("{}", code.trim_end());
  execute!(std::io::stdout(), MoveTo(x as u16, 2 + (y as u16) - (editor.page_down as u16))).expect("_");
}
