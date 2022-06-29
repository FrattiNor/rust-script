use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::time::Duration;
use std::{fs, thread};
use winapi::shared::windef::HWND__;
use winapi::um::winuser::{FindWindowW, SetForegroundWindow, ShowWindow, SW_RESTORE};

pub fn sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
}

pub fn create_dir(dir: &str) {
    match fs::create_dir(dir) {
        Ok(_) => (),
        Err(_) => (),
    }
}

pub fn encode_wide(s: String) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(&s).encode_wide().chain(once(0)).collect();
    wide
}

#[cfg(windows)]
pub fn open_window(title: &str) -> Option<*mut HWND__> {
    unsafe {
        // 根据名称获取窗口句柄
        let window_handle = FindWindowW(null_mut(), encode_wide(String::from(title)).as_ptr());

        // 判断句柄是否获取到
        if window_handle.is_null() {
            println!("未找到窗口");
            return None;
        }

        ShowWindow(window_handle, SW_RESTORE);

        sleep(100);

        SetForegroundWindow(window_handle);

        sleep(100);

        Some(window_handle)
    }
}
