use std::mem;

use cfg_if::cfg_if;
use windows::Win32::{Foundation, UI::WindowsAndMessaging};

use crate::{MouseButton, VirtualKey};

type Error = Foundation::WIN32_ERROR;

struct Window {
    handle: Foundation::HWND,
}

impl Window {
    fn enumerate() -> Result<Vec<Window>, Error> {
        let window_handles = enum_windows()?;
        let windows = window_handles
            .into_iter()
            .map(|handle| Window { handle })
            .collect();
        Ok(windows)
    }

    fn name(&self) {}
}

fn enum_windows() -> Result<Vec<Foundation::HWND>, Error> {
    unsafe extern "system" fn enum_windows_callback(
        window_handle: Foundation::HWND,
        vec: Foundation::LPARAM,
    ) -> Foundation::BOOL {
        let vec = vec.0 as *mut Vec<Foundation::HWND>;
        let vec = &mut *vec;
        vec.push(window_handle);
        Foundation::TRUE
    }

    let mut vec: Vec<Foundation::HWND> = Vec::with_capacity(1024);
    let result = unsafe {
        WindowsAndMessaging::EnumWindows(
            Some(enum_windows_callback),
            Foundation::LPARAM(&mut vec as *mut _ as isize),
        )
    };
    if result == Foundation::FALSE {
        Err(unsafe { Foundation::GetLastError() })
    } else {
        Ok(vec)
    }
}

fn get_window_thread_process_id(window_handle: Foundation::HWND) -> Result<(u32, u32), Error> {
    let mut process_id = 0u32;
    let thread_id = unsafe {
        WindowsAndMessaging::GetWindowThreadProcessId(
            window_handle,
            Some(&mut process_id as *mut _),
        )
    };
    if thread_id == 0 {
        Err(unsafe { Foundation::GetLastError() })
    } else {
        Ok((thread_id, process_id))
    }
}

fn send_message(
    window_handle: Foundation::HWND,
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

fn key_down(window_handle: Foundation::HWND, key: VirtualKey) -> Foundation::LRESULT {
    send_message(
        window_handle,
        WindowsAndMessaging::WM_KEYDOWN,
        key.code().0 as usize,
        0,
    )
}

fn key_up(window_handle: Foundation::HWND, key: VirtualKey) -> Foundation::LRESULT {
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
            let x = x as u32 as i64;
            let y = (y as i64) << 32;
            (y | x) as isize
        }
    } else if #[cfg(target_pointer_width = "32")] {
        fn lparam_mouse_pos(x: i16, y: i16) -> isize {
            let x = x as u16 as i32;
            let y = (y as i32) << 16;
            (y | x) as isize
        }
    } else {
        fn lparam_mouse_pos(x: i16, y: i16) -> isize {
            panic!("This is not implemented for target that pointer width is not 32 or 64")
        }
    }
}

fn wparam_xbutton1() -> usize {
    1 << (mem::size_of::<usize>() * 8)
}

fn wparam_xbutton2() -> usize {
    2 << (mem::size_of::<usize>() * 8)
}

fn mouse_button_down(
    window_handle: Foundation::HWND,
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
