use crate::mem::Memory;

use crate::stack::Stack;
use crate::types::*;

pub struct Processor {
    pc          : address,
    i           : address,
    delay_t     : data,
    sound_t     : data,
    registers   : Vec<u8>,
    stack       : Stack
}