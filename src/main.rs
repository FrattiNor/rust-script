mod imagee;
mod rect;
mod utils;
mod win;

use utils::create_dir;
use win::CheckRGB;
use win::Win;

fn main() {
    match Win::new("*无标题 - 记事本") {
        Some(mut win) => {
            create_dir("assets");

            win.show_win();

            win.image_save("assets/1.png");

            win.until_check_same_rgb(vec![
                CheckRGB {
                    x: 100,
                    y: 100,
                    rgb: [30, 30, 30],
                    desc: "test",
                },
                CheckRGB {
                    x: 101,
                    y: 101,
                    rgb: [30, 30, 30],
                    desc: "test2",
                },
            ]);

            win.key_click('a');
        }
        None => (),
    }
}
