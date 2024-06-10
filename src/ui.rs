mod files; 
mod editor; 

use crate::structs;
use structs::Editor;
use structs::Interface;

use crate::utils;
use utils::explorer;

pub fn assembler(editor: &mut Editor) {
  let local: String = format!("{}", std::env::current_dir().unwrap().display());
  match editor.interface {
    Interface::Files => {
      let files = explorer::get_all(&local);
      editor.files = files.clone();
      files::assemble(editor, files);
    },
    Interface::Editor => {
      utils::terminal::back_to_zero();
      editor::assemble(editor);
    }
  }
}