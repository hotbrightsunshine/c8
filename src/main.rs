pub mod c8;
pub mod mem;
pub mod types;
pub mod stack;
pub mod err;
pub mod timer;
pub mod io;

use std::{time::Duration, thread, str::SplitAsciiWhitespace};

use c8::Chip;
use io::Screen;
use minifb::{Window, WindowOptions, Key, Scale};

fn main() {
    let mut chip = Chip::new();
    chip.load("run/ibm.ch8");
    chip.start();
    while chip.screen.window.is_open() && !chip.screen.window.is_key_down(Key::Escape) {
        chip.cycle();
        thread::sleep(Duration::from_millis(100));
        chip.screen.window.update();
    }
}
