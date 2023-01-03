use crate::types::*;

const MAX : usize = 16;

pub struct Stack {
    vector : Vec<address_long>
}

impl Stack {
    pub fn new () -> Stack {
        Stack { vector: Vec::new() }
    }

    pub fn pop (&mut self) -> Option<address_long> {
        return self.vector.pop()
    }

    pub fn push (&mut self, data : address_long) {
        match self.vector.len()+1 {
            MAX..   => panic!("Stack Overflow"),
            _       => self.vector.push(data)
        }
    }
}