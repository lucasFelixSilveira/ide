use colored::*;
use crossterm::style::Stylize;

fn keywords() -> Vec<String> {
  vec![
    String::from("case"),
    String::from("while"),
    String::from("for"),
    String::from("do"),
    String::from("typeof"),
    String::from("struct"),
    String::from("enum"),
    String::from("include"),
    String::from("define"),
    String::from("mod"),
    String::from("else"),
    String::from("if"),
    String::from("union"),
    String::from("continue"),
    String::from("break"),
    String::from("swtich"),
    String::from("pragma"),
    String::from("ifdef"),
    String::from("elsif"),
    String::from("endif"),
    String::from("ifndef"),
    String::from("volatile"),
    String::from("not"),
    String::from("and"),
    String::from("xor"),
    String::from("return"),
    String::from("defined")    
  ]
}

fn std_types() -> Vec<String> {
  vec![
    String::from("char"),
    String::from("short"),
    String::from("int"),
    String::from("long"),
    String::from("unsigned"),
    String::from("void")
  ]
}

fn special_keywords() -> Vec<String> {
  vec![
    String::from("static"),
    String::from("const"),
    String::from("goto"),
    String::from("asm"),
    String::from("#")
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
  let mut mac_mode: bool = false;

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
    if [ ' ', '[', ']', ',', '{', '}', ';', '(', ')', '.', ':', '!', '<', '>', '*', '/', '#', '%', '=', '&', '"', '\'' ].contains(&c) {
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
      !string_mode &&
      (
        lexame.starts_with('<') && lexame.ends_with('<') && line.clone().trim().starts_with('#') ||
        lexame.starts_with('>') && lexame.ends_with('>') && line.clone().trim().starts_with('#') 
      )
    {
      final_string.push_str(&format!("{}", lexame.bright_green().bold()));
      mac_mode = !mac_mode;
      continue;
    }

    if mac_mode {
      final_string.push_str(&format!("{}", lexame.bright_green().bold()));
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
    let mut z: String = String::new();

    for c in chars { 
      if c.is_alphanumeric() || c == '_' {
        z.push(c.clone());
      }
    }

    if x == z  {
      if (lexames.len() - 1) != current {
        if lexames.clone()[current + 1].1 == "(".to_string() {
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