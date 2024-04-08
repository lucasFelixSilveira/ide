use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crate::structs;
use crate::utils;
use crate::structs::Interface;
use utils::terminal;
use structs::KeyEvents;
use structs::Editor;
use structs::Mode;
use std::fs;

pub fn binds(event: KeyEvents, editor: &mut Editor) {
    match event.modifiers {
        KeyModifiers::CONTROL if editor.mode != Mode::Command => {
            match event.code {
                KeyCode::Char('s') => {
                    terminal::clear();
                    println!("Saving...");
                    std::thread::sleep(std::time::Duration::from_millis(400));
                    fs::write(&editor.files[editor.cursor.file].path, fs::read_to_string(&editor.file_tmp).unwrap()).expect("Fail to write file.");
                }
                _ => {}
            }
        }
        KeyModifiers::ALT if editor.mode != Mode::Command => {
            match event.code {
                KeyCode::Char('x') => {
                    editor.before_mode = editor.mode.clone();
                    editor.mode = Mode::Command;
                    editor.cursor.z = 0;
                    editor.command = String::new();
                }
                _ => {}
            }
        }
        KeyModifiers::CONTROL if editor.mode == Mode::Command => {
            match event.code {
                KeyCode::Char('g') => {
                    editor.mode = editor.before_mode;
                    editor.cursor.z = 0;
                    editor.command = String::new();
                }
                _ => {}
            }
        }
        _ => {
            match editor.mode {
                Mode::Movement => {
                    match event.code {
                        KeyCode::Down  | KeyCode::Char('s') | KeyCode::Char('k') if editor.file_lines - 1 > editor.cursor.y => editor.cursor.y += 1,
                        KeyCode::Up    | KeyCode::Char('w') | KeyCode::Char('j') if editor.cursor.y > 0 => editor.cursor.y -= 1,
                        KeyCode::Left  | KeyCode::Char('a') | KeyCode::Char('l') | KeyCode::Char('f') if editor.cursor.x > 0 => editor.cursor.x -= 1,
                        KeyCode::Left  | KeyCode::Char('a') | KeyCode::Char('l') | KeyCode::Char('f') if editor.cursor.x == 0 && editor.cursor.y != 0 => { editor.cursor.x = editor.file_lines_vec[editor.cursor.y-1].len(); editor.cursor.y -= 1; },
                        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('h') | KeyCode::Char('b') => editor.cursor.x += 1,
                        KeyCode::Char('i') => editor.mode = Mode::Insert,
                        KeyCode::Esc => {
                            editor.interface = Interface::Files;
                            fs::remove_file(&editor.file_tmp).unwrap();
                        }
                        _ => {}
                    }
                }
                Mode::Insert => {
                    match event.code {
                        KeyCode::Down if editor.file_lines - 1 > editor.cursor.y => {
                            editor.cursor.y += 1;
                            if editor.file_lines_vec[editor.cursor.y].len() < editor.cursor.x {
                                editor.cursor.x = editor.file_lines_vec[editor.cursor.y].len();
                            }
                        }
                        KeyCode::Up if editor.cursor.y > 0 => {
                            editor.cursor.y -= 1;
                            if editor.file_lines_vec[editor.cursor.y].len() < editor.cursor.x {
                                editor.cursor.x = editor.file_lines_vec[editor.cursor.y].len();
                            }
                        }
                        KeyCode::Left  if editor.cursor.x > 0 => editor.cursor.x -= 1,
                        KeyCode::Left  if editor.cursor.x == 0 && editor.cursor.y != 0 => { editor.cursor.x = editor.file_lines_vec[editor.cursor.y-1].len(); editor.cursor.y -= 1; },
                        KeyCode::Right => {
                            editor.cursor.x += 1;
                            if editor.cursor.x == editor.file_lines_vec[editor.cursor.y].len() && editor.file_lines - 1 > editor.cursor.y {
                                editor.cursor.y += 1;
                                editor.cursor.x = 0;
                            }
                        }
                        KeyCode::Char(c) => {
                            let mut new_content = String::new();
                    
                            for (index, line) in editor.file_lines_vec.iter().enumerate() {
                                if index == editor.cursor.y {
                                    if line.is_empty() {
                                        new_content.push(c);
                                    } else {
                                        if editor.cursor.x >= line.len() {
                                            new_content.push_str(line);
                                            new_content.push(c);
                                        } else if editor.cursor.x == 0 {
                                            new_content.push(c);
                                            new_content.push_str(line);
                                        } else {
                                            new_content.push_str(&line[..editor.cursor.x]);
                                            new_content.push(c);
                                            new_content.push_str(&line[editor.cursor.x..]);
                                        }
                                    }
                                    editor.cursor.x += 1;
                                } else {
                                    new_content.push_str(line);
                                }
                                if index != editor.file_lines_vec.len() - 1 {
                                    new_content.push('\n');
                                }
                            }
                    
                            editor.file_lines_vec = new_content.lines().map(|x| x.to_string()).collect();
                            fs::write(&editor.file_tmp, new_content).expect("Failed to write new file");
                        }
                        
                        KeyCode::Backspace => {
                            if editor.cursor.x == 0 && editor.cursor.y > 0 {
                                let current_line = editor.file_lines_vec[editor.cursor.y].clone();
                                let prev_line = editor.file_lines_vec[editor.cursor.y - 1].clone();
                                editor.file_lines_vec.remove(editor.cursor.y);
                                editor.cursor.y -= 1;
                                editor.cursor.x = prev_line.len();
                                editor.file_lines_vec[editor.cursor.y] = format!("{}{}", prev_line, current_line);
                            }
                            else if editor.cursor.x > 0 {
                                let current_line = &mut editor.file_lines_vec[editor.cursor.y];
                                let index = editor.cursor.x - 1;
                                current_line.remove(index);
                                editor.cursor.x -= 1;
                            }
                            else if editor.cursor.x == 0 && editor.cursor.y == 0 {}
                            let result = editor.file_lines_vec.join("\n");
                            fs::write(&editor.file_tmp, result).expect("Failed to write new file");
                        }
                        KeyCode::Enter => {
                            if editor.cursor.y == editor.file_lines_vec.len() - 1 {
                                editor.file_lines_vec.push(String::from("\r"));
                                editor.file_lines += 1;
                                editor.cursor.x = 0;
                                editor.cursor.y += 1;
                            } else {
                                let current_line = editor.file_lines_vec[editor.cursor.y].clone();
                                let (before_cursor, after_cursor) = current_line.split_at(editor.cursor.x);
                                editor.file_lines_vec[editor.cursor.y] = before_cursor.to_string();
                                editor.file_lines_vec.insert(editor.cursor.y + 1, after_cursor.to_string());
                                editor.file_lines += 1;
                                editor.cursor.x = 0;
                                editor.cursor.y += 1;
                            }
                    
                            let result = editor.file_lines_vec.join("\n");
                            fs::write(&editor.file_tmp, result).expect("Failed to write new file");
                        }
                        KeyCode::Esc => editor.mode = Mode::Movement,
                        _ => {}
                    }
                }
                Mode::Command => {
                    match event.code {
                        KeyCode::Char(c) => {
                            if editor.cursor.z >= editor.command.len() {
                                editor.command.push(c);
                            } else {
                                editor.command.insert(editor.cursor.z, c);
                            }
                            editor.cursor.z += 1;
                        }
                        KeyCode::Left  if editor.cursor.z > 0 => editor.cursor.z -= 1,
                        KeyCode::Right if editor.command.len() > editor.cursor.z => editor.cursor.z += 1,
                        KeyCode::Backspace => {
                            if editor.cursor.z > 0 {
                                editor.command.remove(editor.cursor.z - 1);
                                editor.cursor.z -= 1;
                            }
                        }
                        KeyCode::Enter => {
                            let reset = |editor: &mut Editor| { editor.command = String::new(); editor.cursor.z = 0; };
                            match editor.command.as_str() {
                                "hi" => {
                                    reset(editor);
                                    editor.output = "Hello, world!".to_string();
                                }
                                _ => editor.output = "Invalid command".to_string()
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}