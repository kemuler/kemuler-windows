use std::mem::size_of;
use windows::{
    self,
    Win32::UI::{
        Input::KeyboardAndMouse,
        WindowsAndMessaging::{self, GetSystemMetrics},
    },
};

use super::{MouseButton, VirtualKey};

// Thanks solution from https://stackoverflow.com/questions/35138778/sending-keys-to-a-directx-game

#[allow(non_snake_case)]
enum WindowsSendInputEnum {
    Keyboard {
        wVk: KeyboardAndMouse::VIRTUAL_KEY,
        wScan: u16,
        dwFlags: KeyboardAndMouse::KEYBD_EVENT_FLAGS,
    },
    Mouse {
        dx: i32,
        dy: i32,
        mouseData: i32,
        dwFlags: KeyboardAndMouse::MOUSE_EVENT_FLAGS,
    },
    #[allow(unused)]
    Hardware {
        uMsg: u32,
        wParamL: u16,
        wParamH: u16,
    },
}

impl WindowsSendInputEnum {
    pub fn into_windows(self) -> KeyboardAndMouse::INPUT {
        let (a, b) = match self {
            WindowsSendInputEnum::Keyboard {
                wVk,
                wScan,
                dwFlags,
            } => (
                KeyboardAndMouse::INPUT_KEYBOARD,
                KeyboardAndMouse::INPUT_0 {
                    ki: KeyboardAndMouse::KEYBDINPUT {
                        wVk,
                        wScan,
                        dwFlags,
                        time: 0,
                        dwExtraInfo: get_message_extra_info(),
                    },
                },
            ),
            WindowsSendInputEnum::Mouse {
                dx,
                dy,
                mouseData,
                dwFlags,
            } => (
                KeyboardAndMouse::INPUT_MOUSE,
                KeyboardAndMouse::INPUT_0 {
                    mi: KeyboardAndMouse::MOUSEINPUT {
                        dx,
                        dy,
                        mouseData,
                        dwFlags,
                        time: 0,
                        dwExtraInfo: get_message_extra_info(),
                    },
                },
            ),
            WindowsSendInputEnum::Hardware {
                uMsg,
                wParamL,
                wParamH,
            } => (
                KeyboardAndMouse::INPUT_HARDWARE,
                KeyboardAndMouse::INPUT_0 {
                    hi: KeyboardAndMouse::HARDWAREINPUT {
                        uMsg,
                        wParamL,
                        wParamH,
                    },
                },
            ),
        };
        KeyboardAndMouse::INPUT {
            r#type: a,
            Anonymous: b,
        }
    }
}

#[allow(unused)]
fn primary_screen_size() -> (i32, i32) {
    // SAFETY: calls has no dangerous side-effects
    let x = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CXSCREEN) };
    let y = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CYSCREEN) };
    (x, y)
}

fn virtual_screen_size() -> (i32, i32) {
    // SAFETY: calls has no dangerous side-effects
    let x = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CXVIRTUALSCREEN) };
    let y = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CYVIRTUALSCREEN) };
    (x, y)
}

#[allow(unused)]
fn get_cursor_position() -> Option<(i32, i32)> {
    let mut pos = windows::Win32::Foundation::POINT { x: 0, y: 0 };
    let res = unsafe { WindowsAndMessaging::GetCursorPos(&mut pos) };
    if res.as_bool() {
        Some((pos.x, pos.y))
    } else {
        None
    }
}

#[allow(unused)]
fn ass_set_cursor_position(x: i32, y: i32) {
    unsafe { WindowsAndMessaging::SetCursorPos(x, y) };
}

fn send_input(inputs: &[KeyboardAndMouse::INPUT]) {
    unsafe { KeyboardAndMouse::SendInput(inputs, size_of::<KeyboardAndMouse::INPUT>() as i32) };
}

fn get_message_extra_info() -> usize {
    unsafe { WindowsAndMessaging::GetMessageExtraInfo() }.0 as usize
}

// TODO: Needed testing
pub fn mouse_scroll(x: i32, y: i32) {
    send_input(&[
        WindowsSendInputEnum::Mouse {
            dx: 0,
            dy: 0,
            mouseData: y,
            dwFlags: KeyboardAndMouse::MOUSEEVENTF_WHEEL,
        }
        .into_windows(),
        WindowsSendInputEnum::Mouse {
            dx: 0,
            dy: 0,
            mouseData: x,
            dwFlags: KeyboardAndMouse::MOUSEEVENTF_HWHEEL,
        }
        .into_windows(),
    ]);
}

// TODO: Needed testing
pub fn mouse_button_down(button: MouseButton) {
    let flag = match button {
        MouseButton::Left => KeyboardAndMouse::MOUSEEVENTF_LEFTDOWN,
        MouseButton::Middle => KeyboardAndMouse::MOUSEEVENTF_MIDDLEDOWN,
        MouseButton::Right => KeyboardAndMouse::MOUSEEVENTF_RIGHTDOWN,
        MouseButton::X1 | MouseButton::X2 => KeyboardAndMouse::MOUSEEVENTF_XDOWN,
    };
    let mouse_data = match button {
        MouseButton::X1 => WindowsAndMessaging::XBUTTON1,
        MouseButton::X2 => WindowsAndMessaging::XBUTTON2,
        MouseButton::Left | MouseButton::Middle | MouseButton::Right => 0,
    };
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: 0,
        dy: 0,
        mouseData: mouse_data as i32,
        dwFlags: flag,
    }
    .into_windows()]);
}

// TODO: Needed testing
pub fn mouse_button_up(button: MouseButton) {
    let flag = match button {
        MouseButton::Left => KeyboardAndMouse::MOUSEEVENTF_LEFTUP,
        MouseButton::Middle => KeyboardAndMouse::MOUSEEVENTF_MIDDLEUP,
        MouseButton::Right => KeyboardAndMouse::MOUSEEVENTF_RIGHTUP,
        MouseButton::X1 | MouseButton::X2 => KeyboardAndMouse::MOUSEEVENTF_XUP,
    };
    let mouse_data = match button {
        MouseButton::X1 => WindowsAndMessaging::XBUTTON1,
        MouseButton::X2 => WindowsAndMessaging::XBUTTON2,
        MouseButton::Left | MouseButton::Middle | MouseButton::Right => 0,
    };
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: 0,
        dy: 0,
        mouseData: mouse_data as i32,
        dwFlags: flag,
    }
    .into_windows()]);
}

/// `x` and `y` contain normalized absolute coordinates between 0 and 65,535.
/// The event procedure maps these coordinates onto the display surface.
/// Coordinate (0,0) maps onto the upper-left corner of the display surface;
/// coordinate (65535,65535) maps onto the lower-right corner.
/// In a multimonitor system, the coordinates map to the primary monitor.
#[allow(unused)]
pub fn mouse_move_to(x: i32, y: i32) {
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: x,
        dy: y,
        mouseData: 0,
        dwFlags: KeyboardAndMouse::MOUSEEVENTF_MOVE | KeyboardAndMouse::MOUSEEVENTF_ABSOLUTE,
    }
    .into_windows()]);
}

// TODO: Needed testing
#[allow(unused)]
pub fn denormalized_mouse_move_to(x: i32, y: i32) {
    let (screen_size_x, screen_size_y) = primary_screen_size();
    let x = x * 65535 / screen_size_x;
    let y = y * 65535 / screen_size_y;
    mouse_move_to(x, y);
}

/// same as [`mouse_move_to`] but the coordinates map to the entire virtual desktop.
pub fn virtual_desktop_mouse_move_to(x: i32, y: i32) {
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: x,
        dy: y,
        mouseData: 0,
        dwFlags: KeyboardAndMouse::MOUSEEVENTF_MOVE
            | KeyboardAndMouse::MOUSEEVENTF_ABSOLUTE
            | KeyboardAndMouse::MOUSEEVENTF_VIRTUALDESK,
    }
    .into_windows()]);
}

// TODO: Needed testing
pub fn virtual_desktop_denormalized_mouse_move_to(x: i32, y: i32) {
    let (screen_size_x, screen_size_y) = virtual_screen_size();
    let x = x * 65535 / screen_size_x;
    let y = y * 65535 / screen_size_y;
    virtual_desktop_mouse_move_to(x, y);
}

/// `x` and `y` specify movement relative to the previous mouse event (the last reported position).
/// Positive values mean the mouse moved right (or down);
/// negative values mean the mouse moved left (or up).
///
/// Relative mouse motion is subject to the effects of the mouse speed and the two-mouse threshold values.
/// A user sets these three values with the Pointer Speed slider of the Control Panel's Mouse Properties sheet.
/// You can obtain and set these values using the `SystemParametersInfo` function.
///
/// The system applies two tests to the specified relative mouse movement.
/// If the specified distance along either the x or y axis is greater than the first mouse threshold value,
/// and the mouse speed is not zero, the system doubles the distance.
/// If the specified distance along either the x or y axis is greater than the second mouse threshold value,
/// and the mouse speed is equal to two,
/// the system doubles the distance that resulted from applying the first threshold test.
/// It is thus possible for the system to multiply specified relative mouse movement along the x or y axis
// by up to four times.
///
/// from https://stackoverflow.com/questions/60268940/sendinput-mouse-movement-calculation
/// It is not worth it trying to normalize by mathing.
#[allow(unused)]
pub fn mouse_move_by(x: i32, y: i32) {
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: x,
        dy: y,
        mouseData: 0,
        dwFlags: KeyboardAndMouse::MOUSEEVENTF_MOVE,
    }
    .into_windows()]);
}

/// return true if is successful
#[allow(unused)]
pub fn deaccelerated_mouse_move_by(x: i32, y: i32) -> bool {
    let Some((current_x, current_y)) = get_cursor_position() else {
        return false;
    };
    denormalized_mouse_move_to(current_x + x, current_y + y);
    true
}

/// return true if is successful
#[allow(unused)]
pub fn virtual_desktop_deaccelerated_mouse_move_by(x: i32, y: i32) -> bool {
    let Some((current_x, current_y)) = get_cursor_position() else {
        return false;
    };
    virtual_desktop_denormalized_mouse_move_to(current_x + x, current_y + y);
    true
}

pub fn virtual_key_down(key: VirtualKey) {
    send_input(&[WindowsSendInputEnum::Keyboard {
        wVk: key.code(),
        wScan: 0,
        dwFlags: KeyboardAndMouse::KEYBD_EVENT_FLAGS::default(),
    }
    .into_windows()]);
}

pub fn virtual_key_up(key: VirtualKey) {
    send_input(&[WindowsSendInputEnum::Keyboard {
        wVk: key.code(),
        wScan: 0,
        dwFlags: KeyboardAndMouse::KEYEVENTF_KEYUP,
    }
    .into_windows()]);
}

#[allow(unused)]
pub fn unicode_utf16_key_down(utf16_char: u16) {
    send_input(&[WindowsSendInputEnum::Keyboard {
        wVk: KeyboardAndMouse::VIRTUAL_KEY(0),
        wScan: utf16_char,
        dwFlags: KeyboardAndMouse::KEYEVENTF_UNICODE,
    }
    .into_windows()]);
}

#[allow(unused)]
pub fn unicode_utf16_key_up(utf16_char: u16) {
    send_input(&[WindowsSendInputEnum::Keyboard {
        wVk: KeyboardAndMouse::VIRTUAL_KEY(0),
        wScan: utf16_char,
        dwFlags: KeyboardAndMouse::KEYEVENTF_UNICODE | KeyboardAndMouse::KEYEVENTF_KEYUP,
    }
    .into_windows()]);
}

pub fn char_key_down(char: char) {
    // from the doc: A buffer of length 2 is large enough to encode any `char`.
    let mut utf16_bytes = [0; 2];
    let utf16_bytes = char.encode_utf16(&mut utf16_bytes);
    let inputs = utf16_bytes
        .iter()
        .map(|c| {
            WindowsSendInputEnum::Keyboard {
                wVk: KeyboardAndMouse::VIRTUAL_KEY(0),
                wScan: *c,
                dwFlags: KeyboardAndMouse::KEYEVENTF_UNICODE,
            }
            .into_windows()
        })
        .collect::<Vec<_>>();
    send_input(&inputs[..]);
}

pub fn char_key_up(char: char) {
    // from the doc: A buffer of length 2 is large enough to encode any `char`.
    let mut utf16_bytes = [0; 2];
    let utf16_bytes = char.encode_utf16(&mut utf16_bytes);
    let inputs = utf16_bytes
        .iter()
        .map(|c| {
            WindowsSendInputEnum::Keyboard {
                wVk: KeyboardAndMouse::VIRTUAL_KEY(0),
                wScan: *c,
                dwFlags: KeyboardAndMouse::KEYEVENTF_UNICODE | KeyboardAndMouse::KEYEVENTF_KEYUP,
            }
            .into_windows()
        })
        .collect::<Vec<_>>();
    send_input(&inputs[..]);
}
