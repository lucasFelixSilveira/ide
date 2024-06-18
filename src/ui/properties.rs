use crate::structs;
use structs::FOption;
use structs::Editor;
use structs::Mode;

use crate::utils;
use utils::terminal;
use terminal::get_size;

use crossterm::cursor::MoveTo;
use crossterm::execute;

use colored::*;

fn abs(i: i16) -> usize {
  if i >= 0 { i as usize }
  else { 0 }
}

pub fn assemble(editor: &mut Editor, files: Vec<FOption>) {

  let (width, height) = get_size();
  
  let selected: usize = editor.prop;
  let division: String = "-".repeat(usize::from(width));
  println!("Properties\n{division}");
  let real_size: usize = height as usize - if editor.mode == Mode::Listen { 5 } else { 3 };

  let mid: usize = (real_size - real_size % 2) / 2;
  let to_ignore: i16 = if selected > mid {
    (selected - mid) as i16
  } else { -1 };

  for (index, option) in files.iter().enumerate() {

    if to_ignore >= index as i16 && editor.file - to_ignore as usize > mid { continue; }
    if index - abs(to_ignore) == real_size {
      break;
    }  

    let mut name: String = option.label.clone();
    let prefix: char = option.symbol;

    if name.len() > usize::from(width - 2) {
      name = format!("{}...", name[0..(usize::from(width-5))].to_string());
    }

    let spaces = " ".repeat(width as usize - 1 - format!(" {} {}", prefix, name).len());
    let text: String = format!(" {} {}{spaces}", prefix, name);

    if selected == index {
      println!("{}", text.purple());
    } else {
      println!("{text}");
    }
  }
  
  if editor.mode == Mode::Listen {
    execute!(std::io::stdout(), MoveTo(0,height - 2)).expect("_");
    println!("{}\n{} {}{}", "=".repeat(width as usize), ">".purple(), editor.listen.1, " ".repeat(width as usize - 2 - editor.listen.1.len() - 1));
  }

  execute!(std::io::stdout(), MoveTo(0,0)).expect("_");
  println!("Properties{}\n{}", " ".repeat(usize::from(width) - "Properties".len() - 2), "-".repeat(usize::from(width)));
  execute!(std::io::stdout(), MoveTo(0,height)).expect("_");
}