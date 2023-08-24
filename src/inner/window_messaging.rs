use std::{fmt, mem};

use cfg_if::cfg_if;
use windows::Win32::{Foundation, UI::WindowsAndMessaging};

use super::window_and_process::WindowHandle;
use crate::{MouseButton, VirtualKey};

fn send_message(
    window_handle: WindowHandle,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> Foundation::LRESULT {
    unsafe {
        WindowsAndMessaging::SendMessageW(
            window_handle,
            msg,
            Foundation::WPARAM(wparam),
            Foundation::LPARAM(lparam),
        )
    }
}

fn key_down(window_handle: WindowHandle, key: VirtualKey) -> Foundation::LRESULT {
    send_message(
        window_handle,
        WindowsAndMessaging::WM_KEYDOWN,
        key.code().0 as usize,
        0,
    )
}

fn key_up(window_handle: WindowHandle, key: VirtualKey) -> Foundation::LRESULT {
    send_message(
        window_handle,
        WindowsAndMessaging::WM_KEYUP,
        key.code().0 as usize,
        0,
    )
}

cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        fn lparam_mouse_pos(x: i16, y: i16) -> isize {
            let x = x as isize & 0xffffffff;
            let y = y as isize & 0xffffffff;
            (y << 32) | x
        }
    } else if #[cfg(target_pointer_width = "32")] {
        fn lparam_mouse_pos(x: i16, y: i16) -> isize {
            let x = x as isize & 0xffff;
            let y = y as isize & 0xffff;
            (y << 16) | x
        }
    } else {
        fn lparam_mouse_pos(x: i16, y: i16) -> isize {
            panic!("This is not implemented for target that pointer width is not 32 or 64")
        }
    }
}

fn wparam_xbutton1() -> usize {
    let pointer_bit_size = mem::size_of::<usize>() * 8;
    1 << (pointer_bit_size / 2)
}

fn wparam_xbutton2() -> usize {
    let pointer_bit_size = mem::size_of::<usize>() * 8;
    2 << (pointer_bit_size / 2)
}

fn mouse_button_down(
    window_handle: WindowHandle,
    mouse_button: MouseButton,
    x: i16,
    y: i16,
) -> Foundation::LRESULT {
    match mouse_button {
        MouseButton::Left => send_message(
            window_handle,
            WindowsAndMessaging::WM_LBUTTONDOWN,
            0,
            lparam_mouse_pos(x, y),
        ),
        MouseButton::Middle => send_message(
            window_handle,
            WindowsAndMessaging::WM_MBUTTONDOWN,
            0,
            lparam_mouse_pos(x, y),
        ),
        MouseButton::Right => send_message(
            window_handle,
            WindowsAndMessaging::WM_RBUTTONDOWN,
            0,
            lparam_mouse_pos(x, y),
        ),
        MouseButton::X1 => send_message(
            window_handle,
            WindowsAndMessaging::WM_XBUTTONDOWN,
            wparam_xbutton1(),
            lparam_mouse_pos(x, y),
        ),
        MouseButton::X2 => send_message(
            window_handle,
            WindowsAndMessaging::WM_XBUTTONDOWN,
            wparam_xbutton2(),
            lparam_mouse_pos(x, y),
        ),
    }
}
