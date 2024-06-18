use crate::structs;
use structs::Interface;
use structs::Editor;
use structs::KeyEvents;
use structs::Mode;

use crate::utils;
use utils::terminal;
use terminal::clear;

use std::path::PathBuf;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;

use colored::*;

pub fn valid(editor: &mut Editor, press: KeyEvents) {
  match press.modifiers {
    KeyModifiers::CONTROL => {
      match press.code {
        KeyCode::Char('s') => {
          let file_path: PathBuf = editor.files[editor.file].path.clone();
          std::fs::write(file_path, editor.content.clone()).unwrap();
          clear();
          println!("{}", "Saving...".yellow());
          std::thread::sleep(std::time::Duration::from_millis(1000));
          terminal::back_to_zero();
          println!("{}", "File saved!".green());
          std::thread::sleep(std::time::Duration::from_millis(300));
        }
        _ => {}
      }
    }
    KeyModifiers::ALT => {
      let lines: Vec<&str> = editor.content.lines().collect();
      match press.code {
        KeyCode::Down if editor.cursor.1 < editor.page_down + 1 && editor.page_down < lines.len() - 1 => {
          editor.page_down += 1;
          editor.cursor.1 = editor.page_down;
        }
        
        KeyCode::Up if editor.page_down != 0  => {
          editor.page_down -= 1;
          if editor.cursor.1 - editor.page_down > terminal::get_size().1 as usize - 4 {
            editor.cursor.1 -= 1;
          }
        }

        KeyCode::Char('s') => {
          let file_path: PathBuf = editor.files[editor.file].path.clone();
          std::fs::write(file_path, editor.content.clone()).unwrap();
          clear();
          println!("{}", "Saving...".yellow());
          std::thread::sleep(std::time::Duration::from_millis(1000));
          terminal::back_to_zero();
          println!("{}", "File saved!".green());
          std::thread::sleep(std::time::Duration::from_millis(300));
        }

        KeyCode::Down if editor.page_down < lines.len() - 1 => editor.page_down += 1,
        _ => {}
      }
    }
    _ => {
      match press.code {
        KeyCode::Char('l') if editor.mode == Mode::Movement => editor.interface = Interface::Files,
        KeyCode::Esc if editor.mode == Mode::Insert => editor.mode = Mode::Movement,
        
        KeyCode::Char('w') if editor.mode == Mode::Movement && (editor.cursor.1 - editor.page_down) != 0 => {
          let lines: Vec<&str> = editor.content.lines().collect();
          editor.cursor.1 -= 1;
          let line: String = lines[editor.cursor.1].to_string();
          if line.len() < editor.cursor.0 {
            editor.cursor.0 = line.len()
          }
        }

        KeyCode::Up if editor.mode == Mode::Insert && (editor.cursor.1 - editor.page_down) != 0 => {
          let lines: Vec<&str> = editor.content.lines().collect();
          editor.cursor.1 -= 1;
          let line: String = lines[editor.cursor.1].to_string();
          if line.len() < editor.cursor.0 {
            editor.cursor.0 = line.len()
          }
        }
        
        KeyCode::Char('s') if editor.mode == Mode::Movement && (editor.cursor.1 - editor.page_down) < terminal::get_size().1 as usize - 4 => {
          let lines: Vec<&str> = editor.content.lines().collect();
          if lines.len() > 0 && (lines.len() - 1) > editor.cursor.1 {
            editor.cursor.1 += 1;
            let line: String = lines[editor.cursor.1].to_string();
            if line.len() < editor.cursor.0 {
              editor.cursor.0 = line.len()
            }
          }
        }

        KeyCode::Down if editor.mode == Mode::Insert && (editor.cursor.1 - editor.page_down) < terminal::get_size().1 as usize - 4 => {
          let lines: Vec<&str> = editor.content.lines().collect();
          if lines.len() > 0 && (lines.len() - 1) > editor.cursor.1 {
            editor.cursor.1 += 1;
            let line: String = lines[editor.cursor.1].to_string();
            if line.len() < editor.cursor.0 {
              editor.cursor.0 = line.len()
            }
          }
        }

        KeyCode::Char('a') if editor.mode == Mode::Movement => {
          if editor.cursor.0 == 0 {
            if editor.cursor.1 == 0 {}
            else {
              editor.cursor.1 -= 1;
              let lines: Vec<&str> = editor.content.lines().collect();
              let line: String = lines[editor.cursor.1].to_string();
              editor.cursor.0 = line.len();
            }
          }
          else {
            editor.cursor.0 -= 1
          }
        }

        KeyCode::Left if editor.mode == Mode::Insert => {
          if editor.cursor.0 == 0 {
            if editor.cursor.1 == 0 {}
            else {
              editor.cursor.1 -= 1;
              let lines: Vec<&str> = editor.content.lines().collect();
              let line: String = lines[editor.cursor.1].to_string();
              editor.cursor.0 = line.len();
            }
          }
          else {
            editor.cursor.0 -= 1
          }
        }
        
        KeyCode::Char('d') if editor.mode == Mode::Movement && editor.content.len() > 0 => {
          let lines: Vec<&str> = editor.content.lines().collect();
          let line: String = lines[editor.cursor.1].to_string();
          if line.len() > editor.cursor.0 {
            editor.cursor.0 += 1
          } else if line.len() == editor.cursor.0 && (lines.len() - 1) > editor.cursor.1 && (editor.cursor.1 - editor.page_down) < terminal::get_size().1 as usize - 4 {
            editor.cursor.0 = 0;            
            editor.cursor.1 += 1;            
          }
        }

        KeyCode::Right if editor.mode == Mode::Insert && editor.content.len() > 0 => {
          let lines: Vec<&str> = editor.content.lines().collect();
          let line: String = lines[editor.cursor.1].to_string();
          if line.len() > editor.cursor.0 {
            editor.cursor.0 += 1
          } else if line.len() == editor.cursor.0 && (lines.len() - 1) > editor.cursor.1 && (editor.cursor.1 - editor.page_down) < terminal::get_size().1 as usize - 4 {
            editor.cursor.0 = 0;            
            editor.cursor.1 += 1;            
          }
        }

        KeyCode::Char('i') if editor.mode == Mode::Movement => editor.mode = Mode::Insert,

        KeyCode::Char(c) if editor.mode == Mode::Insert => {

          if editor.content.len() == 0 {
            editor.content = format!("{}", c);
            editor.cursor.0 += 1;
          } else {

            let mut new_content = String::new();
            let lines: Vec<&str> = editor.content.lines().collect();

            for (index, line) in lines.iter().enumerate() {
              if index == editor.cursor.1 {
                if line.is_empty() {
                  new_content.push(c);
                } else {
                  if editor.cursor.0 >= line.len() {
                    new_content.push_str(line);
                    new_content.push(c);
                  } else if editor.cursor.0 == 0 {
                    new_content.push(c);
                    new_content.push_str(line);
                  } else {
                    new_content.push_str(&line[..editor.cursor.0]);
                    new_content.push(c);
                    new_content.push_str(&line[editor.cursor.0..]);
                  }
                }
                editor.cursor.0 += 1;
              } else {
                new_content.push_str(line);
              }
              if index != lines.len() - 1 {
                new_content.push('\n');
              }
            }
            editor.content = new_content;

          }
        }

        KeyCode::Tab if editor.mode == Mode::Insert => {
          let mut new_content = String::new();
          let lines: Vec<&str> = editor.content.lines().collect();

          for (index, line) in lines.iter().enumerate() {
            if index == editor.cursor.1 {
              if line.is_empty() {
                new_content.push_str("  ");
              } else {
                if editor.cursor.0 >= line.len() {
                  new_content.push_str(line);
                  new_content.push_str("  ");
                } else if editor.cursor.0 == 0 {
                  new_content.push_str("  ");
                  new_content.push_str(line);
                } else {
                  new_content.push_str(&line[..editor.cursor.0]);
                  new_content.push_str("  ");
                  new_content.push_str(&line[editor.cursor.0..]);
                }
              }
              editor.cursor.0 += 1;
            } else {
              new_content.push_str(line);
            }
            if index != lines.len() - 1 {
              new_content.push('\n');
            }
          }
          editor.content = new_content;
        }

        KeyCode::Backspace if editor.mode == Mode::Insert => {
          let li: Vec<&str> = editor.content.lines().collect();
          let mut lines: Vec<String> = li.iter().map(|x| x.to_string()).collect();
          
          if editor.cursor.0 == 0 && editor.cursor.1 > 0 && lines.len() > 1 {
            let current_line = lines[editor.cursor.1].clone();
            let prev_line = lines[editor.cursor.1 - 1].clone();
            lines.remove(editor.cursor.1);
            editor.cursor.1 -= 1;
            editor.cursor.0 = prev_line.len();
            lines[editor.cursor.1] = format!("{}{}", prev_line, current_line);
          }
          else if editor.cursor.0 > 0 {
            let mut current_line = lines[editor.cursor.1].to_string();
            let index = editor.cursor.0 - 1;
            current_line.remove(index);
            editor.cursor.0 -= 1;
            lines[editor.cursor.1] = current_line;
          }
          else if editor.cursor.0 == 0 && editor.cursor.1 == 0 {}
          let result: String = lines.join("\n");
          editor.content = result;
        }

        KeyCode::Enter if editor.mode == Mode::Insert => {
          let li: Vec<&str> = editor.content.lines().collect();
          let mut lines: Vec<String> = li.iter().map(|x| x.to_string()).collect();
          
          if lines.len() > 0 {

            if editor.cursor.1 == lines.len() - 1 {
              lines.push(String::from("\r"));
              editor.cursor.0 = 0;
              editor.cursor.1 += 1;
            } else {
              let current_line = if editor.cursor.1 >= lines.len() { String::new() } else { lines[editor.cursor.1].clone() };
              let (before_cursor, after_cursor) = current_line.split_at(editor.cursor.0);
              while editor.cursor.1 >= lines.len() {
                lines.push("".to_string());
              }
              lines[editor.cursor.1] = before_cursor.to_string();
              lines.insert(editor.cursor.1 + 1, after_cursor.to_string());
              editor.cursor.0 = 0;
              editor.cursor.1 += 1;
            }
    
            let result = lines.join("\n");
            editor.content = result;
          
          }

        }

        _ => {}
      }
    }
  }
}