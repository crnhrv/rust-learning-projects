struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 4096],
    stack: [u8; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn memory_address_from_nibbles(&mut self, n8: u8, n4: u8, n0: u8) {
        let nibble8 = (n8 as u16) << 8;
        let nibble4 = (n4 as u16) << 4; 
        let nibble0 = (n0 as u16) << 0; 

        self.program_counter = (nibble8 + nibble4 + nibble0) as usize;

    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;
            
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0)            => { return; }
                (0x2, n8, n4, n0)       => {
                    self.stack_pointer += 1;
                    self.stack[self.stack_pointer] = self.program_counter as u8;
                    self.memory_address_from_nibbles(n8, n4, n0);
                },
                (0x0, 0x0, 0xE, 0xE)    => {
                    let mem_address = self.stack[self.stack_pointer];
                    self.program_counter = mem_address as usize;
                    self.stack_pointer -= 1;
                }
                (0x8, _, _, 0x4)        => self.add_xy(x, y),
                _                       => todo!("opcode: {:04x}", opcode),
            }
       }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        program_counter: 0,
        registers: [0; 16],
        memory: [0; 4096],
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // opcode 0x8014:   
    //          0x8 == opcode group
    //          0x0 == register index
    //          0x1 == register index
    //          0x4 == add

    let add_twice: [u8; 6] = [
        0x80, 0x14,
        0x80, 0x14,
        0x00, 0xEE
    ];

    let mem = &mut cpu.memory;
    mem[0x100..0x106].copy_from_slice(&add_twice);

    // opcode 0x00EE - return after add_twice;
    mem[0x107] = 0x00;
    mem[0x108] = 0x01;

    // opcode 0x2nnn: - set program counter to nnn (to call add_twice from memory)
    mem[0] = 0x21;   
    mem[1] = 0x00;  

    // call it again
    mem[2] = 0x21;   
    mem[3] = 0x00;  

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("(5 + 10 + 10) + 10 + 10 = {}", cpu.registers[0]);
}
