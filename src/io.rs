use std::{error::Error, thread, time::Duration, fmt::format};

use minifb::{Window, Scale, WindowOptions, Key};

use crate::types::data;

pub const HEIGHT : usize = 32;
pub const WIDTH : usize = 64;

pub struct Screen {
    /// False is `off`, True is `on`
    screen: [[bool; WIDTH]; HEIGHT],
    pub window: Window,
}

impl Screen {
    pub fn new() -> Screen {
        let mut window_options = WindowOptions::default();
        window_options.scale = Scale::X16;

        let mut screen = Screen {
            screen: [[false; WIDTH]; HEIGHT],
            window: Window::new(
                "Test - ESC to exit",
                WIDTH,
                HEIGHT,
                window_options,
                )
                .unwrap_or_else(|e| {
                panic!("{}", e);
            })
        }; 

        screen.window.limit_update_rate
            (Some(std::time::Duration::from_micros(16600)));
        screen
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&bool> {
        self.screen.get(y)?
            .get(x)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        self.screen.get_mut(y)?.get_mut(x)
    }

    pub fn set(&mut self, value: bool, x: usize, y: usize) -> Result<(), ()> {
        match self.get_mut(x, y) {
            Some(x) => {
                *x = value;
                return Ok(());
            },
            None => {
                return Err(());
            },
        }
    }   
}

pub fn load(filepath: &str) -> Vec<data> {
    std::fs::read(filepath).expect(&format!("unable to read {filepath}"))
}
