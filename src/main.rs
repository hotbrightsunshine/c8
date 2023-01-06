pub mod c8;
pub mod mem;
pub mod types;
pub mod stack;
pub mod err;
pub mod timer;
pub mod io;

use c8::Chip;
//use io::Screen;
use minifb::{Key};

fn main() {
    let mut chip = Chip::new();
    chip.load("run/ibm.ch8");
    chip.start();
    while chip.screen.window.is_open() && !chip.screen.window.is_key_down(Key::Escape) {
        chip.cycle();
        chip.screen.window.update_with_buffer(&chip.screen.to_buffer(), io::WIDTH, io::HEIGHT).unwrap();
    }
}
