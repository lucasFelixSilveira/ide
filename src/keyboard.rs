use crate::structs;
use structs::KeyEvents;
use crossterm::event::{self, KeyEvent};


pub fn listenner() -> KeyEvents {
    let result: KeyEvents;
    loop {
        if let Ok(event) = event::read() {
            match event {
                event::Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind: _,
                    state: _,
                }) => {
                    result = KeyEvents { code, modifiers };
                    break;
                }
                _ => (),
            }
        }
    }
    result
}