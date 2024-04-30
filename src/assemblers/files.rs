use crate::structs;
use crate::utils;
use utils::terminal;
use structs::File;
use structs::Editor;
use structs::Cursor;
use colored::*;

pub fn assemble(files: Vec<File>, editor: &mut Editor) {
    let (_, height) = terminal::dimensions();
    if editor.cursor.file >= files.len() {
        editor.cursor.file = files.len() - 1;
    }
    
    let cursor: Cursor = editor.cursor;
    for (index, file) in files.iter().enumerate() {
        let data: String = format!("{} {}", if file.is_folder { '+' } else { '-' }, file.name);
        if files.len() > usize::from(height) && index < cursor.file {}
        else if index == cursor.file {
            println!("{}", data.purple())
        } 
        else if (cursor.file + index) <= usize::from(height) { 
            println!("{data}"); 
        };
    }
}