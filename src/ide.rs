use core::time;
use std::io;
use std::fs;
use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::str::LinesAny;
use std::thread;
use std::vec;
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};
use colored::*;

enum Interface {
    Files,
    Editor,
    // LiveShare
}

struct KeyEvents {
    code: KeyCode,
    modifiers: KeyModifiers
}

#[derive(Debug)]
struct LocalDirectory {
    is_folder: bool,
    extension: Option<String>,
    name: String,
    path: PathBuf, // Novo campo para o caminho completo do arquivo
}

fn read_local_directory() -> Vec<LocalDirectory> {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Erro ao obter o diretório atual.");
            std::process::exit(1);
        }
    };

    if let Ok(entries) = fs::read_dir(&current_dir) {
        let mut result = Vec::new();

        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type();
                let file_name = entry.file_name();
                let path = entry.path();

                let is_folder = file_type.map_or(false, |t| t.is_dir());
                let extension = file_name
                    .to_string_lossy()
                    .to_string()
                    .rsplit('.')
                    .next()
                    .map(String::from);

                let name = file_name.to_string_lossy().to_string();

                result.push(LocalDirectory {
                    is_folder,
                    extension,
                    name,
                    path,
                });
            }
        }

        result
    } else {
        eprintln!("Erro ao ler o diretório atual.");
        std::process::exit(1);
    }
}

fn event_listenner() -> KeyEvents {
    let result: KeyEvents;
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        if let Ok(event) = event::read() {
            match event {
                event::Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind,
                    state: _,
                }) => {
                    result = KeyEvents { code, modifiers };
                    break;
                }
                _ => (),
            }
        }
    }
    result
}

fn get_terminal_size_windows() -> (usize, usize) {
    if let Ok(output) = Command::new("cmd").args(&["/C", "mode", "CON"]).output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            if let Some((width, height)) = parse_mode_output(&stdout) {
                return (width, height);
            }
        }
    }

    (80, 24)
}

fn get_terminal_size_linux() -> (usize, usize) {
    if let Ok(output) = Command::new("sh").args(&["-c", "stty size"]).output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            if let Some((width, height)) = parse_stty_output(&stdout) {
                return (width, height);
            }
        }
    }

    (80, 24)
}

fn parse_mode_output(output: &str) -> Option<(usize, usize)> {
    let lines: Vec<&str> = output.lines().collect();

    if lines.len() >= 4 {
        let width_str = lines[3].split_whitespace().nth(1)?;
        let height_str = lines[4].split_whitespace().nth(1)?;

        if let (Ok(width), Ok(height)) = (width_str.parse(), height_str.parse()) {
            return Some((width, height));
        }
    }

    None
}

fn parse_stty_output(output: &str) -> Option<(usize, usize)> {
    let dimensions: Vec<&str> = output.split_whitespace().collect();

    if dimensions.len() == 2 {
        if let (Ok(width), Ok(height)) = (dimensions[0].parse(), dimensions[1].parse()) {
            return Some((width, height));
        }
    }

    None
}

fn update_file(directory: &mut Vec<String>, selection: &mut usize) {

    let is_windows: bool = cfg!(target_os = "windows");

    let (width, height) = if is_windows {
        get_terminal_size_windows()
    } else {
        get_terminal_size_linux()
    }; 
    
    println!("{}{}\n{}", (|| -> String {
        " ".repeat(usize::from(width - (width % 2) / 2) - usize::from(directory.last().unwrap().len() / 2))
    })(), directory.last().unwrap(), (|| -> String {
        "-".repeat(usize::from(width) * 2)
    })());

    let files: Vec<LocalDirectory> = read_local_directory();
    let mut x: usize = files.len();
    let mut y: usize = 0;
    if x > (height * 2 - 2) {
        y = *selection;
        x = (height * 2 - 2 - 11) + *selection;
    }

    for index in y..x {
        let file: &LocalDirectory = &files[index];
        println!("{}", (|| -> ColoredString {
            let x = format!("{} {}", if file.is_folder { "+" } else { "-" }, file.name);
            if index == *selection { return x.purple() }
            x.white()
        })())
    }

    // mostrar atalhos
    println!("{}{}\n{}", (|| -> String {
        if files.len() > (height * 2 - 2) {
            return String::new();
        }

        "\n".repeat(height * 2 - 4 - 9 - files.len())
    })(), (|| -> String {
        "_".repeat(width * 2)
    })(), (|| -> String {
        let atl = [
            ["Down", "Down file"],
            ["Up", "Up file"],
            ["Enter", "Open"],
            ["Ctrl + C", "Kill process"]
        ];
        let x = atl.len();

        let division = ": ";
        let start_division = " - ";
        format!("{}{}", (|| -> String {
            let mut rst: usize = 0;
            for i in 0..x {
                let a = &atl[i];
                rst += (|| -> usize { if i == 0 { 0 } else { start_division.len() } })() + a[0].len() + division.len() + a[1].len();
            }
            " ".repeat(width * 2 - rst - 9)
        })(), (|| -> String {
            let mut rst = String::new();
            for i in 0..x {
                let at = atl[i];

                if i != 0 {
                    rst.push_str(start_division);
                }

                rst = format!("{rst}{}", at[0].green());
                rst.push_str(division);
                rst = format!("{rst}{}", at[1].blue());
            }
            rst
        })())
    })())
}

fn update_editor(selection: &mut usize, line_selected: &mut usize, col_selected: &mut usize, is_first: &mut bool) {

    let is_windows = cfg!(target_os = "windows");

    let (width, height) = if is_windows {
        get_terminal_size_windows()
    } else {
        get_terminal_size_linux()
    }; 

    let files: Vec<LocalDirectory> = read_local_directory();
    
    println!("{}{}\n{}", (|| -> String {
        " ".repeat(usize::from(width - (width % 2) / 2) - usize::from(files[*selection].name.len() / 2))
    })(), files[*selection].name, (|| -> String {
        "-".repeat(usize::from(width) * 2)
    })());

    let file_content: String = if !*is_first {
        fs::read_to_string(".lgvim").expect("this file is not a valid")
    } else {
        let x: String = fs::read_to_string(files[*selection].path.clone()).expect("this file is not a valid");
        fs::write(".lgvim", x.clone()).expect("Fail to write file");
        *is_first = false;
        x
    };
    for (index, line_) in file_content.lines().enumerate() {
        let mut line: String = line_.to_string();
        line.push(' ');
        println!("{}", (|| -> String {
            let x = if index == *line_selected {
                format!(" {} |", index+1).purple()
            } else {
                format!(" {} |", index+1).white()
            };
            format!("{x} {}", (|| -> String {
                if index != *line_selected { return line.to_string(); }
                let mut y: String = String::new();
                for (i, c) in line.chars().enumerate() {
                    if i == *col_selected { y = format!("{}{}", y, String::from(c).on_white().black()) }
                    else { y.push(c); }
                }
                y
            })())
        })())
    }
}

fn clear() {
    let is_windows = cfg!(target_os = "windows");

    if is_windows {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("_");
    } else {
        Command::new("sh")
            .args(&["-c", "clear"])
            .status()
            .expect("_");
    }
}

pub fn run() {
    fs::write(".lgvim", "NOTHING").expect("Fail to write file");
    let mut dir: Vec<String> = vec![];
    let mut col_selected: usize = 0;
    let mut line_selected: usize = 0;
    let mut selection: usize = 0;
    let mut is_first: bool = true;

    if let Ok(current_dir) = env::current_dir() {
        if let Some(dir_name) = current_dir.file_name() {
            if let Some(dir_name_str) = dir_name.to_str() {
                dir.push(dir_name_str.to_string());
            }
        }
    }

    let mut current_interface: Interface = Interface::Files;
    loop {
            match current_interface {
            Interface::Files => {
                clear();
                update_file(&mut dir, &mut selection);
                terminal::enable_raw_mode().expect("Failed to enable raw mode");
                let event_listenner: KeyEvents = event_listenner();

                match event_listenner.modifiers {
                    KeyModifiers::CONTROL => {
                        if event_listenner.code == KeyCode::Char('c') {
                            terminal::disable_raw_mode().expect("Failed to disable raw mode");
                            clear();
                            println!("Ctrl+C - Kill LGvim process.");
                            fs::write(".lgvim", "NOTHING").expect("Fail to write file");
                            fs::remove_file(".lgvim").expect("Fail to delete file");
                            break;
                        }
                    }
                    _ => {
                        match event_listenner.code {
                            KeyCode::Up => {
                                if selection > 0 { selection -= 1; }
                            } 
                            KeyCode::Down => {
                                if selection < (read_local_directory().len() - 1) {
                                    selection += 1;
                                }
                            }
                            KeyCode::Enter => {
                                line_selected = 0;
                                col_selected  = 0;
                                is_first = true;
                                current_interface = Interface::Editor;
                            }
                            _ => {}
                        }
                    }
                }
                terminal::disable_raw_mode().expect("Failed to disable raw mode");
            },
            Interface::Editor => {
                clear();
                update_editor(&mut selection, &mut line_selected, &mut col_selected, &mut is_first);
                terminal::enable_raw_mode().expect("Failed to enable raw mode");
                let event_listenner: KeyEvents = event_listenner();

                match event_listenner.modifiers {
                    KeyModifiers::CONTROL => {
                        match event_listenner.code {
                            KeyCode::Char('s') => {
                                let to_save: String = fs::read_to_string(".lgvim").expect("Fail to read file");
                                fs::write(read_local_directory()[selection].path.clone(), to_save).expect("Fail to write file");
                                is_first = true;
                                clear();
                                println!("Saving...");
                                thread::sleep(time::Duration::new(1, 0));
                            }
                            KeyCode::Enter => {
                                let mut file_content: String = fs::read_to_string(".lgvim").expect("this file is not a valid");
                                file_content.push('\n');
                                fs::write(".lgvim", file_content).expect("Fail to write file");
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        match event_listenner.code {
                            KeyCode::Esc => {
                                current_interface = Interface::Files;
                            }
                            KeyCode::Down => {
                                let file_content: String = fs::read_to_string(".lgvim").expect("this file is not a valid");
                                let lines: Vec<&str> = file_content.lines().collect();
                                if (lines.len() - 1) > line_selected {
                                    line_selected += 1;

                                    if lines[line_selected].len() < col_selected {
                                        col_selected = lines[line_selected].len() - 1;
                                    }
                                }
                            }
                            KeyCode::Up => {
                                if line_selected > 0 {
                                    line_selected -= 1;

                                    let file_content: String = fs::read_to_string(".lgvim").expect("this file is not a valid");
                                    let lines: Vec<&str> = file_content.lines().collect();
                                    if lines[line_selected].len() < col_selected {
                                        col_selected = lines[line_selected].len() - 1;
                                    }
                                }
                            }
                            KeyCode::Left => {
                                if col_selected > 0 {
                                    col_selected -= 1;
                                }
                            }
                            KeyCode::Right => {
                                let file_content: String = fs::read_to_string(".lgvim").expect("this file is not a valid");
                                let lines: Vec<&str> = file_content.lines().collect();
                                if lines[line_selected].len() > col_selected {
                                    col_selected += 1;
                                }
                            }
                            KeyCode::Enter => {
                                let file_content: String = fs::read_to_string(".lgvim").expect("this file is not valid");
                                let lines: Vec<&str> = file_content.lines().collect();
                                let mut new_lines: Vec<String> = vec![];
                            
                                for (index, line) in lines.iter().enumerate() {
                                    if index == line_selected {
                                        let line_before_cursor: String = line.chars().take(col_selected).collect();
                                        let line_after_cursor: String = line.chars().skip(col_selected).collect();
                            
                                        new_lines.push(line_before_cursor);
                            
                                        if col_selected == line.len() {
                                            new_lines.push("\r".to_string()); 
                                            col_selected = 0;
                                            line_selected += 1;
                                        }
                            
                                        new_lines.push(line_after_cursor);
                                    } else {
                                        new_lines.push(line.to_string());
                                    }
                                }

                                line_selected += 1;
                                col_selected = 0;
                            
                                fs::write(".lgvim", new_lines.join("\n")).expect("Fail to write file");
                            }
                            KeyCode::Backspace => {
                                let file_content: String = fs::read_to_string(".lgvim").expect("Fail to read file");
                                let lines: Vec<&str> = file_content.lines().collect();
                                let mut new_lines: Vec<String> = vec![];
                            
                                for (index, line) in lines.iter().enumerate() {
                                    if index == line_selected {
                                        if col_selected == 0 && line_selected > 0 {
                                            let ind: usize = new_lines.len() - 1;
                                            col_selected = new_lines[ind].len() + 1;
                                            line_selected -= 1;
                                            new_lines[ind] = format!("{}{}", new_lines[ind], line);
                                        } else if col_selected > 0 {
                                            let mut ch: Vec<char> = line.chars().collect();
                                            ch.remove(col_selected - 1);
                                            col_selected -= 1;
                                            let mut content: String = String::new();
                                            for c in ch {
                                                content.push(c);
                                            }
                                            new_lines.push(content)
                                        } else if col_selected == 0 && line_selected == 0 {
                                            new_lines.push(line.to_string())
                                        }
                                    } else { new_lines.push(line.to_string()) }
                                }
                            
                                fs::write(".lgvim", new_lines.join("\n")).expect("Fail to write file");
                            }                                                                            
                            KeyCode::Char(c) => {
                                let x: String =  fs::read_to_string(".lgvim").expect("Fail to read file");
                                let lines: Vec<&str> = x.lines().collect();
                                let mut new_lines: Vec<String> = vec![];

                                if x.is_empty() {
                                    fs::write(".lgvim", String::from(c)).expect("Fail to write file");
                                } else {

                                    for (index, line) in lines.iter().enumerate() {
                                        let mut line_: String = String::new();
                                        if index == line_selected {
                                            if col_selected >= (line.len() - 1) {
                                                line_.push_str(line);
                                                line_.push(c);
                                            } else {
                                                for i in 0..line.len() {
                                                    if i == col_selected {
                                                        line_.push(c);
                                                    }
                                                    let y: Vec<char> = line.chars().collect();
                                                    line_.push(y[i])
                                                }
                                            }
                                        } else { line_.push_str(line) }
                                        new_lines.push(line_);
                                    }

                                    fs::write(".lgvim", new_lines.join("\n")).expect("Fail to write file");

                                }
                                col_selected += 1;
                            }
                            _ => {}
                        }
                    }
                }
                terminal::disable_raw_mode().expect("Failed to disable raw mode");
            },
            // Iterface::LiveShare => {}
        }
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}