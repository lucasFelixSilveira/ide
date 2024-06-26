use crate::ui;
use ui::highlight;

use crate::structs;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use structs::Editor;
use structs::File;
use structs::Mode;

use crate::utils;
use utils::terminal;
use terminal::get_size;

use colored::*;

pub fn assemble(editor: &mut Editor) {
  let file: File = editor.files[editor.file].clone();
  let name: String = file.clone().name;
  
  let (width, height) = get_size();

  let (x, y) = editor.cursor;

  let local: String = format!("col: {} line: {} scrolled: {}", (x+1), (y+1), editor.page_down);

  let division: String = "-".repeat((width as usize) - local.len() - 2);
  let header_start: String = format!("Editing '{name}' in {} mode.", format!(" {:?} ", editor.mode).black().on_purple());
  
  let help: String = if editor.mode == Mode::Insert {
    String::from("(Escape - Movement mode) (M-S - Save file)")
  } else if editor.mode == Mode::Movement {
    String::from("(I - Insert mode) (L - Leave file) (M-S - Save file) (C or Z - Scroll)")
  } else if editor.mode == Mode::Selection {
    String::from("(Arrows - Move cursor) (E - Back to insert) (O - Overwrite)")
  } else { String::new() };

  let spaces_to_help: String = " ".repeat((width as usize) - header_start.len());
  println!("{header_start}{spaces_to_help}\n{division} {local}");
  execute!(std::io::stdout(), MoveTo(width - help.len() as u16 - 1, 0)).unwrap();
  print!("{}", help.bold());
  execute!(std::io::stdout(), MoveTo(0,2)).unwrap();

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
    code.push_str(&format!("{}\n", highlight::language(lines[i].to_string(), file.name.clone()  )));
    i += 1;
  }

  println!("{}", code.trim_end());
  execute!(std::io::stdout(), MoveTo(x as u16, 2 + (y as u16) - (editor.page_down as u16))).expect("_");
}
