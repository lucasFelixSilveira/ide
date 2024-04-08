use crate::structs;
use structs::KeyEvents;
use structs::Editor;
mod kb_files;
mod kb_file_editor;

pub fn files(event: KeyEvents, editor: &mut Editor) {
    kb_files::binds(event, editor);
}

pub fn file_editor(event: KeyEvents, editor: &mut Editor) {
    kb_file_editor::binds(event, editor);
}