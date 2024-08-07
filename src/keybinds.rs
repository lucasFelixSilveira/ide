mod files;
mod editor;
mod properties;
pub mod modules;

use crate::structs;
use structs::Editor;
use structs::KeyEvents;
use structs::Interface;

use crate::utils;
use utils::terminal;
use terminal::clear;

pub fn emit(editor: &mut Editor, press: KeyEvents) {
  let interface: Interface = editor.interface.clone();
  match interface {
    Interface::Files => files::valid(editor, press),
    Interface::Editor => editor::valid(editor, press),
    Interface::Properties => properties::valid(editor, press)
  }

  if interface != editor.interface { clear() }
}