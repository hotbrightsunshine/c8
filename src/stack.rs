use crate::types::*;

const MAX : usize = 216;

pub struct Stack {
    vector : Vec<address>
}

impl Stack {
    pub fn new () -> Stack {
        Stack { vector: Vec::new() }
    }

    pub fn pop (&mut self) -> Option<address> {
        return self.vector.pop()
    }

    pub fn push (&mut self, data : address) {
        match self.vector.len()+1 {
            MAX..   => panic!("Stack Overflow"),
            _       => self.vector.push(data)
        }
    }
}