use crate::mem::Memory;

use crate::stack::Stack;
use crate::types::*;

pub struct Processor {
    pc          : address_long, // current address
    i           : address_long, // stores memory addresses
    sp          : address_short, // stack pointer
    delay_t     : data,
    sound_t     : data,
    registers   : Vec<data>,
    stack       : Stack,
    memory      : Memory
}