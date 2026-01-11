//! Event handling for keyboard and mouse input
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use std::io;

/// Represents different types of input events
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Keyboard key press
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resize
    Resize((), ()),
}

/// Read the next event from the terminal
///
/// # Returns
/// An InputEvent or an error
pub fn read_event() -> Result<InputEvent, io::Error> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            Ok(InputEvent::Key(key_event))
        }
        Event::Mouse(mouse_event) => Ok(InputEvent::Mouse(mouse_event)),
        Event::Resize(_width, _height) => Ok(InputEvent::Resize((), ())),
        _ => read_event(), // Ignore release events and others, read again
    }
}

/// Check if a key event matches a specific key code
pub fn is_key(key_event: &KeyEvent, code: KeyCode) -> bool {
    key_event.code == code
}

/// Check if Ctrl (Linux) or Command/Super/Meta (macOS) is pressed with a key
/// This handles cross-platform copy/paste shortcuts
/// Note: On macOS, Command key may be reported as SUPER or META depending on terminal
pub fn is_copy_paste_key(key_event: &KeyEvent, code: KeyCode) -> bool {
    let has_ctrl = key_event.modifiers.contains(event::KeyModifiers::CONTROL);
    let has_super = key_event.modifiers.contains(event::KeyModifiers::SUPER);
    let has_meta = key_event.modifiers.contains(event::KeyModifiers::META);
    (has_ctrl || has_super || has_meta) && is_key(key_event, code)
}

/// Check if Alt/Meta is pressed with a numeric key (1-9)
/// Alternative to Ctrl+number for tab navigation
pub fn is_alt_number(key_event: &KeyEvent, number: u8) -> bool {
    if number < 1 || number > 9 {
        return false;
    }

    let has_alt = key_event.modifiers.contains(event::KeyModifiers::ALT);
    if !has_alt {
        return false;
    }

    if let KeyCode::Char(c) = key_event.code {
        let expected_char = (b'0' + number) as char;
        if c == expected_char {
            return true;
        }
    }

    false
}
