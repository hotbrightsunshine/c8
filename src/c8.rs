use crate::io::{Screen, self};
use crate::mem::Memory;

use crate::stack::Stack;
use crate::{types::*, timer::Timer};

pub struct Chip {
        pc          : address_long, // current address
        i           : address_long, // stores memory addresses
        sp          : address_short, // stack pointer
        delay_t     : Timer,
        sound_t     : Timer,
        registers   : Vec<data>,
        stack       : Stack,
        memory      : Memory,
    pub screen      : Screen
}

impl Chip {
    pub fn new() -> Chip {
        Chip { pc: 512, i: 0, sp: 0, delay_t: Timer::new(), sound_t: Timer::new(), registers: Vec::with_capacity(16), stack: Stack::new(), memory: Memory::new(), screen: Screen::new() }
    }

    pub fn start(&mut self) -> () {
        self.delay_t.set(100);
        self.sound_t.set(100);
        self.delay_t.start();
        self.sound_t.start();
    }

    pub fn dump(&self) {
        println!("\n================");
        println!("Chip-8 Debug Dump");
        println!("Program Counter: {}", self.pc);
        println!("I (Memory addresses): {}", self.i);
        println!("Stack Pointer: {}", self.sp);
        println!("Delay Timer: {}", self.delay_t.get());
        println!("Sound Timer: {}", self.sound_t.get());
        println!("Registers: {:?}", self.registers);
        println!("Stack: {:?}", self.stack);
        //println!("Memory: {:?}", self.memory);
        println!("================\n");
    } 

    pub fn cycle(&mut self) {
        // fetch
        let read = self.read2();
        // execute 
        self.execute(read);
    }

    fn read(&mut self) -> data {
        let val = self.memory.get(self.pc as usize).unwrap();
        //println!("{}: {val}", self.pc);
        self.pc = self.pc + 1;
        val as data
    }

    fn read2(&mut self) -> address_long {
        print!("PC: {:x?} \t", self.pc);
        let first = self.read();
        let second = self.read();
        let first = (first as u16) << 8;
        let combined = first + second as u16;
        println!("{:x?}", combined);
        combined
    }

    fn execute(&mut self, instr: address_long) {
        match instr {
            0x00E0 => {self.screen.clear();}
            0x1000..=0x1FFF => {
                self.pc = instr - 0x1000;
            }
            0x6000..=0x6FFF => {
                let reg = ((instr & 0x0F00) >> 8) as u8;
                let val = (instr & 0x00FF) as u8;
                *self.registers.get_mut(reg as usize).unwrap() = val;
            }
            _ => {}
        }
    }

    pub fn load(&mut self, filepath :&str) {
        let buffer = io::load(filepath);
        for (i, val) in buffer.iter().enumerate() {
            self.memory.write(*val, i+512);
        }
    }
}