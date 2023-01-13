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
            decoder::Instruction::Display { register_x, register_y, nibble } => todo!(),
            decoder::Instruction::SkipIfKeyIsPressed { register } => todo!(),
            decoder::Instruction::SkipIfKeyIsNotPressed { register } => todo!(),
            decoder::Instruction::SetRegisterToDelayTimer { register } => todo!(),
            decoder::Instruction::WaitForKey { register } => todo!(),
            decoder::Instruction::SetDelayTimer { register } => todo!(),
            decoder::Instruction::SetSoundTimer { register } => todo!(),
            decoder::Instruction::AddRegisterToI { register } => todo!(),
            decoder::Instruction::SetIToLocationOfSprite { register } => todo!(),
            decoder::Instruction::StoreBCD { register } => todo!(),
            decoder::Instruction::StoreRegistersToMemory { to_register } => todo!(),
            decoder::Instruction::LoadRegistersFromMemory { to_register } => todo!(),
            decoder::Instruction::Invalid => todo!(),
        }
    }

    fn execute_old(&mut self, instr: AddressLong) {
        match instr {
            // Clear screen 
            0x00E0 => {self.screen.clear();}
            // Jump
            0x1000..=0x1FFF => {
                self.pc = instr - 0x1000;
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
            // Return
            0x00EE => {
                println!("RETURN CALLED");
                self.dump();
                let val = self.stack.pop().expect("unable to pop values from stack");
                self.pc = val;
                self.dump();
            }
            // Call
            0x2000..=0x2FFF => {
                let val = instr - 0x2000;
                self.stack.push(self.pc);
                self.pc = val;
            }
            // Skip next instruction if Vx = kk.
            0x3000..=0x3FFF => {
                println!("SE CALLED");
                let instr = instr - 0x3000;
                let reg = (instr & 0x0F00) >> 8;
                let val = (instr & 0x00FF) as u8;
                if *self.registers.get(reg as usize).unwrap() == val {
                    self.pc += 2;
                }
            }
            // Skip next instruction if Vx != kk.
            0x4000..=0x4FFF => {
                println!("SNE CALLED");
                self.dump();
                let instr = instr - 0x3000;
                let reg = (instr & 0x0F00) >> 8;
                let val = (instr & 0x00FF) as u8;
                if *self.registers.get(reg as usize).unwrap() != val {
                    self.pc += 2;
                }
                self.dump();
            }
            // Skip next instruction if Vx = Vy.
            0x5000..=0x5FF0 => {
                if instr & 0x000F != 0 { panic!("SE Vx, Vy (0x5xy0) NOT RECOGNIZED") }
                let reg1 = (instr & 0x0F00) >> 8;
                let reg2 = (instr & 0x00F0) >> 4;
                if reg1 == reg2 {
                    self.pc += 2;
                }
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
            // 0x8000 INSTRUCTIONS
            0x8000..=0x8FFF => {
                let regx = (instr & 0x0F00) >> 8;
                let regy = (instr & 0x00F0) >> 4;
                let param = (instr & 0x000F);
                match param {
                    0 => {
                        //Stores the value of register Vy in register Vx.
                        * self.registers.get_mut(regx as usize).unwrap() = 
                            * self.registers.get(regy as usize).unwrap();
                    }
                    1 => {
                        // Performs a bitwise OR on the values of Vx and Vy, 
                        // then stores the result in Vx. 

                        // clones it to prevent borrow checker errors
                        let regxval = *self.registers.get(regx as usize).unwrap();
                        let regyval = *self.registers.get(regy as usize).unwrap();
                        *self.registers.get_mut(regx as usize).unwrap() = 
                            regxval | regyval;
                    }
                    2 => {
                        // Performs a bitwise AND on the values of Vx and Vy, 
                        // then stores the result in Vx. 

                        // clones it to prevent borrow checker errors
                        let regxval = *self.registers.get(regx as usize).unwrap();
                        let regyval = *self.registers.get(regy as usize).unwrap();
                        *self.registers.get_mut(regx as usize).unwrap() = 
                            regxval & regyval;
                    }
                    3 => {
                        // Performs a bitwise AND on the values of Vx and Vy, 
                        // then stores the result in Vx. 

                        // clones it to prevent borrow checker errors
                        let regxval = *self.registers.get(regx as usize).unwrap();
                        let regyval = *self.registers.get(regy as usize).unwrap();
                        *self.registers.get_mut(regx as usize).unwrap() = 
                            regxval ^ regyval;
                    }
                    4 => {
                        // Set Vx = Vx + Vy, set VF = carry.
                        // The values of Vx and Vy are added together.
                        let regxval = *self.registers.get(regx as usize).unwrap() as usize;
                        let regyval = *self.registers.get(regy as usize).unwrap() as usize;
                        
                        let result = regxval + regyval;
                        if result > 0xFF {
                            *self.registers.get_mut(regx as usize).unwrap() = result as Data;
                            *self.registers.get_mut(0xF as usize).unwrap() = (result - 0xFF) as Data;
                        } else {
                            *self.registers.get_mut(regx as usize).unwrap() = result as Data;
                        }
                    }
                    5 => {
                        // If Vx > Vy, then VF is set to 1, otherwise 0. 
                        // Then Vy is subtracted from Vx, and the results stored in Vx.
                        let regxval = *self.registers.get(regx as usize).unwrap() as usize;
                        let regyval = *self.registers.get(regy as usize).unwrap() as usize;
                        
                        let result = if regxval > regyval {
                            *self.registers.get_mut(0xF as usize).unwrap() = 1 as Data;
                            (regxval - regyval) as Data
                        } else {
                            *self.registers.get_mut(0xF as usize).unwrap() = 0 as Data;
                            0 as Data
                        };

                        *self.registers.get_mut(regx as usize).unwrap() = result;
                    }
                    7 => {
                        //Set Vx = Vy - Vx, set VF = NOT borrow.
                        //If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                        let regxval = *self.registers.get(regx as usize).unwrap() as usize;
                        let regyval = *self.registers.get(regy as usize).unwrap() as usize;
                        
                        let result = if regyval > regxval {
                            *self.registers.get_mut(0xF as usize).unwrap() = 1 as Data;
                            (regyval - regxval) as Data
                        } else {
                            *self.registers.get_mut(0xF as usize).unwrap() = 0 as Data;
                            0 as Data
                        };

                        *self.registers.get_mut(regx as usize).unwrap() = result;
                    }
                    6 => {
                        // Set Vx = Vx SHR 1.
                        // If the least-significant bit of Vx is 1, 
                        // then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                        let regxval = *self.registers.get(regx as usize).unwrap() as usize;
                        if regxval % 2 == 1 {
                            *self.registers.get_mut(0xF as usize).unwrap() = 1 as Data;
                        } else {
                            *self.registers.get_mut(0xF as usize).unwrap() = 0 as Data;
                        }
                        *self.registers.get_mut(regx as usize).unwrap() = (regxval / 2) as Data;
                    }
                    0xE => {
                        // Set Vx = Vx SHL 1.
                        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. 
                        // Then Vx is multiplied by 2.
                        let regxval = *self.registers.get(regx as usize).unwrap() as usize;
                        if (regxval >> 7) == 0b1  {
                            *self.registers.get_mut(0xF as usize).unwrap() = 1 as Data;
                        } else {
                            *self.registers.get_mut(0xF as usize).unwrap() = 0 as Data;
                        }
                        *self.registers.get_mut(regx as usize).unwrap() = (regxval * 2) as Data;
                    } 
                    _ => {panic!("no such instruction!");}
                }
            }
            0x9000..=0x9FFF => {
                // Skip next instruction if Vx != Vy.
                // The values of Vx and Vy are compared, and if they are not equal, 
                // the program counter is increased by 2.
                if instr & 0x000F != 0 { panic!("SNE Vx, Vy (0x9xy0) NOT RECOGNIZED") }
                let reg1 = (instr & 0x0F00) >> 8;
                let reg2 = (instr & 0x00F0) >> 4;
                if reg1 != reg2 {
                    self.pc += 2;
                }
            }
            0xB000..=0xBFFF => {
                // Jump to location nnn + V0.
                //The program counter is set to nnn plus the value of V0.
                let val_v0 = *self.registers.get(0).unwrap();
                self.pc = (instr & 0x0FFF) + (val_v0 as u16);
            }
            0xC000..=0xCFFF => {
                // Set Vx = random byte AND kk.
                //The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. 
                //The results are stored in Vx. 
                //See instruction 8xy2 for more information on AND.

                let rand : u8 = rand::random();
                let reg = (instr & 0x0F00) >> 8 as Data;
                let kk = (instr & 0x00FF) as Data;
                *self.registers.get_mut(reg as usize).unwrap() = rand & kk;
            }
            0xE0A1..=0xEFA1 => {
                if instr & 0xF0FF != 0xE0A1 { panic!("no such instruction (0xExA1)")}
                let regx = ((instr & 0x0F00) >> 8) as Data;
                let key_wanted = *self.registers.get(regx as usize).unwrap();
                if !self.screen.window.is_key_down(io::u8_to_key(key_wanted)) {
                    self.pc += 2;
                }
            }
            0xE09E..=0xEF9E => {
                /*
                Skip next instruction if key with the value of Vx is pressed.
                Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2. 
                */
                if instr & 0xF0FF != 0xE09E { panic!("no such instruction (0xEx9E)")}
                let regx = ((instr & 0x0F00) >> 8) as Data;
                let key_wanted = *self.registers.get(regx as usize).unwrap();
                if self.screen.window.is_key_down(io::u8_to_key(key_wanted)) {
                    self.pc += 2;
                }
            }  

            0xF029..=0xFF29 => {}
            0xF033..=0xFF33 => {}
            0xF055..=0xFF55 => {}
            0xF065..=0xFF65 => {}

            0xF00A..=0xFF0A => {
                /*
                Fx0A - LD Vx, K
                Wait for a key press, store the value of the key in Vx.
                All execution stops until a key is pressed, then the value of that key is stored in Vx.
                 */
                if instr & 0xF0FF != 0xF00A { panic!("no such instruction (0xFx0A)")}
                let reg = (instr & 0x0F00) >> 8;
                let key = self.screen.wait_for_key();
                *self.registers.get_mut(reg as usize).unwrap() = key
            }
            0xF015..=0xFF15 => {
                /*
                Fx15 - LD DT, Vx
                Set delay timer = Vx.
                DT is set equal to the value of Vx.
               */
                if instr & 0xF0FF != 0xF015 { panic!("no such instruction (0xFx15)")}
                let reg = (instr & 0x0F00) >> 8;
                self.delay_t.set(
                    *self.registers.get(reg as usize).unwrap()
                )
            }
            0xF018..=0xFF18 => {
                /*
                Fx18 - LD ST, Vx
                Set sound timer = Vx.
                ST is set equal to the value of Vx. */
                if instr & 0xF0FF != 0xF018 { panic!("no such instruction (0xFx18)")}
                let reg = (instr & 0x0F00) >> 8;
                self.sound_t.set(
                    *self.registers.get(reg as usize).unwrap()
                )
            }
            0xF01E..=0xFF1E => {
                /*
                Fx1E - ADD I, Vx
                Set I = I + Vx.
                The values of I and Vx are added, and the results are stored in I. */
                let reg =  (instr & 0x0F00) >> 8;
                self.i += *self.registers.get(reg as usize).unwrap() as AddressLong;
            }
            0xF007..=0xFF07 => {
                /*Set Vx = delay timer value.
                The value of DT is placed into Vx. */
                if instr & 0xF0FF != 0xF007 { panic!("no such instruction (0xFx07)")}
                let regx = ((instr & 0x0F00) >> 8) as Data;

                self.delay_t.set(
                    *self.registers.get(regx as usize).unwrap() as Data
                )
            }
            _ => {panic!("no such instruction. undefined.")}
        }
    }

    pub fn load(&mut self, filepath :&str) {
        let buffer = io::load(filepath);
        for (i, val) in buffer.iter().enumerate() {
            self.memory.write(*val, i+512);
        }
    }

}

