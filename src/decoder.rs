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
    LoadRegisterToRegister { from_register: Data, to_register: Data },

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

pub fn subdivide_instr(val: AddressLong) -> (AddressLong, AddressLong, AddressLong, AddressLong) {
    (
        val & 0xF000,
        val & 0x0F00,
        val & 0x00F0,
        val & 0x000F
    )
}

pub fn decode(instr : u16) -> Instruction {
    let instr = subdivide_instr(instr);
    match instr {
        ( 0x0000, 0x0000, 0x00E0, 0x0000 ) => Instruction::Cls,
        ( 0x0000, 0x0000, 0x00E0, 0x000E ) => Instruction::Ret,
        ( 0x1000, _,      _,      _      ) => Instruction::Jump { location: instr.1 + instr.2 + instr.3 },
        ( 0x2000, _,      _,      _      ) => Instruction::Call { location: instr.1 + instr.2 + instr.3 },
        ( 0x3000, _,      _,      _      ) => Instruction::SkipEqualRegisterBytes { register_index: (instr.1 >> 8) as Data, bytes: (instr.2 + instr.3) as Data },
        ( 0x4000, _,      _,      _      ) => Instruction::SkipNotEqualRegisterBytes { register_index: (instr.1 >> 8) as Data, bytes: (instr.2 + instr.3) as Data },
        ( 0x5000, _,      _,      0x0000 ) => Instruction::SkipEqualRegisterRegister { register_x: (instr.1 >> 8) as Data, register_y: (instr.2 >> 4) as Data },
        ( 0x6000, _,      _,      _      ) => Instruction::SetRegisterToBytes { register: (instr.1 >> 8) as Data, bytes: (instr.2 + instr.3) as Data },
        ( 0x7000, _,      _,      _      ) => Instruction::AddBytesToRegister { register: (instr) , bytes: () }
        _ => Instruction::Invalid
    }
}
