use virtual_cpu_core::memory::Memory;
use virtual_cpu_core::program::Program;
use virtual_cpu_core::stack::Stack;

use crate::memory::Memory8080;
use crate::stack::Stack8080;

static INSTRUCTION_LENGTH: [u16; 256] = [
    1, 3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 0x00..0x0f
    1, 3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 0x10..0x1f
    1, 3, 3, 1, 1, 1, 2, 1, 1, 1, 3, 1, 1, 1, 2, 1, // 0x20..0x2f
    1, 3, 3, 1, 1, 1, 2, 1, 1, 1, 3, 1, 1, 1, 2, 1, // 0x30..0x3f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x40..0x4f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x50..0x5f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x60..0x6f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x70..0x7f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x80..0x8f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x90..0x9f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0xa0..0xaf
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0xb0..0xbf
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 3, 3, 3, 2, 1, // 0xc0..0xcf
    1, 1, 3, 2, 3, 1, 2, 1, 1, 1, 3, 2, 3, 3, 2, 1, // 0xd0..0xdf
    1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1, // 0xc0..0xcf
    1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1, // 0xc0..0xcf
];

#[derive(Default, Debug)]
pub struct Program8080 {
    pc: u16,
    instruction_length: u16,
}

impl Program8080 {
    pub fn new() -> Program8080 {
        Program8080::default()
    }
}

impl Program for Program8080 {
    type Address = u16;
    type Mem = Memory8080;
    type Stk = Stack8080;

    fn get_pc(&self) -> u16 {
        self.pc
    }

    fn get_instruction(&mut self, m: &Memory8080) -> Vec<u8> {
        let opcode = m.get_byte(self.pc);
        self.instruction_length = INSTRUCTION_LENGTH[opcode as usize];
        m.view(self.pc, self.pc + self.instruction_length - 1).to_vec()
    }

    fn advance(&mut self) {
        self.pc += self.instruction_length;
        self.instruction_length = 0;
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr;
        self.instruction_length = 0;
    }

    fn call(&mut self, m: &mut Memory8080, s: &mut Stack8080, addr: u16) {
        s.push_word(m, self.pc + self.instruction_length);
        self.jump(addr);
    }

    fn ret(&mut self, m: &mut Memory8080, s: &mut Stack8080) {
        self.jump(s.pop_word(m));
    }
}
