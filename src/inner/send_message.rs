use windows::Win32::{Foundation, UI::WindowsAndMessaging};

struct WindowHandle {
    handle: Foundation::HWND,
}

fn send_message(window_handle: &WindowHandle) -> Foundation::LRESULT {
    let w = Foundation::WPARAM;
    let l = Foundation::LPARAM;
    unsafe { WindowsAndMessaging::SendMessageW(window_handle.handle, 0, w, l) }
}
