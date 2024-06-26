use colored::*;
use crossterm::style::Stylize;

fn brackets() -> Vec<String> {
  vec![
    String::from("{"),
    String::from("}"),
    String::from("("),
    String::from(")")
  ]
}

pub fn parse(line: String) -> String {
  let brackets: Vec<String> = brackets();
  let mut string_mode: bool = false;
  let mut bg_mode: bool = false;

  let mut final_string: String = String::new();

  let string: String = line.clone();
  let mut lexames: Vec<(usize, String)> = Vec::new();
  let mut lexame: String = String::new();
  let mut spaces: usize = 0;
  let mut ch: Vec<char> = string.chars().collect();
  ch.push(' ');

  if line.trim().to_string().starts_with("#") {
    return format!("{}", line.dark_grey());
  }

  for c in ch {
    if [ ' ', '[', ']', '`', ',', '+', '-', '{', '}', ';', '(', ')', '.', ':', '!', '<', '>', '*', '/', '#', '%', '=', '&', '"', '\'' ].contains(&c) {
      if !lexame.is_empty() {
        lexames.push((spaces, lexame.clone())); 
        lexame.clear();
        spaces = 0;
      } 
      if c != ' ' && lexame.is_empty() {
        lexames.push((spaces, String::from(c)));
        spaces = 0;
      }
      if c == ' ' {
        spaces += 1;
      }
    }
    else {
      lexame.push(c);
    }
  }

  for lex in lexames {

    final_string.push_str(&format!("{}", if bg_mode { format!("{}", " ".repeat(lex.0).as_str().on_dark_yellow()) } else { " ".repeat(lex.0).to_string() }));
    let lexame: String = lex.clone().1;

    if 
      lexame.starts_with('[') && lexame.ends_with('[') || 
      lexame.starts_with(']') && lexame.ends_with(']') 
    {
      final_string.push_str(&format!("{}", lexame.clone().on_dark_yellow().black().bold()));
      bg_mode = if lexame.clone().starts_with('[') && !bg_mode { !bg_mode } else if lexame.starts_with('[') { bg_mode } else if !bg_mode { bg_mode } else { !bg_mode };
      continue;
    } 

    if bg_mode {
      final_string.push_str(&format!("{}", lexame.on_dark_yellow().black().bold()));
      continue;
    }

    if lexame == "=" {
      final_string.push_str(&format!("{}", lexame.red().bold()));
      continue;
    } 

    if lexame.starts_with('"') && lexame.ends_with('"') {
      final_string.push_str(&format!("{}", lexame.dark_yellow().bold()));
      string_mode = !string_mode;
      continue;
    } 

    
    if string_mode {
      final_string.push_str(&format!("{}", lexame.dark_yellow().bold()));
      continue;
    }

    if brackets.contains(&lexame.to_string()) {
      final_string.push_str(&format!("{}", lexame.red()));
      continue;
    } 

    

    // identifiers
    let chars: Vec<char> = lexame.chars().collect();

    if chars[0].is_numeric() {
      let mut ch: Vec<char> = Vec::new();
      for c in chars.clone() {
        if c.is_numeric() {
          ch.push(c);
        }
      }
      if ch.len() == chars.len() {
        final_string.push_str(&format!("{}", lexame.blue())); 
        continue;
      }
    }

   

    let x: String = lexame.clone();
    let y: String = lexame.to_lowercase();
    let mut z: String = String::new();

    if x == y  {
      final_string.push_str(&format!("{}", lexame.cyan())); 
      continue 
    }

    for c in chars { 
      if c.is_alphanumeric() || c == '_' {
        z.push(c.clone());
      }
    }

    // nothing
    final_string.push_str(&lexame);

  }

  final_string
}