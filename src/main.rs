use image::{ImageBuffer, Rgb};
use std::ffi::OsStr;
use std::iter::once;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::time::Duration;
use std::{fs, thread};
use winapi::ctypes::c_void;
use winapi::shared::windef::{HWND__, POINT, RECT};
use winapi::um::wingdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteObject, GetDIBits, GetObjectW,
    SelectObject, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, SRCCOPY,
};
use winapi::um::winuser::{
    ClientToScreen, FindWindowW, GetClientRect, GetDC, ReleaseDC, SetForegroundWindow, ShowWindow,
    SW_RESTORE,
};

pub fn encode_wide(s: String) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(&s).encode_wide().chain(once(0)).collect();
    wide
}

struct Rect {
    width_u32: u32,
    height_u32: u32,
    width_i32: i32,
    height_i32: i32,
    top_i32: i32,
    left_i32: i32,
}

fn open_window(title: &str) -> *mut HWND__ {
    let window_handle = unsafe {
        // 根据名称获取窗口句柄
        let window_handle = FindWindowW(null_mut(), encode_wide(String::from(title)).as_ptr());

        // 判断句柄是否获取到
        if window_handle.is_null() {
            panic!("未找到窗口");
        } else {
            println!("找到窗口");
        }

        ShowWindow(window_handle, SW_RESTORE);

        thread::sleep(Duration::from_millis(100));

        SetForegroundWindow(window_handle);

        thread::sleep(Duration::from_millis(100));

        window_handle
    };
    window_handle
}

fn get_window_rect(window: *mut HWND__) -> Rect {
    let res = unsafe {
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
            width_i32: width,
            height_i32: height,
            top_i32: top,
            left_i32: left,
        };

        res
    };
    res
}

fn get_img_by_rect(r: Rect) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let img = unsafe {
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

        let mut bitmap: BITMAP = BITMAP {
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

        let bitmap_size: usize = (((bitmap.bmWidth * 32 + 31) / 32) * 4 * bitmap.bmHeight) as usize;

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

        let img = ImageBuffer::from_fn(r.width_u32, r.height_u32, move |x, y| {
            let y = r.height_u32 - y - 1;
            let b = buffer[((y * r.width_u32 + x) * 4 + 0) as usize];
            let g = buffer[((y * r.width_u32 + x) * 4 + 1) as usize];
            let r = buffer[((y * r.width_u32 + x) * 4 + 2) as usize];
            Rgb([r, g, b])
        });

        img
    };
    img
}

fn create_dir(dir: &str) {
    match fs::create_dir(dir) {
        Ok(_) => (),
        Err(_) => (),
    }
}

fn main() {
    let window_handle = open_window("10");

    let rect = get_window_rect(window_handle);

    let img = get_img_by_rect(rect);

    create_dir("ok");

    img.save("ok/c.png").unwrap();
}
