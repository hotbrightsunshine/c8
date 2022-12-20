use crate::types::data;

pub struct Memory {
    vector : Vec<data>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            vector : vec![0; 512]
        }
    }
}