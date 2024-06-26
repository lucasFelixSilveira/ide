use colored::*;
use crossterm::style::Stylize;

fn keywords() -> Vec<String> {
  vec![
    String::from("fn"),
    String::from("pub"),
    String::from("let"),
    String::from("loop"),
    String::from("while"),
    String::from("for"),
    String::from("do"),
    String::from("impl"),
    String::from("struct"),
    String::from("enum"),
    String::from("use"),
    String::from("crate"),
    String::from("mod"),
    String::from("else"),
    String::from("if"),
    String::from("in"),
    String::from("union"),
    String::from("continue"),
    String::from("break"),
    String::from("match"), 
    String::from("return") 
  ]
}

fn std_types() -> Vec<String> {
  vec![
    String::from("u8"),
    String::from("u16"),
    String::from("u32"),
    String::from("u64"),
    String::from("u128"),
    String::from("i8"),
    String::from("i16"),
    String::from("i32"),
    String::from("i64"),
    String::from("i128"),
    String::from("bool"),
    String::from("Option"),
    String::from("Result"),
    String::from("Vec"),
    String::from("String"),
    String::from("str"),
    String::from("char"),
    String::from("usize"),
    String::from("f32"),
    String::from("f64")
  ]
}

fn special_keywords() -> Vec<String> {
  vec![
    String::from("mut"),
    String::from("static"),
    String::from("const"),
    String::from("self"),
    String::from("false"),
    String::from("true"),
    String::from("std")
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
    if [ ' ', '[', ']', ',', '+', '-', '{', '}', ';', '(', ')', '.', ':', '!', '<', '>', '*', '/', '#', '%', '=', '&', '"', '\'' ].contains(&c) {
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
      lexame.starts_with('\'') && lexame.ends_with('\'')
    {
      final_string.push_str(&format!("{}", lexame.dark_yellow().bold()));
      string_mode = !string_mode;
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