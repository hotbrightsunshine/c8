use std::collections::btree_map::Values;

use crate::{types::data, err::C8Err};
#[derive(Debug)]
pub struct Memory {
    vector : Vec<data>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            vector : vec![0; 4096]
        }
    }

    pub fn get(&self, index: usize) -> Result<data, C8Err> {
        if index < 512 {
            Err(C8Err::MEMORY_UNACCESSIBLE)
        } else {
            match self.vector.get(index) {
                Some(value) => Ok(*value),
                None => Err(C8Err::MEMORY_UNACCESSIBLE)
            }
        }
    }

    pub fn write(&mut self, v:data, index:usize) {
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