use std::{error::Error, thread, time::Duration, fmt::format};

use minifb::{Window, Scale, WindowOptions, Key};

use crate::types::Data;

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
            screen: [[true; WIDTH]; HEIGHT],
            window: Window::new(
                "Test - ESC to exit",
                WIDTH,
                HEIGHT,
                WindowOptions { scale: Scale::X16, ..Default::default() },
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
                Ok(())
            },
            None => {
                Err(())
            },
        }
    }   

    pub fn clear(&mut self){
        self.screen = [[false; WIDTH]; HEIGHT];
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[Data]) {
        let sprite_cols : Vec<[bool; 8]> = sprite.iter().map(|x| -> [bool; 8] {Screen::u8_to_bools(x)}).collect();
        
        for i in 0..sprite.len() {
            println!("{}, {}", i+y, x);
            let col = &mut self.screen[i+y];
            let to_paint = &mut col[x..x+8];
            for (k, val) in (*to_paint).iter_mut().enumerate(){
                *val = *sprite_cols.get(i).unwrap().get(k).unwrap();
            }
        }
    }

    fn u8_to_bools(val: &u8) -> [bool; 8] {
        std::array::from_fn(|i| 1 << (7 - i) & val != 0) 
    }

    pub fn to_buffer(&self) -> Vec<u32> { // TODO
        let mut vec = Vec::<u32>::new();
        for row in self.screen {
            for val in row {
                if val {
                    vec.push(0xFFFFFF);
                } else {
                    vec.push(0);
                }
            }
        }
        vec
    }
}

pub fn load(filepath: &str) -> Vec<Data> {
    std::fs::read(filepath).unwrap_or_else(|_| panic!("unable to read {filepath}"))
}