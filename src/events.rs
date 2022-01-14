use crossterm::event::{
    poll, read, Event as TermEvent, KeyCode as TermKeyCode, KeyModifiers as CrosstermKeyModifiers,
    MouseButton as CrosstermMouseButton, MouseEvent as CrossTermMouseEvent,
    MouseEventKind as CrosstermMouseEventKind,
};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyMod {
    Shift,
    Control,
    Alt,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub keymod: KeyMod,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MouseEventKind {
    Down(MouseButton),
    Up(MouseButton),
    Drag(MouseButton),
    Moved,
    ScrollDown,
    ScrollUp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Esc,
    Null,
}

#[derive(Clone, Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Key(KeyMod, KeyCode),
    Resize((u16, u16)),
}
impl Event {
    pub fn read_blocking() -> Option<Event> {
        match read() {
            Ok(event) => match event {
                TermEvent::Resize(column, row) => Some(Event::Resize((column, row))),
                TermEvent::Key(event) => Some(Event::Key(
                    crossterm_keymod_converter(event.modifiers),
                    crossterm_keycode_converter(event.code),
                )),
                TermEvent::Mouse(event) => {
                    Some(Event::Mouse(crossterm_mouseevent_converter(event)))
                }
            },
            Err(_) => None,
        }
    }

    pub fn read_non_blocking(duration: Duration) -> Option<Event> {
        if poll(duration).unwrap_or(false) {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read() {
                Ok(event) => match event {
                    TermEvent::Resize(column, row) => Some(Event::Resize((column, row))),
                    TermEvent::Key(event) => Some(Event::Key(
                        crossterm_keymod_converter(event.modifiers),
                        crossterm_keycode_converter(event.code),
                    )),
                    TermEvent::Mouse(event) => {
                        Some(Event::Mouse(crossterm_mouseevent_converter(event)))
                    }
                },
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

fn crossterm_keycode_converter<'a>(code: TermKeyCode) -> KeyCode {
    let code: KeyCode = match code {
        TermKeyCode::Backspace => KeyCode::Backspace,
        TermKeyCode::Enter => KeyCode::Enter,
        TermKeyCode::Left => KeyCode::Left,
        TermKeyCode::Right => KeyCode::Right,
        TermKeyCode::Up => KeyCode::Up,
        TermKeyCode::Down => KeyCode::Down,
        TermKeyCode::Home => KeyCode::Home,
        TermKeyCode::End => KeyCode::End,
        TermKeyCode::PageUp => KeyCode::PageDown,
        TermKeyCode::PageDown => KeyCode::PageDown,
        TermKeyCode::Tab => KeyCode::Tab,
        TermKeyCode::BackTab => KeyCode::BackTab,
        TermKeyCode::Delete => KeyCode::Delete,
        TermKeyCode::Insert => KeyCode::Insert,
        TermKeyCode::F(digit) => KeyCode::F(digit),
        TermKeyCode::Char(char) => KeyCode::Char(char),
        TermKeyCode::Esc => KeyCode::Esc,
        TermKeyCode::Null => KeyCode::Null,
    };
    code
}

fn crossterm_keymod_converter(modifier: CrosstermKeyModifiers) -> KeyMod {
    match modifier {
        CrosstermKeyModifiers::SHIFT => KeyMod::Shift,
        CrosstermKeyModifiers::CONTROL => KeyMod::Control,
        CrosstermKeyModifiers::ALT => KeyMod::Alt,
        CrosstermKeyModifiers::NONE => KeyMod::None,
        _ => KeyMod::None,
    }
}

fn crossterm_mouseevent_converter(event: CrossTermMouseEvent) -> MouseEvent {
    let keymod: KeyMod = crossterm_keymod_converter(event.modifiers);
    let column = event.column;
    let row = event.row;
    let kind: MouseEventKind = match event.kind {
        CrosstermMouseEventKind::Down(btn) => {
            MouseEventKind::Down(crossterm_mousebutton_converter(btn))
        }
        CrosstermMouseEventKind::Up(btn) => {
            MouseEventKind::Up(crossterm_mousebutton_converter(btn))
        }
        CrosstermMouseEventKind::Drag(btn) => {
            MouseEventKind::Drag(crossterm_mousebutton_converter(btn))
        }
        CrosstermMouseEventKind::Moved => MouseEventKind::Moved,
        CrosstermMouseEventKind::ScrollDown => MouseEventKind::ScrollDown,
        CrosstermMouseEventKind::ScrollUp => MouseEventKind::ScrollUp,
    };

    MouseEvent {
        keymod,
        column,
        row,
        kind,
    }
}

fn crossterm_mousebutton_converter(button: CrosstermMouseButton) -> MouseButton {
    match button {
        CrosstermMouseButton::Left => MouseButton::Left,
        CrosstermMouseButton::Middle => MouseButton::Middle,
        CrosstermMouseButton::Right => MouseButton::Right,
    }
}
