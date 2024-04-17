use crossterm::terminal;
use std::env;
use std::fs;
use std::path::PathBuf;
use crate::structs::{self, Editor, Interface, File, KeyEvents};
use crate::utils::{terminal::clear, self};
use crate::keyboard;
use crate::keybinds;
use assemblers::{files, file_editor, self};

pub fn run() {
    let mut editor: Editor = Editor::default();
    terminal::enable_raw_mode().expect("Failed to enable raw mode");

    while !editor.stopped {
        let local = env::current_dir().unwrap();
        assemble_ui(&local, &mut editor);
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn assemble_ui(local: &PathBuf, editor: &mut Editor) {
    clear();
    match editor.interface {
        Interface::Files => {
            let files: Vec<File> = utils::fs::get_all(&local.display().to_string());
            editor.files = files.clone();
            files::assemble(files, editor);
            let event: KeyEvents = keyboard::listenner();
            keybinds::files(event, editor);
        },
        Interface::Editor => {
            let content: String = fs::read_to_string(&editor.file_tmp).unwrap();
            let lines: Vec<&str> = content.lines().collect();
            editor.file_lines = lines.len();
            editor.file_lines_vec = lines.iter().map(|x| x.to_string()).collect();
            if editor.cursor.y > lines.len() { editor.cursor.y = lines.len() - 1 }
            file_editor::assemble(content, editor);            
            let event: KeyEvents = keyboard::listenner();
            keybinds::file_editor(event, editor);
        }
        _ => todo!()
    }
}
