use colored::*;
use crossterm::style::Stylize;

fn keywords() -> Vec<String> {
  vec![
    String::from("sub"),
    String::from("use"),
    String::from("my"),
    String::from("our"),
    String::from("while"),
    String::from("for"),
    String::from("foreach"),
    String::from("else"),
    String::from("if"),
    String::from("elsif"),
    String::from("continue"),
    String::from("break"),
    String::from("return")
  ]
}

fn std_types() -> Vec<String> {
  vec![
    String::from("eq"),
    String::from("ne"),
    String::from("not"),
    String::from("xor"),
    String::from("or"),
    String::from("s"),
    String::from("$"),
    String::from("~")
  ]
}

fn special_keywords() -> Vec<String> {
  vec![
    String::from("shift"),
    String::from("unshift"),
    String::from("false"),
    String::from("true"),
    String::from("std"),
    String::from("@")
  ]
}

fn operators() -> Vec<String> {
  vec![
    String::from(":"),
    String::from(">"),
    String::from("<"),
    String::from("!"),
    String::from("&"),
    String::from("*"),
    String::from("/"),
    String::from("%"),
    String::from("#"),
    String::from("="),
    String::from("-"),
    String::from("+")
  ]
}

pub fn parse(line: String) -> String {
  let keywords: Vec<String> = keywords();
  let special_keywords: Vec<String> = special_keywords();
  let operators: Vec<String> = operators();
  let std_types: Vec<String> = std_types();
  let mut string_mode: bool = false;
  let mut double_q: bool = false;

  let mut final_string: String = String::new();

  let string: String = line.clone();
  let mut lexames: Vec<(usize, String)> = Vec::new();
  let mut lexame: String = String::new();
  let mut spaces: usize = 0;
  let mut ch: Vec<char> = string.chars().collect();
  ch.push(' ');

  if line.trim().to_string().starts_with("//") {
    return format!("{}", line.dark_grey());
  }

  for c in ch {
    if [ ' ', '[', ']', ',', '+', '-', '@', '$', '{', '}', ';', '(', ')', '.', ':', '!', '<', '>', '*', '/', '#', '%', '=', '&', '"', '\'' ].contains(&c) {
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

  for (current, lex) in lexames.iter().enumerate() {

    final_string.push_str(" ".repeat(lex.0).as_str());
    let lexame: String = lex.clone().1;

    if lexame.starts_with('\\') {
      final_string.push_str(&format!("{}", lexame.dark_red().bold()));
      continue;
    } 

    if 
      lexame.starts_with('"') && lexame.ends_with('"') ||
      (lexame.starts_with('/') && lexame.ends_with('/') && !double_q) ||
      (lexame.starts_with('\'') && lexame.ends_with('\'') && !double_q)
    {
      final_string.push_str(&format!("{}", lexame.clone().dark_yellow().bold()));
      string_mode = !string_mode;
      double_q = if !string_mode { false } else if lexame == "\x22" { true } else { false };  
      continue;
    } 

    if string_mode {
      final_string.push_str(&format!("{}", lexame.dark_yellow().bold()));
      continue;
    }

    if keywords.contains(&lexame.to_string()) {
      final_string.push_str(&format!("{}", lexame.red()));
      continue;
    } 

    if std_types.contains(&lexame.to_string()) {
      final_string.push_str(&format!("{}", lexame.green()));
      continue;
    } 

    if special_keywords.contains(&lexame.to_string()) {
      final_string.push_str(&format!("{}", lexame.purple().bold()));
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

    let x: String = lexame.clone();
    let y: String = lexame.to_lowercase();
    let mut z: String = String::new();

    for c in chars { 
      if c.is_alphanumeric() || c == '_' {
        z.push(c.clone());
      }
    }

    if x == y && y == z  {
      if (lexames.len() - 1) != current {
        if 
          lexames.clone()[current + 1].1 == "(".to_string() ||
          lexames.clone()[current + 1].1 == "!".to_string()
        {
          final_string.push_str(&format!("{}", lexame.yellow())); 
          continue;
        } 
      }

      final_string.push_str(&format!("{}", lexame.cyan())); 
      continue 
    }

    let chrs: &[u8] = lexame.as_bytes();
    let first: u8 = chrs[0];
    if first >= b'\x41' && first <= b'\x5a' {
      final_string.push_str(&format!("{}", lexame.green())); 
      continue 
    }

    // nothing
    final_string.push_str(&lexame);

  }

  final_string
}