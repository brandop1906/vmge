use crate::opcodes::Opcode;

pub fn assemble_line(line: &str) -> Vec<u8> {

    let (opcode_name, rest) = match line.split_once(' ') {
        Some((name, rest)) => (name, rest),
        None => (line, ""),
    };
    
    let trimmed = if rest.is_empty() {
        vec![]
    } else {
        rest.split(',').map(|piece| piece.trim()).collect()
    };

    let opcode = Opcode::from_string(opcode_name);
    let opcode_byte = opcode.to_byte();
    let sizes = opcode.operand_sizes();

    if trimmed.len() != sizes.len() {
        panic!("SIZE MISMATCH");
    }

    let mut opcode_vec = vec![opcode_byte];

    let operand_bytes: Vec<u8> = trimmed.iter().zip(sizes.iter()).flat_map(|(text, size)| {
        match size {
            1 => {let byte = text.parse::<u8>().unwrap();
                vec![byte]
            }
            2 => {
                let first_op: u16 = text.parse().unwrap();
                let little_byte = (first_op & 0xFF) as u8;
                let big_byte = (first_op >> 8) as u8;
                vec![little_byte, big_byte]
            }
            _ => {
                panic!("Unknown size: {}! Expected: 1 or 2!", size);
            }
        }
    }).collect();

    opcode_vec.extend(operand_bytes);
    return opcode_vec;
}