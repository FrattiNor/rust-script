use crate::rect::Rect;
use crate::utils::{encode_wide, sleep};
use enigo::{Enigo, KeyboardControllable};
use enigo::{Key, MouseButton, MouseControllable};
use std::ptr::null_mut;
use winapi::shared::windef::HWND__;
use winapi::um::winuser::{FindWindowW, SetForegroundWindow, ShowWindow, SW_RESTORE};

pub struct Win {
    handle: *mut HWND__,
    pub mouse: Enigo,
}

pub struct CheckRGB {
    pub x: u32,
    pub y: u32,
    pub rgb: [u8; 3],
    pub desc: &'static str,
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

    pub fn until_check_rgb(&self, x: u32, y: u32, rgb: [u8; 3], desc: &str) {
        match self.get_rect().get_image().get_rgb_from_bitmap(x, y) {
            Some(image_rgb) => {
                if image_rgb[0] == rgb[0] && image_rgb[1] == rgb[1] && image_rgb[2] == rgb[2] {
                    return ();
                } else {
                    println!(
                        "{},{}坐标,{}的RGB为{:?}和{:?}不匹配，等待一秒后再对比",
                        x, y, desc, image_rgb, rgb
                    );
                    sleep(1000);
                    self.until_check_rgb(x, y, rgb, desc);
                }
            }
            None => {
                println!("获取不到{}, 等待一秒后再对比", desc);
                sleep(1000);
                self.until_check_rgb(x, y, rgb, desc);
            }
        }
    }

    pub fn until_check_same_rgb(&self, list: Vec<CheckRGB>) {
        let rect = self.get_rect();
        let img = rect.get_image();
        let mut same = true;

        for item in list.iter() {
            match img.get_rgb_from_bitmap(item.x, item.y) {
                Some(image_rgb) => {
                    if image_rgb[0] != item.rgb[0]
                        || image_rgb[1] != item.rgb[1]
                        || image_rgb[2] != item.rgb[2]
                    {
                        println!(
                            "{},{}坐标,{}的RGB为{:?}和{:?}不匹配，等待一秒后再对比",
                            item.x, item.y, item.desc, image_rgb, item.rgb
                        );
                        same = false;
                    }
                }
                None => {
                    println!("获取不到{}, 等待一秒后再对比", item.desc);
                    same = false;
                }
            }
        }

        if !same {
            sleep(1000);
            self.until_check_same_rgb(list);
        }
    }

    pub fn image_save(&self, path: &str) {
        self.get_rect().get_image().save(path);
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

    pub fn key_click(&mut self, c: char) {
        self.mouse.key_click(Key::Layout(c));
    }
}
