use rand::Rng;

use crate::decoder;
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
        self.memory.load_font();
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
        // fetch + decode
        let read = decoder::decode(self.read2());
        println!("Instruction: {:x?}", read);
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
        &mem_vec.vector[(from as usize) .. (from as usize + amount as usize)]
    }

    fn execute(&mut self, instr: decoder::Instruction){
        match instr {
            decoder::Instruction::Cls => { self.screen.clear(); },
            decoder::Instruction::Ret => {                
                let val = self.stack.pop().expect("unable to pop values from stack");
                self.pc = val; 
            },

            decoder::Instruction::Jump { location } => { self.pc = location; },
            decoder::Instruction::Call { location } => {
                self.stack.push(self.pc);
                self.pc = location;
            },

            decoder::Instruction::SkipEqualRegisterBytes { register_index, bytes } => {
                if *self.registers.get(register_index as usize).unwrap() == bytes {
                    self.pc += 2;
                }
            },

            decoder::Instruction::SkipNotEqualRegisterBytes { register_index, bytes } => {
                if *self.registers.get(register_index as usize).unwrap() != bytes {
                    self.pc += 2;
                }
            },

            decoder::Instruction::SkipEqualRegisterRegister { register_x, register_y } => {
                let register_x = self.registers.get(register_x as usize).unwrap();
                let register_y = self.registers.get(register_y as usize).unwrap();
                if register_x == register_y {
                    self.pc += 2;
                }
            },

            decoder::Instruction::SetRegisterToBytes { register, bytes } => {
                *self.registers.get_mut(register as usize).unwrap() = bytes;
            },

            decoder::Instruction::AddBytesToRegister { register, bytes } => {
                *self.registers.get_mut(register as usize).unwrap() += bytes;
            },

            decoder::Instruction::SetRegisterToRegister { register_x, register_y } => {
                * self.registers.get_mut(register_x as usize).unwrap() = 
                            * self.registers.get(register_y as usize).unwrap();
            },

            decoder::Instruction::BitwiseOr { register_x, register_y } => {
                let regxval = *self.registers.get(register_x as usize).unwrap();
                        let regyval = *self.registers.get(register_y as usize).unwrap();
                        *self.registers.get_mut(register_x as usize).unwrap() = 
                            regxval | regyval;
            },

            decoder::Instruction::BitwiseAnd { register_x, register_y } => {
                let regxval = *self.registers.get(register_x as usize).unwrap();
                        let regyval = *self.registers.get(register_y as usize).unwrap();
                        *self.registers.get_mut(register_x as usize).unwrap() = 
                            regxval & regyval;
            },

            decoder::Instruction::BitwiseXor { register_x, register_y } => {
                let regxval = *self.registers.get(register_x as usize).unwrap();
                        let regyval = *self.registers.get(register_y as usize).unwrap();
                        *self.registers.get_mut(register_x as usize).unwrap() = 
                            regxval ^ regyval;
            },

            decoder::Instruction::AddRegisterToRegister { register_x, register_y } => {
                let regxval = *self.registers.get(register_x as usize).unwrap() as usize;
                let regyval = *self.registers.get(register_y as usize).unwrap() as usize;
                
                let result = regxval + regyval;
                if result > 0xFF {
                    *self.registers.get_mut(register_x as usize).unwrap() = result as Data;
                    *self.registers.get_mut(0xF as usize).unwrap() = (result - 0xFF) as Data;
                } else {
                    *self.registers.get_mut(register_y as usize).unwrap() = result as Data;
                }
            },

            decoder::Instruction::SubtractRegisterToRegister { register_x, register_y } => {
                let regxval = *self.registers.get(register_x as usize).unwrap() as usize;
                let regyval = *self.registers.get(register_y as usize).unwrap() as usize;
                
                let result = if regxval > regyval {
                    *self.registers.get_mut(0xF as usize).unwrap() = 1 as Data;
                    (regxval - regyval) as Data
                } else {
                    *self.registers.get_mut(0xF as usize).unwrap() = 0 as Data;
                    0 as Data
                };

                *self.registers.get_mut(register_x as usize).unwrap() = result;
            },

            decoder::Instruction::LeastSignificantBit { register } => {
                let regxval = *self.registers.get(register as usize).unwrap() as usize;
                if regxval % 2 == 1 {
                    *self.registers.get_mut(0xF as usize).unwrap() = 1 as Data;
                } else {
                    *self.registers.get_mut(0xF as usize).unwrap() = 0 as Data;
                }
                *self.registers.get_mut(register as usize).unwrap() = (regxval / 2) as Data;
            },

            decoder::Instruction::SubtractInversed { register_x, register_y } => {
                let regxval = *self.registers.get(register_x as usize).unwrap() as usize;
                let regyval = *self.registers.get(register_y as usize).unwrap() as usize;
                
                let result = if regyval > regxval {
                    *self.registers.get_mut(0xF as usize).unwrap() = 1 as Data;
                    (regyval - regxval) as Data
                } else {
                    *self.registers.get_mut(0xF as usize).unwrap() = 0 as Data;
                    0 as Data
                };

                *self.registers.get_mut(register_x as usize).unwrap() = result;
            },

            decoder::Instruction::MostSignificantBit { register } => {
                let regxval = *self.registers.get(register as usize).unwrap() as usize;
                if (regxval >> 7) == 0b1  {
                    *self.registers.get_mut(0xF as usize).unwrap() = 1 as Data;
                } else {
                    *self.registers.get_mut(0xF as usize).unwrap() = 0 as Data;
                }
                *self.registers.get_mut(register as usize).unwrap() = (regxval * 2) as Data;
            },

            decoder::Instruction::SkipNotEqualRegisterRegister { register_x, register_y } => {
                let register_x = self.registers.get(register_x as usize).unwrap();
                let register_y = self.registers.get(register_y as usize).unwrap();
                if register_x != register_y {
                    self.pc += 2;
                }
            },
            decoder::Instruction::SetI { value } => {
                self.i = value;
            },
            decoder::Instruction::JumpToLocationPlusZeroRegister { address } => {
                self.pc = address + (*self.registers.first().unwrap() as u16);
            },
            decoder::Instruction::Random { register, value } => {
                let x :u8 = rand::thread_rng().gen_range(0..=255) & value;
                *self.registers.get_mut(register as usize).unwrap() = x;
            },
            decoder::Instruction::Display { register_x, register_y, nibble } => {
                let x = self.registers.get((register_x as usize)).unwrap();
                let y = self.registers.get(register_y as usize).unwrap();
                let sprite = Chip::read_sprite(self.i, &self.memory, nibble);
                self.screen.draw(*x as usize, *y as usize, sprite);
            },
            decoder::Instruction::SkipIfKeyIsPressed { register } => {
                /*
                Skip next instruction if key with the value of Vx is pressed.
                Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2. 
                */
                let key_wanted = *self.registers.get(register as usize).unwrap();
                if self.screen.window.is_key_down(io::u8_to_key(key_wanted)) {
                    self.pc += 2;
                }
            },
            decoder::Instruction::SkipIfKeyIsNotPressed { register } => {
                let key_wanted = *self.registers.get(register as usize).unwrap();
                if !self.screen.window.is_key_down(io::u8_to_key(key_wanted)) {
                    self.pc += 2;
                }
            },
            decoder::Instruction::SetRegisterToDelayTimer { register } => {
                *self.registers.get_mut(register as usize).unwrap() = self.delay_t.get();
            },
            decoder::Instruction::WaitForKey { register } => {
                let key = self.screen.wait_for_key(); // blocking
                *self.registers.get_mut(register as usize).unwrap() = key
            },
            decoder::Instruction::SetDelayTimer { register } => {
                self.delay_t.set(
                    *self.registers.get(register as usize).unwrap() as Data
                )
            },
            decoder::Instruction::SetSoundTimer { register } => {
                self.sound_t.set(
                    *self.registers.get(register as usize).unwrap() as Data
                )
            },
            decoder::Instruction::AddRegisterToI { register } => {
                self.i += *self.registers.get(register as usize).unwrap() as u16;
            },
            decoder::Instruction::SetIToLocationOfSprite { register } => {
                let ch = *self.registers.get(register as usize).unwrap();
                self.i = self.memory.get_font(ch) as u16;
            },
            decoder::Instruction::StoreBCD { register } => {
                // Highly inspired by:
                // https://github.com/taniarascia/chip8/blob/master/classes/CPU.js
                let mut x = *self.registers.get(register as usize).unwrap();
                let a = x / 100;
                x -= a * 100;
                let b = x / 10;
                x -= b * 10;
                let c = x;

                self.memory.write(a, self.i as usize);
                self.memory.write(b, (self.i + 1) as usize);
                self.memory.write(c, (self.i + 2) as usize);
            },
            decoder::Instruction::StoreRegistersToMemory { to_register } => {
                for i in 0..=(to_register as usize) {
                    self.memory.write(
                        *self.registers.get(i as usize).unwrap(),
                        self.i as usize + i)
                }
            },
            decoder::Instruction::LoadRegistersFromMemory { to_register } => {
                for i in 0..=(to_register as usize) {
                    *self.registers.get_mut(i).unwrap() = 
                        self.memory.get(self.i as usize + i).unwrap()
                }
            },
            decoder::Instruction::Invalid => {
                panic!("Invalid instruction");
            },
        }
    }

    pub fn load(&mut self, filepath :&str) {
        let buffer = io::load(filepath);
        for (i, val) in buffer.iter().enumerate() {
            self.memory.write(*val, i+512);
        }
    }

}

