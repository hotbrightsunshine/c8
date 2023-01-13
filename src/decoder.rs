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
    StoreBCD { register: Data },
    
    /// Fx55 - LD [I], Vx
    StoreRegistersToMemory { to_register: Data },
    
    /// Fx65 - LD Vx, [I]
    LoadRegistersFromMemory { to_register: Data },

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

        ( 0x3000, _ ) => Instruction::SkipEqualRegisterBytes { register_index: neck, bytes: bodytail },

        ( 0x4000, _ ) => Instruction::SkipNotEqualRegisterBytes { register_index: neck as u8, bytes: bodytail },

        ( 0x5000, _) => {
            if tail != 0 {
                Instruction::Invalid
            } else {
                Instruction::SkipEqualRegisterRegister { register_x: neck, register_y: body }
            }
        },

        ( 0x6000, _ ) => Instruction::SetRegisterToBytes { register: neck, bytes: bodytail },

        (0x7000, _ ) => Instruction::AddBytesToRegister { register: neck, bytes: bodytail },

        (0x8000, _ ) => {
            match tail {
                0 => Instruction::SetRegisterToRegister { register_x: neck, register_y: body },
                1 => Instruction::BitwiseOr { register_x: neck, register_y: body },
                2 => Instruction::BitwiseAnd { register_x: neck, register_y: body },
                3 => Instruction::BitwiseXor { register_x: neck, register_y: body },
                4 => Instruction::AddRegisterToRegister { register_x: neck, register_y: body },
                5 => Instruction::SubtractRegisterToRegister { register_x: neck, register_y: body },
                6 => Instruction::LeastSignificantBit { register: neck },
                7 => Instruction::SubtractInversed { register_x: neck, register_y: body },
                0xE => Instruction::MostSignificantBit { register: neck },
                _ => Instruction::Invalid
            }
        },

        (0x9000, _ ) => {
            if tail != 0 {
                Instruction::Invalid
            } else {
                Instruction::SkipNotEqualRegisterRegister { register_x: neck, register_y: body }
            }
        },

        (0xA000, lower ) =>  Instruction::SetI { value: lower },
        (0xB000, lower) => Instruction::JumpToLocationPlusZeroRegister { address: lower },
        (0xC000, _ ) => Instruction::Random { register: neck, value: bodytail },
        (0xD000, _ ) => Instruction::Display { register_x: neck, register_y: body, nibble: tail },
        (0xE000, _ ) => {
            match bodytail {
                0x9E => Instruction::SkipIfKeyIsPressed { register: neck },
                0xA1 => Instruction::SkipIfKeyIsNotPressed { register: neck },
                _ => Instruction::Invalid
            }
        },
        (0xF000, _ ) => {
            match bodytail {
                0x07 => Instruction::SetRegisterToDelayTimer { register: neck },
                0x0A => Instruction::WaitForKey { register: neck },
                0x15 => Instruction::SetDelayTimer { register: neck },
                0x18 => Instruction::SetSoundTimer { register: neck },
                0x1E => Instruction::AddRegisterToI { register: neck },
                0x29 => Instruction::SetIToLocationOfSprite { register: neck },
                0x33 => Instruction::StoreBCD { register: neck },
                0x55 => Instruction::StoreRegistersToMemory { to_register: neck },
                0x65 => Instruction::LoadRegistersFromMemory { to_register: neck },
                _ => Instruction::Invalid
            }
        }
        _ => Instruction::Invalid
    }
}
