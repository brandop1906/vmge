use crate::opcodes::Opcode;
use crate::commands::Command;

pub struct VM {
    counter_position: usize,
    bytecode: Vec<u8>,
    banks: [[u8; 256]; 256],
    pub commands: Vec<Command>,
}

impl VM {
    pub fn new(bytecode: Vec<u8>) -> VM {
        VM {
            bytecode,
            counter_position: 0,
            banks: [[0; 256]; 256],
            commands: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = Opcode::from_byte(self.bytecode[self.counter_position]);
            match opcode {
                Opcode::Solid => {
                    self.commands.push(Command::SetSolid {
                        character_id: 0, // hardcoded for now
                        enabled: self.bytecode[self.counter_position + 1] != 0,
                    });

                    self.counter_position += 2;
                    println!("SOLID");
                }
                Opcode::Anime1 => {
                    self.counter_position += 3;
                    println!("ANIME1");
                }
                Opcode::Pc => {
                    self.counter_position += 2;
                    println!("PC");
                }
                Opcode::Ifub => {
                    let bank = self.bytecode[self.counter_position + 1] as usize;
                    let slot = self.bytecode[self.counter_position + 2] as usize;
                    let compare_value = self.bytecode[self.counter_position + 3];
                    let current_value = self.banks[bank][slot];
                    if current_value == compare_value {
                        self.counter_position += 6;
                        println!("if IFUB");
                    } else {
                        let little_endian_value = (self.bytecode[self.counter_position + 4] as u16) 
                        | ((self.bytecode[self.counter_position + 5] as u16) << 8);
                        self.counter_position += little_endian_value as usize;
                        println!("else IFUB");
                    }
                }
                Opcode::Message => {
                    self.counter_position += 4;
                    println!("MESSAGE");
                }
                Opcode::WinClose => {
                    self.counter_position += 2;
                    println!("WINCLOSE");
                }
                Opcode::Window => {
                    let x = self.bytecode[self.counter_position + 1] as i16
                        | (self.bytecode[self.counter_position + 2] as i16) << 8;
                    let y = self.bytecode[self.counter_position + 3] as i16
                        | (self.bytecode[self.counter_position + 4] as i16) << 8;
                    let width = self.bytecode[self.counter_position + 5] as i16
                        | (self.bytecode[self.counter_position + 6] as i16) << 8;
                    let height = self.bytecode[self.counter_position + 7] as i16
                        | (self.bytecode[self.counter_position + 8] as i16) << 8;
                    let window_id = self.bytecode[self.counter_position + 9];

                    self.commands.push(Command::WindowOpen {
                        x, y, width, height, window_id,
                    });

                    self.counter_position += 10;
                }
                Opcode::SetByte => {
                    let bank = self.bytecode[self.counter_position + 1] as usize;
                    let slot = self.bytecode[self.counter_position + 2] as usize;
                    let value = self.bytecode[self.counter_position + 3] as u8;
                    self.banks[bank][slot] = value;
                    self.counter_position += 4;
                    println!("SETBYTE");
                }
                Opcode::Ret => {
                    println!("RET");
                    break;
                }
            }
            
        }
        self.counter_position = 0;
    }
}