pub enum Opcode {
    Solid = 0x02, // ENTITY STATE
    Anime1 = 0x00, // ENTITY STATE
    Pc = 0x01, // ENTITY STATE
    Ifub = 0x0A, // CONTROL FLOW
    Ret = 0x0B, // CONTROL FLOW
    Message = 0x14, // UI
    WinClose = 0x15, // UI
    Window = 0x16, // UI
    SetByte = 0x28, // GAME STATE
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Opcode {
        match byte {
            0x00 => Opcode::Anime1,
            0x01 => Opcode::Pc, 
            0x02 => Opcode::Solid,
            0x0A => Opcode::Ifub,
            0x0B => Opcode::Ret,
            0x14 => Opcode::Message,
            0x15 => Opcode::WinClose,
            0x16 => Opcode::Window,
            0x28 => Opcode::SetByte,
            _ => panic!("Unknown opcode: {}", byte),
        }
    }
    
    pub fn to_byte(&self) -> u8 {
        match self {
            Opcode::Anime1 => 0x00,
            Opcode::Pc => 0x01,
            Opcode::Solid => 0x02,
            Opcode::Ifub => 0x0A,
            Opcode::Ret => 0x0B,
            Opcode::Message => 0x14,
            Opcode::WinClose => 0x15,
            Opcode::Window => 0x16,
            Opcode::SetByte => 0x28,
        }
    }

    pub fn from_string(string: &str) -> Opcode {
        match string {
            "ANIME1" => Opcode::Anime1,
            "PC" => Opcode::Pc,
            "SOLID" => Opcode::Solid,
            "IFUB" => Opcode::Ifub,
            "RET" => Opcode::Ret,
            "MESSAGE" => Opcode::Message,
            "WINCLOSE" => Opcode::WinClose,
            "WINDOW" => Opcode::Window,
            "SETBYTE" => Opcode::SetByte,
            _ => panic!("Unknown opcode: {}", string),
        }
    }

    pub fn operand_sizes(&self) -> Vec<u8> {
        match self {
            Opcode::Anime1 => vec![1, 1],
            Opcode::Pc => vec![1],
            Opcode::Solid => vec![1],
            Opcode::Ifub => vec![1, 1, 1, 2],
            Opcode::Ret => vec![],
            Opcode::Window => vec![2, 2, 2, 2, 1],
            Opcode::Message => vec![1, 2],
            Opcode::WinClose => vec![1],
            Opcode::SetByte => vec![1, 1, 1],
        }
    }
}