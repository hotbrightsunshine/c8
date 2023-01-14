use crate::{types::Data, err::C8Err};
#[derive(Debug)]
pub struct Memory {
    pub vector : Vec<Data>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            vector : vec![0; 4096]
        }
    }

    pub fn load_font(&mut self) {
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        let first_address:u8 = 0x050;
        for i in font {
            self.write(font[i as usize], (first_address + i) as usize);
        }
    }

    pub fn get(&self, index: usize) -> Result<Data, C8Err> {
        if index < 512 {
            Err(C8Err::MemoryUnaccessible)
        } else {
            match self.vector.get(index) {
                Some(value) => Ok(*value),
                None => Err(C8Err::MemoryUnaccessible)
            }
        }
    }

    pub fn write(&mut self, v:Data, index:usize) {
        if index < 512 {
            panic!("mem unaccessible")
        } else {
            match self.vector.get_mut(index) {
                Some(value) => *value = v,
                None => panic!("mem unaccessible")
            }
        }
    }
}