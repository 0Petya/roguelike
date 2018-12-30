extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

fn main() {
    tcod::system::set_fps(LIMIT_FPS);
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike")
        .init();

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.clear();
        root.put_char(1, 1, '@', BackgroundFlag::None);
        root.flush();
        root.wait_for_keypress(true);
    }
}
