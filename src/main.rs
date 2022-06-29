mod imagee;
mod rect;
mod utils;
mod win;

use utils::{create_dir, sleep};
use win::Win;
use win::{CheckRGB, Padding};

const MAX_CARD_NUMBER: usize = 6;

fn tap_arrow(win: &mut Win) {
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 635,
            y: 1195,
            rgb: [255, 255, 255],
            desc: "开始menu",
        },
        CheckRGB {
            x: 675,
            y: 1195,
            rgb: [236, 236, 236],
            desc: "开始menu",
        },
        CheckRGB {
            x: 635,
            y: 1235,
            rgb: [236, 236, 236],
            desc: "开始menu",
        },
        CheckRGB {
            x: 675,
            y: 1235,
            rgb: [255, 255, 255],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(675, 1235);
    win.mouse_click("left");
}

fn create_user(win: &mut Win) {
    // 进入游戏
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 645,
            y: 1207,
            rgb: [255, 255, 255],
            desc: "开始menu",
        },
        CheckRGB {
            x: 685,
            y: 1207,
            rgb: [255, 255, 255],
            desc: "开始menu",
        },
        CheckRGB {
            x: 645,
            y: 1250,
            rgb: [236, 236, 236],
            desc: "开始menu",
        },
        CheckRGB {
            x: 685,
            y: 1250,
            rgb: [255, 255, 255],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(600, 1235);
    win.mouse_click("left");

    // 跳过教程
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 400,
            y: 815,
            rgb: [158, 221, 8],
            desc: "开始menu",
        },
        CheckRGB {
            x: 400,
            y: 850,
            rgb: [109, 195, 12],
            desc: "开始menu",
        },
        CheckRGB {
            x: 630,
            y: 815,
            rgb: [138, 207, 4],
            desc: "开始menu",
        },
        CheckRGB {
            x: 630,
            y: 850,
            rgb: [148, 212, 5],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(630, 850);
    win.mouse_click("left");

    // 输入名称
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 250,
            y: 815,
            rgb: [100, 137, 5],
            desc: "开始menu",
        },
        CheckRGB {
            x: 250,
            y: 850,
            rgb: [69, 123, 8],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 815,
            rgb: [82, 125, 3],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 850,
            rgb: [88, 129, 2],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(225, 575);
    win.mouse_click("left");
    sleep(1000);
    win.key_click('a');
    sleep(1000);
    win.mouse_click("left");
    sleep(1000);
    win.mouse_move(460, 850);
    win.mouse_click("left");

    // ok
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 400,
            y: 815,
            rgb: [158, 221, 8],
            desc: "开始menu",
        },
        CheckRGB {
            x: 400,
            y: 850,
            rgb: [109, 195, 12],
            desc: "开始menu",
        },
        CheckRGB {
            x: 630,
            y: 815,
            rgb: [138, 207, 4],
            desc: "开始menu",
        },
        CheckRGB {
            x: 630,
            y: 850,
            rgb: [148, 212, 5],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(630, 850);
    win.mouse_click("left");

    tap_arrow(win);
    tap_arrow(win);
    tap_arrow(win);
    tap_arrow(win);

    // 关闭
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 250,
            y: 1160,
            rgb: [253, 253, 253],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 1160,
            rgb: [247, 246, 247],
            desc: "开始menu",
        },
        CheckRGB {
            x: 630,
            y: 1200,
            rgb: [249, 249, 249],
            desc: "开始menu",
        },
        CheckRGB {
            x: 250,
            y: 1200,
            rgb: [231, 229, 238],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(250, 1200);
    win.mouse_click("left");

    // 关闭
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 250,
            y: 815,
            rgb: [252, 252, 252],
            desc: "开始menu",
        },
        CheckRGB {
            x: 250,
            y: 850,
            rgb: [233, 230, 239],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 815,
            rgb: [247, 246, 248],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 850,
            rgb: [252, 252, 252],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(460, 850);
    win.mouse_click("left");

    // 关闭
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 250,
            y: 1160,
            rgb: [253, 253, 253],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 1160,
            rgb: [247, 246, 247],
            desc: "开始menu",
        },
        CheckRGB {
            x: 630,
            y: 1200,
            rgb: [249, 249, 249],
            desc: "开始menu",
        },
        CheckRGB {
            x: 250,
            y: 1200,
            rgb: [231, 229, 238],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(250, 1200);
    win.mouse_click("left");
}

fn receive_gift(win: &mut Win) {
    // 点开gift
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 640,
            y: 915,
            rgb: [230, 230, 230],
            desc: "开始menu",
        },
        CheckRGB {
            x: 650,
            y: 915,
            rgb: [221, 68, 102],
            desc: "开始menu",
        },
        CheckRGB {
            x: 665,
            y: 915,
            rgb: [230, 230, 230],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(650, 915);
    win.mouse_click("left");

    // 一起领取
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 400,
            y: 1160,
            rgb: [159, 222, 8],
            desc: "开始menu",
        },
        CheckRGB {
            x: 630,
            y: 1160,
            rgb: [138, 207, 4],
            desc: "开始menu",
        },
        CheckRGB {
            x: 400,
            y: 1200,
            rgb: [106, 195, 12],
            desc: "开始menu",
        },
        CheckRGB {
            x: 630,
            y: 1200,
            rgb: [149, 213, 4],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(630, 1200);
    win.mouse_click("left");

    // 关闭
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 250,
            y: 1160,
            rgb: [253, 253, 253],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 1160,
            rgb: [247, 246, 247],
            desc: "开始menu",
        },
        CheckRGB {
            x: 630,
            y: 1200,
            rgb: [249, 249, 249],
            desc: "开始menu",
        },
        CheckRGB {
            x: 250,
            y: 1200,
            rgb: [231, 229, 238],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(250, 1200);
    win.mouse_click("left");

    // 关闭
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 80,
            y: 1160,
            rgb: [253, 253, 253],
            desc: "开始menu",
        },
        CheckRGB {
            x: 80,
            y: 1200,
            rgb: [231, 231, 238],
            desc: "开始menu",
        },
        CheckRGB {
            x: 320,
            y: 1160,
            rgb: [253, 253, 253],
            desc: "开始menu",
        },
        CheckRGB {
            x: 320,
            y: 1200,
            rgb: [255, 255, 255],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(320, 1200);
    win.mouse_click("left");

    // 任务奖励
    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 650,
            y: 825,
            rgb: [221, 68, 68],
            desc: "开始menu",
        },
        CheckRGB {
            x: 615,
            y: 850,
            rgb: [255, 112, 16],
            desc: "开始menu",
        },
        CheckRGB {
            x: 690,
            y: 850,
            rgb: [255, 112, 16],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(650, 825);
    win.mouse_click("left");

    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 635,
            y: 1195,
            rgb: [255, 255, 255],
            desc: "开始menu",
        },
        CheckRGB {
            x: 675,
            y: 1195,
            rgb: [235, 235, 235],
            desc: "开始menu",
        },
        CheckRGB {
            x: 635,
            y: 1235,
            rgb: [237, 237, 237],
            desc: "开始menu",
        },
        CheckRGB {
            x: 675,
            y: 1235,
            rgb: [255, 255, 255],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(675, 1235);
    win.mouse_click("left");

    sleep(1000);

    win.mouse_move(270, 480);
    win.mouse_click("left");

    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 250,
            y: 1060,
            rgb: [158, 220, 8],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 1100,
            rgb: [139, 208, 4],
            desc: "开始menu",
        },
        CheckRGB {
            x: 250,
            y: 1060,
            rgb: [158, 220, 8],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 1100,
            rgb: [139, 208, 4],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(460, 1100);
    win.mouse_click("left");

    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 250,
            y: 815,
            rgb: [252, 252, 252],
            desc: "开始menu",
        },
        CheckRGB {
            x: 250,
            y: 850,
            rgb: [233, 230, 239],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 815,
            rgb: [247, 246, 248],
            desc: "开始menu",
        },
        CheckRGB {
            x: 460,
            y: 850,
            rgb: [252, 252, 252],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(460, 850);
    win.mouse_click("left");

    win.until_check_same_rgb(vec![
        CheckRGB {
            x: 622,
            y: 1209,
            rgb: [217, 102, 61],
            desc: "开始menu",
        },
        CheckRGB {
            x: 663,
            y: 1209,
            rgb: [217, 102, 61],
            desc: "开始menu",
        },
    ]);

    win.mouse_move(663, 1209);
    win.mouse_click("left");
}

fn draw_card() -> usize {
    1
}

fn delete_user() {}

fn main() {
    match Win::new(
        "雷电模拟器",
        Padding {
            top: 34,
            right: 41,
            bottom: 2,
            left: 1,
        },
    ) {
        Some(mut win) => {
            create_dir("assets");

            win.show_win();

            win.image_save("assets/test.png");

            win.until_check_same_rgb(vec![
                CheckRGB {
                    x: 148,
                    y: 37,
                    rgb: [251, 129, 97],
                    desc: "开始menu",
                },
                CheckRGB {
                    x: 211,
                    y: 37,
                    rgb: [255, 157, 69],
                    desc: "开始menu",
                },
            ]);
        
            win.mouse_move(26, 776);
            win.mouse_click("left");

            // create_user(win);
            // loop {
            //     create_user();
            //     receive_gift();
            //     let card_number = draw_card();
            //     if card_number > MAX_CARD_NUMBER {
            //         break;
            //     }
            //     delete_user();
            // }

            // win.until_check_same_rgb(vec![
            //     CheckRGB {
            //         x: 100,
            //         y: 100,
            //         rgb: [30, 30, 30],
            //         desc: "test",
            //     },
            //     CheckRGB {
            //         x: 101,
            //         y: 101,
            //         rgb: [30, 30, 30],
            //         desc: "test2",
            //     },
            // ]);

            // win.key_click('a');
        }
        None => (),
    }
}
