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

pub fn assemble_scene(scene: &str) -> Vec<u8> {
    scene.lines().flat_map(|line| assemble_line(line)).collect()
}


#[cfg(test)]
mod tests {
    use crate::assembler::assemble_line;
    #[test]
        fn assemble_test_solid() {
            let actual_value = assemble_line("SOLID 1");
            let expected_value = vec![0x02, 0x01];
            assert_eq!(actual_value, expected_value);
        }
    #[test]
        fn assemble_test_ret() {
            let actual_value = assemble_line("RET");
            let expected_value = vec![0x0B];
            assert_eq!(actual_value, expected_value);
        }
    #[test]
        fn assemble_test_anime1() {
            let actual_value = assemble_line("ANIME1 0, 2");
            let expected_value = vec![0x00, 0x00, 0x02];
            assert_eq!(actual_value, expected_value);
        }
    #[test]
        fn assemble_test_pc() {
            let actual_value = assemble_line("PC 0");
            let expected_value = vec![0x01, 0x00];
            assert_eq!(actual_value, expected_value);
        }
    #[test]
        fn assemble_test_ifub() {
            let actual_value = assemble_line("IFUB 0, 3, 0, 8");
            let expected_value = vec![0x0A, 0x00, 0x03, 0x00, 0x08, 0x00];
            assert_eq!(actual_value, expected_value);
        }
    #[test]
        fn assemble_test_setbyte() {
            let actual_value = assemble_line("SETBYTE 0, 3, 1");
            let expected_value = vec![0x28, 0x00, 0x03, 0x01];
            assert_eq!(actual_value, expected_value);
        }
    #[test]
        fn assemble_test_window() {
            let actual_value = assemble_line("WINDOW 0, 50, 100, 200, 60");
            let expected_value = vec![0x16, 0x00, 0x00, 0x32, 0x00, 0x64, 0x00, 0xC8, 0x00, 0x3C];
            assert_eq!(actual_value, expected_value);
        }
    #[test]
        fn assemble_test_message() {
            let actual_value = assemble_line("MESSAGE 0, 0");
            let expected_value = vec![0x14, 0x00, 0x00, 0x00];
            assert_eq!(actual_value, expected_value);
        }
    #[test]
        fn assemble_test_winclose() {
            let actual_value = assemble_line("WINCLOSE 0");
            let expected_value = vec![0x15, 0x00];
            assert_eq!(actual_value, expected_value);
        }
}