use crossterm::terminal;
use std::env;
use std::fs;
use std::path::PathBuf;
use crate::structs;
use crate::utils;
use crate::assemblers;
use crate::keyboard;
use crate::keybinds;
use utils::terminal::clear;
use assemblers::files;
use assemblers::file_editor;
use structs::Editor;
use structs::Interface;
use structs::File;
use structs::KeyEvents;
use structs::Mode;

pub fn run() {
    let mut editor: Editor = Editor::default();
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    let mut local: PathBuf = env::current_dir().unwrap();
    
    while !editor.stopped {
        assemble_ui(&mut local, &mut editor);
        local = env::current_dir().unwrap();
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn assemble_ui(local: &mut PathBuf, editor: &mut Editor) {
    clear();
    match editor.interface {
        Interface::Files => {
            let files: Vec<File> = utils::fs::get_all(&local.display().to_string());
            editor.files = files.clone();
            files::assemble(files, editor);
            let event: KeyEvents = keyboard::listener();
            keybinds::files(event, editor);
        },
        Interface::Editor => {
            let content: String = fs::read_to_string(&editor.file_tmp).unwrap();
            let lines: Vec<&str> = content.lines().collect();
            editor.file_lines = lines.len();
            editor.file_lines_vec = lines.iter().map(|x| x.to_string()).collect();
            if editor.cursor.y > lines.len() { editor.cursor.y = lines.len() - 1 }
            file_editor::assemble(content, editor);  
            
            if !editor.force_updates {
                let event: KeyEvents = keyboard::listener();
                keybinds::file_editor(event, editor);
            } else {
                std::thread::sleep(std::time::Duration::from_micros(500));
                editor.updated += 1;
                if editor.updated >= editor.force_quant {
                    match editor.mode {
                        Mode::Command => {
                            editor.updated = 0;
                            editor.force_quant = 3;
                            editor.force_updates = false;
                            editor.output = "The request took too long. Process finished.".to_string()
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
