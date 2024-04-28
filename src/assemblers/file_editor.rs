use crate::structs;
use crate::utils;
use crossterm::style::Stylize;
use utils::terminal;
use structs::Editor;
use structs::Mode;
use colored::*;

pub fn assemble(content: String, editor: &mut Editor) {
    let (width, mut height) = terminal::dimensions();
    let splitter = "_".repeat(<u16 as Into<usize>>::into(width));

    println!("Editing {} with mode: {:?}\n{}\n", editor.files[editor.cursor.file].name, editor.mode, splitter.clone());
    height -= if editor.mode == Mode::Command { 7 } else { 3 };

    let mut diference: usize = 0;
    let lines: Vec<&str> = content.lines().collect();
    let index_size: usize = format!("{}", lines.len()).len();
    let _height = <u16 as Into<usize>>::into(height);
    for (i, line) in lines.iter().enumerate() {
        if lines.len() > height.into() && (i < editor.cursor.y || i > editor.cursor.y+_height-2) {}
        else {
            diference += 1;
            let this_size: usize = format!("{}", i+1).len();
            let index: String = format!(" {}{}", " ".repeat(index_size - this_size), i+1);
            
            if i == editor.cursor.y {
                let show_index: String = format!("{} {} ", index.purple(), "|".purple());
                let x: usize = editor.cursor.x;
                let line_size: usize = line.len();
                let mut _line: String = String::new();
                if line_size > x {
                    let _chars: Vec<char> = line.chars().collect();
                    let _char: char = _chars[x];
                    let _str = if editor.cursor.x == 0 && line.is_empty() { String::from(" ").on_white() } else { _char.to_string().on_white().black() };
                    let _first: String = if editor.cursor.x == 0 { String::new() } else { line[0..x].to_string() };
                    _line = format!("{}{}{}", _first, _str, line[x+1..line_size].to_string())
                } 
                else if line_size == x {
                    if line_size == 0 && line.is_empty() { 
                        _line = format!("{}", String::from(" ").on_white())
                    } 
                    else if line_size == 0 { 
                        _line = format!("{}{}", line[0..0].to_string().on_white(), line[1..line_size-1].to_string())
                    } 
                    else {
                        _line = format!("{}{}", line, " ".to_string().on_white())
                    }
                } else if x > line_size {
                    _line = format!("{} {}{}", line, " ".repeat(x - 1 - line.len()), String::from(" ").on_white())
                }

                let data: String = format!("{}{}", show_index, _line);
                println!("{data}");
            } else {
                println!("{index} | {}", line);
            }
        }
    }

    if editor.mode == Mode::Command { 
        print!("{}", "\n".repeat(_height - diference - 1));

        let splitter2 = "=".repeat(<u16 as Into<usize>>::into(width));
        println!(
            "{}\n> {}: {}\n{}\n$ {}",
            splitter2.clone(),
            "M-X".to_string().purple(),
            (|| -> String {
                if editor.input_command.is_empty() {
                    return format!("{}", " ".to_string().on_white());
                }
                if editor.input_command.len() == 1 && editor.cursor.z == 0 {
                    return format!("{}", editor.input_command.clone().on_white().black());
                }
                if editor.cursor.z == editor.input_command.len() {
                    return format!("{}{}", editor.input_command, " ".to_string().on_white().black());
                }
                format!(
                    "{}{}{}",
                    editor.input_command[0..editor.cursor.z].to_string(),
                    editor.input_command[editor.cursor.z..=editor.cursor.z].to_string().on_white().black(),
                    editor.input_command[(editor.cursor.z + 1)..].to_string()
                )
            })(),
            splitter2,
            editor.output.clone().dark_grey()
        );
    }
}