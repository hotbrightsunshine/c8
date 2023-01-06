use crate::types::*;

const MAX : usize = 16;

#[derive(Debug)]
pub struct Stack {
    vector : Vec<AddressLong>
}

impl Stack {
    pub fn new () -> Stack {
        Stack { vector: Vec::new() }
    }

    pub fn pop (&mut self) -> Option<AddressLong> {
        self.vector.pop()
    }

    pub fn push (&mut self, data : AddressLong) {
        match self.vector.len()+1 {
            MAX..   => panic!("Stack Overflow"),
            _       => self.vector.push(data)
        }
    }
}