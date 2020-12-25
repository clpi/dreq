use std::convert::TryFrom;
use crossterm::{
    event::{KeyCode, KeyModifiers},
};

pub enum KeyAction {
    Move(Direction),
    Scroll(Direction),
    Input(String),
    Unregistered((KeyCode, KeyModifiers)),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl KeyAction {

}

impl From<(KeyCode, KeyModifiers)> for KeyAction {
    fn from((key, kmod): (KeyCode, KeyModifiers)) -> Self {
        use Direction::{Up, Down, Left, Right};
        if kmod.eq(&KeyModifiers::NONE) {
            match key {
                KeyCode::Up => return Self::Move(Up),
                KeyCode::Down => return Self::Move(Down),
                KeyCode::Right => return Self::Move(Right),
                KeyCode::Left => return Self::Move(Left),
                KeyCode::Char(c) => return Self::Input(c.to_string()),
                _ => return Self::Unregistered((key, kmod)),
            }
        } else if kmod.contains(KeyModifiers::CONTROL) {
            if kmod.contains(KeyModifiers::SHIFT) {
                match key {
                    _ => return Self::Unregistered((key, kmod)),
                }
            } else if kmod.contains(KeyModifiers::ALT) {
                match key {
                    _ => return Self::Unregistered((key, kmod)),
                }
            } else {
                match key {
                    KeyCode::Char('j') => Self::Move(Down),
                    KeyCode::Char('k') => Self::Move(Up),
                    KeyCode::Char('h') => Self::Move(Left),
                    KeyCode::Char('l') => Self::Move(Right),
                    KeyCode::Up => Self::Scroll(Up),
                    KeyCode::Down => Self::Scroll(Up),
                    KeyCode::Left => Self::Scroll(Up),
                    KeyCode::Right => Self::Scroll(Up),
                    _ => return Self::Unregistered((key, kmod)),
                }
            }
        } else if kmod.contains(KeyModifiers::ALT) {
            match key {
                _ => return Self::Unregistered((key, kmod)),
            }
        } else if kmod.contains(KeyModifiers::SHIFT) {
            match key {
                KeyCode::Char(c) => Self::Input(c.to_string().to_uppercase()),
                _ => return Self::Unregistered((key, kmod)),
            }
        } else {
            return Self::Unregistered((key, kmod));
        }
    }
}

