use std::collections::btree_map::Values;

use crate::{types::data, err::C8Err};

pub struct Memory {
    vector : Vec<data>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            vector : vec![0; 512]
        }
    }

    pub fn get(&self, index: usize) -> Result<data, C8Err> {
        if index * 8 < 512 {
            Err(C8Err::MEMORY_UNACCESSIBLE)
        } else {
            match self.vector.get(index) {
                Some(value) => Ok(*value),
                None => Err(C8Err::MEMORY_UNACCESSIBLE)
            }
        }
    }
}