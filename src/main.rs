extern crate tcod;

use tcod::RootConsole;
// use tcod::{Console, RootConsole, FontLayout, FontType};

fn main() {
    // let mut root = RootConsole::initializer()
    //     .font("terminal16x16_gs_ro.png", FontLayout::Tcod)
    //     .font_type(FontType::Greyscale)
    //     .font_dimensions(32, 60) // angband16x16.bmp has 32 columns & 60 rows of
    //                              // characters
    //     .size(MAP_WIDTH, MAP_HEIGHT)
    //     .title("Using custom character mapping with libtcod")
    //     .init();
    let mut root = RootConsole::initializer().size(80, 50).title("Minimal libtcod loop").init();

    while !root.window_closed() {
        root.flush();
        let key = root.wait_for_keypress(true);
        println!("Pressed key: {:?}", key);
    }
}
