use crate::io::{Screen, self};
use crate::mem::{Memory};

use crate::stack::Stack;
use crate::{types::*, timer::Timer};

pub struct Chip {
        pc          : AddressLong, // current address
        i           : AddressLong, // stores memory addresses
        sp          : AddressShort, // stack pointer
        delay_t     : Timer,
        sound_t     : Timer,
        registers   : [Data; 16],
        stack       : Stack,
        memory      : Memory,
    pub screen      : Screen
}

impl Chip {
    pub fn new() -> Chip {
        Chip { pc: 512, i: 0, sp: 0, delay_t: Timer::new(), sound_t: Timer::new(), registers: [0; 16], stack: Stack::new(), memory: Memory::new(), screen: Screen::new() }
    }

    pub fn start(&mut self) {
        self.delay_t.set(100);
        self.sound_t.set(100);
        self.delay_t.start();
        self.sound_t.start();
    }

    pub fn dump(&self) {
        println!("\n================");
        println!("Chip-8 Debug Dump");
        println!("Program Counter: {:x?}", self.pc);
        println!("I (Memory addresses): {:x?}", self.i);
        println!("Stack Pointer: {:x?}", self.sp);
        println!("Delay Timer: {:x?}", self.delay_t.get());
        println!("Sound Timer: {:x?}", self.sound_t.get());
        println!("Registers: {:x?}", self.registers);
        println!("Stack: {:x?}", self.stack);
        //println!("Memory: {:?}", self.memory);
        println!("================\n");
    } 

    pub fn cycle(&mut self) {
        // fetch
        let read = self.read2();
        // execute 
        self.execute(read);
    }

    fn read(&mut self) -> Data {
        let val = self.memory.get(self.pc as usize).unwrap();
        //println!("{}: {val}", self.pc);
        self.pc += 1;
        val as Data
    }

    fn read2(&mut self) -> AddressLong {
        print!("PC: {:x?} \t", self.pc);
        let first = self.read();
        let second = self.read();
        let first = (first as u16) << 8;
        let combined = first + second as u16;
        println!("{:x?}", combined);
        combined
    }

    fn read_sprite(from: AddressLong, mem_vec: &Memory, amount: Data) -> &[u8] {
        &mem_vec.vector[(from as usize) .. ((from+(amount as u16)) as usize)]
    }

    fn execute(&mut self, instr: AddressLong) {
        match instr {
            // Clear screen 
            0x00E0 => {self.screen.clear();}
            // Jump
            0x1000..=0x1FFF => {
                self.pc = instr - 0x1000;
            }
            // Set Register
            0x6000..=0x6FFF => {
                let reg = ((instr & 0x0F00) >> 8) as u8;
                let val = (instr & 0x00FF) as u8;
                *self.registers.get_mut(reg as usize).unwrap() = val;
            }
            // Add register
            0x7000..=0x7FFF => {
                let reg = ((instr & 0x0F00) >> 8) as u8;
                let val = (instr & 0x00FF) as u8;
                *self.registers.get_mut(reg as usize).unwrap() += val;
            }
            // Set register I
            0xA000..=0xAFFF => {
                self.i = instr & 0x0FFF;
            }
            // Draw
            0xD000..=0xDFFF => {
                let x = self.registers.get(((instr & 0x0F00) >> 8) as usize).unwrap();
                let y = self.registers.get(((instr & 0x00f0) >> 4) as usize).unwrap();
                let nib = (instr & 0x000F) as u8;
                let sprite = Chip::read_sprite(self.i, &self.memory, nib);
                println!("x: {:x?}, y: {:x?}, amount: {}, sprite: {:x?}", x, y, nib, sprite);
                self.screen.draw(*x as usize, *y as usize, sprite);
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