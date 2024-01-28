use std::io;
use std::fs;
use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::vec;
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};
use colored::*;

enum Iterface {
    Files,
    // Editor,
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

    let is_windows = cfg!(target_os = "windows");

    let (width, height) = if is_windows {
        get_terminal_size_windows()
    } else {
        get_terminal_size_linux()
    }; 
    
    println!("{}{}\n{}", (|| -> String {
        " ".repeat(usize::from(width - (width % 2) / 2) - usize::from(directory.last().unwrap().len()))
    })(), directory.last().unwrap(), (|| -> String {
        "-".repeat(usize::from(width) * 2)
    })());

    for (index, file) in read_local_directory().iter().enumerate() {
        println!("{}", (|| -> ColoredString {
            let x = format!("{} {}", if file.is_folder { "+" } else { "-" }, file.name);
            if index == *selection { return x.purple() }
            x.white()
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
    let mut dir: Vec<String> = vec![];
    let mut selection: usize = 0;

    if let Ok(current_dir) = env::current_dir() {
        if let Some(dir_name) = current_dir.file_name() {
            if let Some(dir_name_str) = dir_name.to_str() {
                dir.push(dir_name_str.to_string());
            }
        }
    }

    let current_interface: Iterface = Iterface::Files;
    loop {
            match current_interface {
            Iterface::Files => {
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
                            _ => {}
                        }
                    }
                }
                terminal::disable_raw_mode().expect("Failed to disable raw mode");
            },
            // Iterface::Editor => {},
            // Iterface::LiveShare => {}
        }
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}