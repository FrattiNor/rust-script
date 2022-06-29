use crate::rect::Rect;
use image::{ImageBuffer, Rgb};
use std::mem::size_of;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::um::wingdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteObject, GetDIBits, GetObjectW,
    SelectObject, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, SRCCOPY,
};
use winapi::um::winuser::{GetDC, ReleaseDC};

pub struct Image<'a> {
    bitmap: Vec<u8>,
    rect: &'a Rect,
}

impl<'a> Image<'a> {
    #[cfg(windows)]
    pub fn new(r: &'a Rect) -> Self {
        unsafe {
            let dc_window = GetDC(null_mut());

            let dc_mem = CreateCompatibleDC(dc_window);

            let hbm = CreateCompatibleBitmap(dc_window, r.width_i32, r.height_i32);

            SelectObject(dc_mem, hbm as *mut c_void);

            BitBlt(
                dc_mem,
                0,
                0,
                r.width_i32,
                r.height_i32,
                dc_window,
                r.left_i32,
                r.top_i32,
                SRCCOPY,
            );

            let mut bitmap = BITMAP {
                bmBits: 0 as *mut c_void,
                bmBitsPixel: 0,
                bmPlanes: 0,
                bmWidthBytes: 0,
                bmHeight: 0,
                bmWidth: 0,
                bmType: 0,
            };

            GetObjectW(
                hbm as *mut c_void,
                size_of::<BITMAP>() as i32,
                (&mut bitmap) as *mut BITMAP as *mut c_void,
            );

            let mut bi = BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: bitmap.bmWidth,
                biHeight: bitmap.bmHeight,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            };

            let bitmap_size: usize =
                (((bitmap.bmWidth * 32 + 31) / 32) * 4 * bitmap.bmHeight) as usize;

            let mut buffer: Vec<u8> = vec![0; bitmap_size];

            GetDIBits(
                dc_window,
                hbm,
                0,
                bitmap.bmHeight as u32,
                buffer.as_mut_ptr() as *mut c_void,
                (&mut bi) as *mut BITMAPINFOHEADER as *mut BITMAPINFO,
                DIB_RGB_COLORS,
            );

            DeleteObject(hbm as *mut c_void);
            DeleteObject(dc_mem as *mut c_void);
            ReleaseDC(null_mut(), dc_window);

            Image {
                bitmap: buffer,
                rect: r,
            }
        }
    }

    pub fn get_rgb_from_bitmap(&self, x: u32, y: u32) -> Option<[u8; 3]> {
        let len = self.bitmap.len();
        let y = self.rect.height_i32 - y as i32 - 1;

        if y < 0 {
            return None;
        }

        let b_index = (((y as u32) * self.rect.width_u32 + x) * 4 + 0) as usize;
        let g_index = (((y as u32) * self.rect.width_u32 + x) * 4 + 1) as usize;
        let r_index = (((y as u32) * self.rect.width_u32 + x) * 4 + 2) as usize;

        if b_index > len || g_index > len || r_index > len {
            return None;
        }

        let b = self.bitmap[b_index];
        let g = self.bitmap[g_index];
        let r = self.bitmap[r_index];
        Some([r, g, b])
    }

    pub fn save(&self, path: &str) {
        let img =
            ImageBuffer::from_fn(
                self.rect.width_u32,
                self.rect.height_u32,
                move |x, y| match self.get_rgb_from_bitmap(x, y) {
                    Some(v) => Rgb(v),
                    None => Rgb([0, 0, 0]),
                },
            );

        match img.save(path) {
            Ok(_) => (),
            Err(_) => {
                println!("图片保存失败");
            }
        }
    }
}
