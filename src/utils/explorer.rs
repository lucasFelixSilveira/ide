use crate::structs;
use structs::File;
use std::fs;

pub fn get_all(local: &String) -> Vec<File> {
  let current_dir: String = local.clone();
  
  match fs::read_dir(&current_dir) {
    Ok(entries) => {
      let mut result: Vec<File> = vec![];

      for (_, entry) in entries.enumerate() {
        if let Ok(entry) = entry {
          let file_type = entry.file_type();
          let file_name = entry.file_name();
          let path = entry.path();

          let is_folder = file_type.map_or(false, |t| t.is_dir());
          let name = file_name.to_string_lossy().to_string();

          result.push(File {
            is_folder,
            name,
            path
          });
        }
      }
      result
    }
    Err(_) => {
      eprintln!("Error reading the current directory.");
      std::process::exit(1);
    }
  }
}