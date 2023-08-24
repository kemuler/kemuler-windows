use std::mem::{self, MaybeUninit};
use windows::{
    core::{Error, Result},
    Win32::{
        Foundation::{self, FALSE, TRUE},
        System::{ProcessStatus, Threading},
        UI::WindowsAndMessaging,
    },
};

pub use Foundation::{HANDLE as ProcessHandle, HMODULE as ModuleHandle, HWND as WindowHandle};
pub use Threading::PROCESS_ACCESS_RIGHTS as ProcessAccessRights;

pub struct Window {
    handle: WindowHandle,
}

impl Window {
    pub fn enum_top_windows() -> Result<Vec<Window>> {
        let top_windows = enum_top_windows(1024)?;
        let windows = top_windows
            .into_iter()
            .map(|handle| Window { handle })
            .collect();
        Ok(windows)
    }

    pub fn enum_all_windows() -> Result<Vec<Window>> {
        let top_windows = enum_top_windows(1024)?;
        let child_windows = top_windows
            .iter()
            .flat_map(|parent_window| enum_child_windows(*parent_window, 1024))
            .collect::<Vec<WindowHandle>>();
        let mut windows = top_windows;
        windows.extend(child_windows);
        let windows = windows
            .into_iter()
            .map(|handle| Window { handle })
            .collect();
        Ok(windows)
    }

    pub fn get_text(&self, buffer_capacity: usize) -> Result<String> {
        let mut buffer: Vec<u16> = vec![0; buffer_capacity];
        let result = unsafe { WindowsAndMessaging::GetWindowTextW(self.handle, &mut buffer) };
        if result == 0 {
            return Err(Error::from_win32());
        }
        buffer.truncate(result as usize);
        let string = String::from_utf16(&buffer).unwrap();
        Ok(string)
    }

    pub fn handle(&self) -> WindowHandle {
        self.handle
    }
}

pub struct Process {
    id: u32,
    handle: ProcessHandle,
}

impl Process {
    pub fn open(permission: ProcessAccessRights, process_id: u32) -> Result<Process> {
        let process_handle = unsafe { Threading::OpenProcess(permission, FALSE, process_id)? };
        Ok(Process {
            id: process_id,
            handle: process_handle,
        })
    }

    pub fn from_window_handle(
        permission: ProcessAccessRights,
        window_handle: WindowHandle,
    ) -> Result<Process> {
        let (_, process_id) = get_window_thread_process_id(window_handle)?;
        Process::open(permission, process_id)
    }

    pub fn get_a_module(&self) -> Result<ModuleHandle> {
        let mut module = MaybeUninit::<ModuleHandle>::uninit();
        let mut size = 0;
        // SAFETY: the pointer is valid and the size is correct.
        let result = unsafe {
            ProcessStatus::EnumProcessModules(
                self.handle,
                module.as_mut_ptr(),
                mem::size_of::<ModuleHandle>() as u32,
                &mut size,
            )
        };
        if result == FALSE {
            return Err(Error::from_win32());
        }

        // SAFETY: the call succeeded, so module is initialized.
        let module = unsafe { module.assume_init() };
        Ok(module)
    }

    pub fn get_name_from_module(&self, module: ModuleHandle) -> Result<String> {
        let mut buffer: Vec<u16> = vec![0; 128];
        let length = unsafe { ProcessStatus::GetModuleBaseNameW(self.handle, module, &mut buffer) };
        if length == 0 {
            return Err(Error::from_win32());
        }
        buffer.truncate(length as usize);

        Ok(String::from_utf16(&buffer).unwrap())
    }

    pub fn get_file_path(&self, buffer_capacity: usize) -> Result<String> {
        let mut buffer: Vec<u16> = vec![0; buffer_capacity];
        let length = unsafe {
            ProcessStatus::GetModuleFileNameExW(self.handle, Foundation::HMODULE(0), &mut buffer)
        };
        if length == 0 {
            return Err(Error::from_win32());
        }
        let string = String::from_utf16(&buffer).unwrap();
        Ok(string)
    }

    pub fn get_name(&self, buffer_capacity: usize) -> Result<String> {
        let mut buffer: Vec<u16> = vec![0; buffer_capacity];
        let length = unsafe {
            ProcessStatus::GetModuleBaseNameW(self.handle, Foundation::HMODULE(0), &mut buffer)
        };
        if length == 0 {
            return Err(Error::from_win32());
        }
        buffer.truncate(length as usize);

        // SAFETY: the call succeeded and length represents bytes.
        unsafe { buffer.set_len(length as usize) };
        Ok(String::from_utf16(&buffer).unwrap())
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn handle(&self) -> ProcessHandle {
        self.handle
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe { Foundation::CloseHandle(self.handle) };
    }
}

#[test]
fn uh_main() {
    let windows = Window::enum_all_windows().unwrap();
    let mut windows_processes = windows
        .into_iter()
        .filter_map(|window| {
            let process = Process::from_window_handle(
                Threading::PROCESS_QUERY_INFORMATION | Threading::PROCESS_VM_READ,
                window.handle,
            );
            match process {
                Ok(process) => Some((window, process)),
                Err(_e) => None,
            }
        })
        .collect::<Vec<_>>();
    windows_processes.sort_by_key(|(window, process)| process.id);
    windows_processes.dedup_by_key(|(window, process)| process.id);
    for (window, process) in &windows_processes {
        println!(
            "[{}, {}] {}",
            process.id,
            process.get_name(256).unwrap(),
            window.get_text(256).unwrap_or_default(),
        )
    }

    let window_process = windows_processes
        .into_iter()
        .find(|(window, process)| {
            // process
            //     .get_name(256)
            //     .unwrap()
            //     .to_lowercase()
            //     .contains("firefox")
            process.id == 13224
        })
        .unwrap();
    // println!("{}", window_process.0.get_text(256).unwrap());
    let window = window_process.0;
    std::thread::sleep_ms(2000);
}

fn enum_top_windows(buffer_capacity: usize) -> Result<Vec<WindowHandle>> {
    unsafe extern "system" fn enum_windows_callback(
        window_handle: WindowHandle,
        vec: Foundation::LPARAM,
    ) -> Foundation::BOOL {
        let vec = vec.0 as *mut Vec<WindowHandle>;
        let vec = &mut *vec;
        vec.push(window_handle);
        TRUE
    }

    let mut vec: Vec<WindowHandle> = Vec::with_capacity(buffer_capacity);
    let result = unsafe {
        WindowsAndMessaging::EnumWindows(
            Some(enum_windows_callback),
            Foundation::LPARAM(&mut vec as *mut _ as isize),
        )
    };
    if result == FALSE {
        Err(Error::from_win32())
    } else {
        Ok(vec)
    }
}

fn enum_child_windows(parent_window: WindowHandle, buffer_capacity: usize) -> Vec<WindowHandle> {
    unsafe extern "system" fn enum_windows_callback(
        window_handle: WindowHandle,
        vec: Foundation::LPARAM,
    ) -> Foundation::BOOL {
        let vec = vec.0 as *mut Vec<WindowHandle>;
        let vec = &mut *vec;
        vec.push(window_handle);
        TRUE
    }

    let mut vec: Vec<WindowHandle> = Vec::with_capacity(buffer_capacity);
    let _ = unsafe {
        WindowsAndMessaging::EnumChildWindows(
            parent_window,
            Some(enum_windows_callback),
            Foundation::LPARAM(&mut vec as *mut _ as isize),
        )
    };
    vec
}

fn get_window_thread_process_id(window_handle: WindowHandle) -> Result<(u32, u32)> {
    let mut process_id = 0u32;
    let thread_id = unsafe {
        WindowsAndMessaging::GetWindowThreadProcessId(
            window_handle,
            Some(&mut process_id as *mut _),
        )
    };
    if thread_id == 0 {
        Err(Error::from_win32())
    } else {
        Ok((thread_id, process_id))
    }
}
