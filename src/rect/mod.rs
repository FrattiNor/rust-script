use crate::imagee::Image;
use winapi::shared::windef::HWND__;
use winapi::shared::windef::{POINT, RECT};
use winapi::um::winuser::{ClientToScreen, GetClientRect};

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

    pub fn get_image(&self) -> Image {
        Image::new(self)
    }

    // pub fn get_rgb_from_bitmap(&self, x: u32, y: u32) -> Option<[u8; 3]> {
    //     Image::new(self).get_rgb_from_bitmap(x, y)
    // }

    // pub fn until_check_rgb(&self, x: u32, y: u32, rgb: [u8; 3]) {
    //     match self.get_rgb_from_bitmap(x, y) {
    //         Some(image_rgb) => {
    //             if image_rgb[0] == rgb[0] && image_rgb[1] == rgb[1] && image_rgb[2] == rgb[2] {
    //                 return ();
    //             } else {
    //                 println!(
    //                     "{},{}坐标的RGB为{:?},和{:?}不匹配，等待一秒后再对比",
    //                     x, y, image_rgb, rgb
    //                 );
    //                 sleep(1000);
    //                 self.until_check_rgb(x, y, rgb);
    //             }
    //         }
    //         None => {
    //             println!("获取不到{},{}坐标的RGB,等待一秒后再对比", x, y);
    //             sleep(1000);
    //             self.until_check_rgb(x, y, rgb);
    //         }
    //     }
    // }
}
