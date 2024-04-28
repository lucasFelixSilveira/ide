use crate::structs;
use structs::KeyEvents;
use crossterm::event::{self, KeyEvent};


pub fn listener() -> KeyEvents {
    while let Ok(event) = event::read() {
        match event {
            event::Event::Key(KeyEvent {
                code,
                modifiers,
                kind: _,
                state: _,
            }) => {
                return KeyEvents { code, modifiers };
            }
            _ => (),
        }
    }
    panic!("failed to read key event")
}
