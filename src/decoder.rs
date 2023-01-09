use crate::types::AddressLong;
use crate::types::Data;


pub enum Instruction {
    /// 00E0 - CLS
    Cls,

    /// 00EE - RET
    Ret,

    /// 1nnn - JP addr
    Jump { location: AddressLong },

    /// 2nnn - CALL addr
    Call { location: AddressLong },

    /// 3xkk - SE Vx, byte
    SkipEqualRegisterBytes { register_index: Data, bytes: Data },

    /// 4xkk - SNE Vx, byte
    SkipNotEqualRegisterBytes { register_index: Data, bytes: Data },

    /// 5xy0 - SE Vx, Vy
    SkipEqualRegisterRegister {register_x: Data, register_y: Data },

    /// 6xkk - LD Vx, byte
    SetRegisterToBytes { register: Data, bytes: Data },

    /// 7xkk - ADD Vx, byte
    AddBytesToRegister { register: Data, bytes: Data },

    /// 8xy0 - LD Vx, Vy
    SetRegisterToRegister { register_x: Data, register_y: Data },

    /// 8xy1 - OR Vx, Vy
    BitwiseOr { register_x: Data, register_y: Data },

    /// 8xy2 - AND Vx, Vy
    BitwiseAnd { register_x: Data, register_y: Data },

    /// 8xy3 - XOR Vx, Vy
    BitwiseXor { register_x: Data, register_y: Data },

    /// 8xy4 - ADD Vx, Vy
    AddRegisterToRegister { register_x: Data, register_y: Data },

    /// 8xy5 - SUB Vx, Vy
    SubtractRegisterToRegister { register_x: Data, register_y: Data },

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    LeastSignificantBit { register: Data },

    /// 8xy7 - SUBN Vx, Vy
    SubtractInversed { register_x: Data, register_y: Data },

    /// 8xyE - SHL Vx {, Vy}
    MostSignificantBit { register: Data },

    /// 9xy0 - SNE Vx, Vy
    SkipNotEqualRegisterRegister { register_x: Data, register_y: Data },

    /// Annn - LD I, addr
    SetI { value: AddressLong },

    /// Bnnn - JP V0, addr
    JumpToLocationPlusZeroRegister { address: AddressLong },

    /// Cxkk - RND Vx, byte
    Random { register: Data, value: Data },

    /// Dxyn - DRW Vx, Vy, nibble
    Display { register_x: Data, register_y: Data, nibble: Data },

    /// Ex9E - SKP Vx
    SkipIfKeyIsPressed { register: Data },

    /// ExA1 - SKNP Vx
    SkipIfKeyIsNotPressed { register: Data },

    /// Fx07 - LD Vx, DT
    SetRegisterToDelayTimer { register: Data },

    /// Fx0A - LD Vx, K
    WaitForKey { register: Data },

    /// Fx15 - LD DT, Vx
    SetDelayTimer { register: Data },

    /// Fx18 - LD ST, Vx
    SetSoundTimer { register: Data },

    /// Fx1E - ADD I, Vx
    AddRegisterToI { register: Data },

    /// Fx29 - LD F, Vx
    SetIToLocationOfSprite { register: Data },

    /// Fx33 - LD B, Vx
    
    /// Fx55 - LD [I], Vx
    
    /// Fx65 - LD Vx, [I]

    Invalid
}

pub fn decode(instr : u16) -> Instruction {
    let (upper, lower) = (instr & 0xF000, instr & 0x0FFF);
    
    // A struct to do so might be needed in the near future!
    // Deconstructing 
    // Imagine this as 
    
    //   O      [   0,     HEAD
    //   |          1,     NECK
    //  /|\
    //   |          2,     BODY
    //   | 
    //   /\         3 ]    TAIL
    //  /  \     

    let (head, neck, body, tail) = (
        (instr & 0xF000 >> 12) as u8,
        (instr & 0x0F00 >> 8) as u8,
        (instr & 0x00F0 >> 4) as u8,
        (instr & 0x000F) as u8,
    );

    let bodytail = (instr & 0xFF) as u8; 

    match (upper, lower) {
        ( 0x0   , 0xE0  ) => Instruction::Cls,

        ( 0x0   , 0xEE  ) => Instruction::Ret,

        ( 0x1000, _     ) => Instruction::Jump { location: lower },

        ( 0x2000, _     ) => Instruction::Call { location: lower },

        ( 0x3000, lower ) => {
            Instruction::SkipEqualRegisterBytes { register_index: neck, bytes: bodytail }
        }

        ( 0x4000, lower ) => {
            Instruction::SkipNotEqualRegisterBytes { register_index: neck as u8, bytes: bodytail }
        }

        ( 0x5000, _) => {
            if tail != 0 {
                Instruction::Invalid
            } else {
                Instruction::SkipEqualRegisterRegister { register_x: neck, register_y: body }
            }
        }

        ( 0x6000, lower ) => {
            let (register, value) = (lower & 0xF00, lower & 0xFF);
            Instruction::SetRegisterToBytes { register: (register >> 8) as u8, bytes: value as u8 }
        }

        (0x7000, lower ) => {
            let (register, value) = (lower & 0xF00, lower & 0xFF);
            Instruction::AddBytesToRegister { register: (register >> 8) as u8, bytes: value as u8 }
        }

        (0x8000, lower ) => {
            match ((lower & 0xF00 >> 8) as u8, (lower & 0xF0 >> 4) as u8, (lower & 0xF) as u8) {
                (regx, regy, 0) => {
                    Instruction::SetRegisterToRegister { register_x: regx, register_y: regy }
                }

                (regx, regy, 1) => {
                    Instruction::BitwiseOr { register_x: regx, register_y: regy }
                }

                (reg)
                ()
                _ => Instruction::Invalid
            }
        }
        _ => Instruction::Invalid
    }
}
