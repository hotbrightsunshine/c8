use std::error::Error;

pub const HEIGHT : usize = 32;
pub const WIDTH : usize = 64;

#[derive(Debug)]
pub struct Screen {
    /// False is `off`, True is `on`
    screen: [[bool; WIDTH]; HEIGHT]
}

impl Screen {
    pub fn new() -> Screen {
        Screen { screen: [[false; WIDTH]; HEIGHT] }
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
