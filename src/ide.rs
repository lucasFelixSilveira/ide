use crossterm::terminal;
use core::num;
use std::env;
use std::fs;
use std::path;
use std::path::PathBuf;
use std::process;
use crate::structs;
use crate::structs::CoopPerson;
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

pub fn run(root: String) {

    let mut editor: Editor = Editor::default();
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    let mut local: PathBuf = env::current_dir().unwrap();
    
    while !editor.stopped {
        assemble_ui(&mut local, &mut editor, &root);
        local = env::current_dir().unwrap();
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn assemble_ui(local: &mut PathBuf, editor: &mut Editor, root: &String) {
    clear();

    let file_: String = format!("{}{}{}", root, path::MAIN_SEPARATOR, ".file_");
    let values_: String = format!("{}{}{}", root, path::MAIN_SEPARATOR, ".values_");
    match editor.interface {
        Interface::Files => {
            match fs::remove_file(file_) { _ => {} };
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

            if editor.coop.started {
                fs::write(
                    values_, 
                    format!(
                        "{} {} {} = {}", 
                        editor.cursor.x, 
                        editor.cursor.y,
                        editor.coop.members[editor.coop.possession].color, 
                        editor.coop.members[editor.coop.possession].name
                    )
                ).unwrap();

                let event_emitter = fs::read_to_string("emited_event");
                match event_emitter {
                    Err(_) => {},
                    Ok(event) => {
                        fs::remove_file("emited_event").unwrap();
                        if event == "new_user" {
                            editor.coop.members.push(CoopPerson {
                                color: editor.coop.members.len(),
                                name: fs::read_to_string("new_user").unwrap(),
                                id: editor.coop.members.len()
                            })
                        }
                    }
                }
            }

            if !editor.force_updates {
                fs::write(file_, editor.file_tmp.clone()).unwrap();
                let event: KeyEvents = keyboard::listener();
                keybinds::file_editor(event, editor);
            } else {
                fs::write("./.buffer", "new_file").unwrap();
                editor.file_tmp = "./.buffer".to_string();
                let mut i = 0;
                for (j, file) in utils::fs::get_all(&format!("{}", std::env::current_dir().unwrap().display())).iter().enumerate() {
                    if file.name == ".buffer" {
                        i = j;
                        break;
                    }
                }
                editor.cursor.file = i;

                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let response = reqwest::get("http://127.0.0.1:6932/buffer_content").await;
                    let content = response.unwrap().text().await;
                    fs::write("./.buffer", content.unwrap()).unwrap();
                });

                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let response = reqwest::get("http://127.0.0.1:6932/values").await;
                    let response_content = response.unwrap().text().await;
                    let content: String = response_content.unwrap().clone();
                    let numbers: Vec<&str> = content.split(' ').collect();
                    
                    let _t: (usize, usize, usize) = (
                        numbers[0].parse().unwrap(),
                        numbers[1].parse().unwrap(),
                        numbers[2].parse().unwrap(),
                    );
                    
                    let ( x, y, color) = _t;
                    let (_, name) = content.split_once(" = ").unwrap();
                    
                    editor.cursor.x = x;
                    editor.cursor.y = y;
                    editor.coop.members[0].color = color;
                    editor.coop.members[0].name = name.to_string();
                    editor.coop.enable = true;
                });

                std::thread::sleep(std::time::Duration::from_millis(600));
            }
        }
    }
}
