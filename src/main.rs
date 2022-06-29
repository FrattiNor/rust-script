mod imagee;
mod rect;
mod utils;
mod win;

use utils::create_dir;
use win::CheckRGB;
use win::Win;

const MAX_CARD_NUMBER: usize = 6;

fn create_user() {}

fn receive_gift() {}

fn draw_card() -> usize {
    1
}

fn delete_user() {}

fn main() {
    match Win::new("*无标题 - 记事本") {
        Some(mut win) => {
            create_dir("assets");

            win.show_win();

            win.image_save("assets/test.png");

            loop {
                create_user();
                receive_gift();
                let card_number = draw_card();
                if card_number > MAX_CARD_NUMBER {
                    break;
                }
                delete_user();
            }

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
