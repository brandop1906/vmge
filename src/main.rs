mod opcodes;
mod vm;
mod assembler;

fn main() {

    let scene = vec! [
        0x02, 0x01,                                           // SOLID 1
        0x00, 0x00, 0x02,                                     // ANIME1 0, 2
        0x01, 0x00,                                           // PC 0
        0x0A, 0x01, 0x05, 0x00, 0x1A, 0x00,                   // IFUB 1, 5, 0, 26
        0x16, 0x00, 0x00, 0x32, 0x00, 0x64, 0x00, 0xC8, 0x00, 0x3C, // WINDOW 0, 50, 100, 200, 60
        0x14, 0x00, 0x00, 0x00,                               // MESSAGE 0, 0
        0x15, 0x00,                                           // WINCLOSE 0
        0x28, 0x01, 0x05, 0x01,                               // SETBYTE 1, 5, 1
        0x0B,                                                  // RET
    ];

    let mut vm = vm::VM::new(scene);

    vm.run();
    vm.run();

    let result = assembler::assemble_line("FOOBAR 1");
    let result2 = assembler::assemble_line("WINDOW 0, 50, 100, 200, 60");
    let result3 = assembler::assemble_line("SETBYTE 1, 5, 1");
    let resul4 = assembler::assemble_line("IFUB 1, 5, 0, 26");
    let result5 = assembler::assemble_line("RET");
    println!("{:?}", result);
}   