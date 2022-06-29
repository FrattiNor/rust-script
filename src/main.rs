mod image;
mod rect;
mod utils;
use crate::image::Image;
use crate::rect::Rect;
use crate::utils::{create_dir, open_window};

fn main() {
    match open_window("无标题 - 记事本") {
        Some(window_handle) => {
            let rect = Rect::new(window_handle);

            let img = Image::new(rect);

            create_dir("ok");

            img.save("ok/d.png");
        }
        None => (),
    }
}
