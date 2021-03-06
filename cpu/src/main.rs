struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;
            
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let addr = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0)            => { return; }
                (0x0, 0x0, 0xE, 0xE)    => self.retrn(),
                (0x2, _, _, _)          => self.call(addr),
                (0x8, _, _, 0x4)        => self.add_xy(x, y),
                _                       => todo!("opcode: {:04x}", opcode),
            }
       }
    }

    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn retrn(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        let call_addr =  self.stack[self.stack_pointer];
        self.program_counter = call_addr as usize;
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

    let mem = &mut cpu.memory;

    // opcode 0x00EE - return after add_twice;
    mem[0x107] = 0x00; mem[0x108] = 0x01;

    // opcode 0x2nnn: - set program counter to 0x100 (to call add at 0x100)
    mem[0x000] = 0x21; mem[0x001] = 0x00;  

    // call it another three times
    mem[0x002] = 0x21; mem[0x003] = 0x00;  
    mem[0x004] = 0x21; mem[0x005] = 0x00;  
    mem[0x006] = 0x21; mem[0x007] = 0x00;  

    // halt CPU
    mem[0x008] = 0x00; mem[0x009] = 0x00;  


    // opcode 0x8014:   
    //          0x8 == opcode group
    //          0x0 == register index
    //          0x1 == register index
    //          0x4 == add

    mem[0x100] = 0x80; mem[0x101] = 0x14;  

    // return opcode
    mem[0x102] = 0x00; mem[0x103] = 0xEE;  
  
    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("((((5 + 10) + 10) + 10) + 10) = {}", cpu.registers[0]);
}
