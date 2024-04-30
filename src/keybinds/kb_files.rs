use crossterm::event::KeyCode;
use std::path::PathBuf;
use crate::utils;
use utils::terminal;
use crate::structs;
use structs::KeyEvents;
use structs::Interface;
use structs::Editor;
use structs::File;
use std::env;
use std::fs;
use reqwest;

pub fn binds(event: KeyEvents, editor: &mut Editor) {
    match event.modifiers {
        _ => {
            match event.code {
                KeyCode::Down | KeyCode::Char('s') if (editor.files.len() - 1) > editor.cursor.file => editor.cursor.file += 1,
                KeyCode::Up | KeyCode::Char('w') if editor.cursor.file > 0 => editor.cursor.file -= 1,
                KeyCode::Enter => {
                    let file: &File = &editor.files[editor.cursor.file];
                    let current_dir: PathBuf = env::current_dir().unwrap();
                    let complement_path: String = format!("{}{}{}", current_dir.display(), std::path::MAIN_SEPARATOR, file.name);
                    if !file.is_folder { 
                        editor.interface = Interface::Editor;
                        let _file: String = format!("{}.tmp", complement_path);
                        editor.file_tmp = _file.clone();
                        fs::write(_file, fs::read_to_string(complement_path).unwrap()).unwrap();
                        editor.cursor.x = 0;                        
                        editor.cursor.y = 0;                        
                    } 
                    else {
                        env::set_current_dir(complement_path.clone()).expect("Fail to trade directory");
                        let files: Vec<File> = utils::fs::get_all(&complement_path);
                        if (files.len() - 1) < editor.cursor.file {
                            editor.cursor.file = files.len() - 1;
                        }
                    }
                }
                KeyCode::Backspace => {
                    let current_dir: PathBuf = env::current_dir().unwrap();
                    let data: String = current_dir.display().to_string();
                    let (cutted_dir, _) = data.rsplit_once(std::path::MAIN_SEPARATOR).unwrap();
                    env::set_current_dir(cutted_dir.to_string()).expect("Fail to trade directory");         
                    let files: Vec<File> = utils::fs::get_all(&cutted_dir.to_string());
                    if (files.len() - 1) > editor.cursor.file {
                        editor.cursor.file = files.len() - 1;
                    }
                }
                KeyCode::Char('x') => {
                    terminal::clear();
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        let response = reqwest::get(format!("http://127.0.0.1:6932/shutdown/{}", editor.coop.enable)).await;
                        if response.unwrap().status().is_success() && editor.coop.enable {
                            println!("Killed coop server.");
                        }
                    });
                    println!("Killed process.");
                    editor.stopped = true;
                }
                _ => {}
            }
        }
    }
}