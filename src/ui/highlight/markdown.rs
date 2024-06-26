use colored::*;
use crossterm::style::Stylize;

fn operators() -> Vec<String> {
  vec![
    String::from(":"),
    String::from(">"),
    String::from("!"),
    String::from("*"),
    String::from(".")
  ]
}

pub fn parse(line: String) -> String {
  let operators: Vec<String> = operators();
  let mut string_mode: bool = false;
  let mut bg_mode: bool = false;

  let mut final_string: String = String::new();

  let string: String = line.clone();
  let mut lexames: Vec<(usize, String)> = Vec::new();
  let mut lexame: String = String::new();
  let mut spaces: usize = 0;
  let mut ch: Vec<char> = string.chars().collect();
  ch.push(' ');

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

    if lexame.starts_with('`') && lexame.ends_with('`') {
      final_string.push_str(&format!("{}", lexame.on_dark_yellow().black().bold()));
      bg_mode = !bg_mode;
      continue;
    } 

    if bg_mode {
      final_string.push_str(&format!("{}", lexame.on_dark_yellow().black().bold()));
      continue;
    }

    if line.starts_with('#') {
      final_string.push_str(&format!("{}", lexame.red().bold()));
      continue;
    } 

    if line.starts_with('>') {
      final_string.push_str(&format!("{}", line.on_dark_grey().bold()));
      break;
    } 

    

    if line.starts_with('-') {
      final_string.push_str(&format!("{}", lexame.cyan().bold()));
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

    if operators.contains(&lexame.to_string()) {
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

    let mut z: String = String::new();

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