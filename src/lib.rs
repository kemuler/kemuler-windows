use std::fmt;

use crate::{
    common_inputs,
    input_event::{ChangeBy, SetTo},
    simulator::Simulate,
};

mod inner;
mod virtual_key;
use common_inputs::ButtonLike;
pub use virtual_key::VirtualKey;

impl ButtonLike for VirtualKey {}

impl fmt::Display for VirtualKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    /// Also known as foward button
    #[doc(alias = "Forward")]
    X1,
    /// Also known as backward button
    #[doc(alias("Backward", "Back"))]
    X2,
}

impl ButtonLike for MouseButton {}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn windowsify_common_mouse_button(button: common_inputs::MouseButton) -> MouseButton {
    match button {
        common_inputs::MouseButton::Left => MouseButton::Left,
        common_inputs::MouseButton::Middle => MouseButton::Middle,
        common_inputs::MouseButton::Right => MouseButton::Right,
    }
}

fn windowsify_common_key(key: common_inputs::Key) -> VirtualKey {
    match key {
        common_inputs::Key::Alt => VirtualKey::Alt,
        common_inputs::Key::Shift => VirtualKey::Shift,
        common_inputs::Key::Control => VirtualKey::Control,
        common_inputs::Key::F1 => VirtualKey::F1,
        common_inputs::Key::F2 => VirtualKey::F2,
        common_inputs::Key::F3 => VirtualKey::F3,
        common_inputs::Key::F4 => VirtualKey::F4,
        common_inputs::Key::F5 => VirtualKey::F5,
        common_inputs::Key::F6 => VirtualKey::F6,
        common_inputs::Key::F7 => VirtualKey::F7,
        common_inputs::Key::F8 => VirtualKey::F8,
        common_inputs::Key::F9 => VirtualKey::F9,
        common_inputs::Key::F10 => VirtualKey::F10,
        common_inputs::Key::F11 => VirtualKey::F11,
        common_inputs::Key::F12 => VirtualKey::F12,
        common_inputs::Key::CapsLock => VirtualKey::CapsLock,
        common_inputs::Key::End => VirtualKey::End,
        common_inputs::Key::Home => VirtualKey::Home,
        common_inputs::Key::PageUp => VirtualKey::PageUp,
        common_inputs::Key::PageDown => VirtualKey::PageDown,
        common_inputs::Key::Escape => VirtualKey::Escape,
        common_inputs::Key::Enter => VirtualKey::Enter,
        common_inputs::Key::Space => VirtualKey::Space,
        common_inputs::Key::Tab => VirtualKey::Tab,
        common_inputs::Key::Backspace => VirtualKey::Backspace,
        common_inputs::Key::Delete => VirtualKey::Delete,
        common_inputs::Key::UpArrow => VirtualKey::UpArrow,
        common_inputs::Key::DownArrow => VirtualKey::DownArrow,
        common_inputs::Key::LeftArrow => VirtualKey::LeftArrow,
        common_inputs::Key::RightArrow => VirtualKey::RightArrow,
    }
}

#[derive(Default)]
pub struct Windows;

impl Windows {
    pub fn new() -> Windows {
        Windows
    }
}

impl Simulate<SetTo<VirtualKey, bool>> for Windows {
    fn simulate(&mut self, simulatable: SetTo<VirtualKey, bool>) {
        let SetTo {
            input: key,
            to: is_down,
        } = simulatable;
        if is_down {
            inner::virtual_key_down(key)
        } else {
            inner::virtual_key_up(key)
        }
    }
}

impl Simulate<SetTo<common_inputs::Key, bool>> for Windows {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::Key, bool>) {
        let SetTo {
            input: key,
            to: is_down,
        } = simulatable;
        let key = windowsify_common_key(key);
        if is_down {
            inner::virtual_key_down(key)
        } else {
            inner::virtual_key_up(key)
        }
    }
}

impl Simulate<SetTo<common_inputs::Char, bool>> for Windows {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::Char, bool>) {
        let SetTo {
            input: char,
            to: is_down,
        } = simulatable;
        let char = char.0;
        if is_down {
            inner::char_key_down(char)
        } else {
            inner::char_key_up(char)
        }
    }
}

impl Simulate<SetTo<common_inputs::MouseButton, bool>> for Windows {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::MouseButton, bool>) {
        let SetTo {
            input: button,
            to: is_down,
        } = simulatable;
        let button = windowsify_common_mouse_button(button);
        if is_down {
            inner::mouse_button_down(button)
        } else {
            inner::mouse_button_up(button)
        }
    }
}

impl Simulate<SetTo<common_inputs::MousePosition, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::MousePosition, (i32, i32)>) {
        let SetTo {
            input: _,
            to: position,
        } = simulatable;
        inner::virtual_desktop_denormalized_mouse_move_to(position.0, position.1);
    }
}

impl Simulate<ChangeBy<common_inputs::MousePosition, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: ChangeBy<common_inputs::MousePosition, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        inner::virtual_desktop_denormalized_mouse_move_to(by.0, by.1);
    }
}

impl Simulate<ChangeBy<common_inputs::MouseScroll, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: ChangeBy<common_inputs::MouseScroll, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        inner::mouse_scroll(by.0, by.1)
    }
}
