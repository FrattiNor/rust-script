mod imagee;
mod utils;
mod win;

use crate::imagee::Image;
use crate::utils::create_dir;
use crate::win::Win;

fn main() {
    match Win::new("Foxmail") {
        Some(mut win) => {
            win.show_win();

            let img = Image::new(win.get_rect());

            println!("{:?}", img.get_rgb_from_bitmap(100, 100));

            create_dir("ok");

            img.save("ok/d.png");

            let r = win.get_rect();
            win.mouse_move(r.left_i32 + r.width_i32 - 20, r.top_i32 + 200);
            win.mouse_scroll_y(-10);
        }
        None => (),
    }
}
