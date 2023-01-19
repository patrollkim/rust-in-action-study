struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

fn main() {

    let mut memory: [u8; 4096] = [0; 4096];
    let mem = &mut memory;

    let add_twice = [
        0x80, 0x14,
        0x80, 0x14,
        0x00, 0xEE,
    ];

    mem[0x100..0x106].copy_from_slice(&add_twice);

    println!("{:?}", &mem[0x100..0x106]);

    let mut memory: [u8; 4096] = [0; 4096];
    let mem = &mut memory;

    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
    println!("{:?}", &mem[0x100..0x106]);
    
} 
