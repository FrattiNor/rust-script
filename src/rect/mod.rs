use winapi::shared::windef::{HWND__, POINT, RECT};
use winapi::um::winuser::{ClientToScreen, GetClientRect};

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
