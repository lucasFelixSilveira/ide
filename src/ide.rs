use crate::ui;
use crate::keybinds;

use crate::structs;
use structs::Editor;
use structs::KeyEvents;

use crate::utils;
use utils::terminal;
use terminal::clear;
use terminal::back_to_zero;

use crate::keyboard;
use keyboard::listener;

pub fn run() {
  let mut editor: Editor = Editor::new();

  clear();
  while !editor.quit {
    back_to_zero();
    ui::assembler(&mut editor);
    keybinds::emit(&mut editor, listener());
  }
}