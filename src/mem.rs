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