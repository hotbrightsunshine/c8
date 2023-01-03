pub mod c8;
pub mod mem;
pub mod types;
pub mod stack;
pub mod err;
pub mod timer;

use std::{thread, time::Duration};

use c8::Chip;

fn main() {
    let mut chip = Chip::new();
    chip.start();
    for _ in 0..3 {
        chip.dump();
        thread::sleep(Duration::from_secs(1));
    }
}
