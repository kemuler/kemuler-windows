use windows::Win32::{Foundation, UI::WindowsAndMessaging};

use crate::{MouseButton, VirtualKey};

struct WindowHandle {
    handle: Foundation::HWND,
}

fn send_message(
    window: &WindowHandle,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> Foundation::LRESULT {
    unsafe {
        WindowsAndMessaging::SendMessageW(
            window.handle,
            msg,
            Foundation::WPARAM(wparam),
            Foundation::LPARAM(lparam),
        )
    }
}

fn key_down(window: &WindowHandle, key: VirtualKey) -> Foundation::LRESULT {
    send_message(
        window,
        WindowsAndMessaging::WM_KEYDOWN,
        key.code().0 as usize,
        0,
    )
}

fn key_up(window: &WindowHandle, key: VirtualKey) -> Foundation::LRESULT {
    send_message(
        window,
        WindowsAndMessaging::WM_KEYUP,
        key.code().0 as usize,
        0,
    )
}

fn lparam_pos(x: i16, y: i16) -> i32 {}

fn mouse_button_down(
    window: &WindowHandle,
    mouse_button: MouseButton,
    x: i16,
    y: i16,
) -> Foundation::LRESULT {
    match mouse_button {
        MouseButton::Left => send_message(window, WindowsAndMessaging::WM_LBUTTONDOWN, 0, 0),
        MouseButton::Middle => send_message(window, WindowsAndMessaging::WM_MBUTTONDOWN, 0, 0),
        MouseButton::Right => send_message(window, WindowsAndMessaging::WM_RBUTTONDOWN, 0, 0),
        MouseButton::X1 => send_message(window, WindowsAndMessaging::WM_XBUTTONDOWN, 1 << 16, 0),
        MouseButton::X2 => send_message(window, WindowsAndMessaging::WM_XBUTTONDOWN, 2 << 16, 0),
    }
}
