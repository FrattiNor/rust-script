use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::time::Duration;
use std::{fs, thread};

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
