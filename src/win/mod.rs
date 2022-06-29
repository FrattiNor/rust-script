use crate::utils::{encode_wide, sleep};
use enigo::Enigo;
use enigo::{MouseButton, MouseControllable};
use std::ptr::null_mut;
use winapi::shared::windef::HWND__;
use winapi::shared::windef::{POINT, RECT};
use winapi::um::winuser::{ClientToScreen, GetClientRect};
use winapi::um::winuser::{FindWindowW, SetForegroundWindow, ShowWindow, SW_RESTORE};

#[derive(Debug)]
pub struct Rect {
    pub width_u32: u32,
    pub height_u32: u32,
    pub top_u32: u32,
    pub left_u32: u32,
    pub width_i32: i32,
    pub height_i32: i32,
    pub top_i32: i32,
    pub left_i32: i32,
}

impl Rect {
    #[cfg(windows)]
    pub fn new(window: *mut HWND__) -> Self {
        unsafe {
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };

            GetClientRect(window, &mut rect);

            let mut point = POINT { x: 0, y: 0 };

            ClientToScreen(window, &mut point);

            let width: i32 = rect.right;
            let height: i32 = rect.bottom;
            let left: i32 = point.x;
            let top: i32 = point.y;

            let res = Rect {
                width_u32: width as u32,
                height_u32: height as u32,
                top_u32: top as u32,
                left_u32: left as u32,
                width_i32: width,
                height_i32: height,
                top_i32: top,
                left_i32: left,
            };

            res
        }
    }
}

pub struct Win {
    handle: *mut HWND__,
    pub mouse: Enigo,
}

impl Win {
    #[cfg(windows)]
    pub fn show_win(&self) {
        unsafe {
            ShowWindow(self.handle, SW_RESTORE);

            sleep(100);

            SetForegroundWindow(self.handle);

            sleep(100);
        }
    }

    #[cfg(windows)]
    fn open_window(title: &str) -> Option<*mut HWND__> {
        unsafe {
            // 根据名称获取窗口句柄
            let window_handle = FindWindowW(null_mut(), encode_wide(String::from(title)).as_ptr());

            // 判断句柄是否获取到
            if window_handle.is_null() {
                println!("未找到窗口");
                return None;
            }

            Some(window_handle)
        }
    }

    pub fn new(title: &str) -> Option<Self> {
        let handle = Self::open_window(title);
        match handle {
            Some(handle) => Some(Win {
                handle: handle,
                mouse: Enigo::new(),
            }),
            None => None,
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.handle)
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.mouse.mouse_move_to(x, y);
        sleep(100);
    }

    pub fn mouse_click(&mut self, fun: &str) {
        match fun {
            "left" => self.mouse.mouse_click(MouseButton::Left),
            "right" => self.mouse.mouse_click(MouseButton::Right),
            "middle" => self.mouse.mouse_click(MouseButton::Middle),
            _ => (),
        }
        sleep(100);
    }

    pub fn mouse_down(&mut self, fun: &str) {
        match fun {
            "left" => self.mouse.mouse_down(MouseButton::Left),
            "right" => self.mouse.mouse_down(MouseButton::Right),
            "middle" => self.mouse.mouse_down(MouseButton::Middle),
            _ => (),
        }
        sleep(100);
    }

    pub fn mouse_up(&mut self, fun: &str) {
        match fun {
            "left" => self.mouse.mouse_up(MouseButton::Left),
            "right" => self.mouse.mouse_up(MouseButton::Right),
            "middle" => self.mouse.mouse_up(MouseButton::Middle),
            _ => (),
        }
        sleep(100);
    }

    pub fn mouse_scroll_x(&mut self, number: i32) {
        self.mouse.mouse_scroll_x(number);
        sleep(100);
    }

    pub fn mouse_scroll_y(&mut self, number: i32) {
        self.mouse.mouse_scroll_y(number);
        sleep(100);
    }
}
