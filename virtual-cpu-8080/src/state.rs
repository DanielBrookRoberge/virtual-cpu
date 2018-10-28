use virtual_cpu_core::registers::*;
use virtual_cpu_core::memory::Memory;
use virtual_cpu_core::program::Program;

use crate::flags::Flags8080;
use crate::memory::Memory8080;
use crate::stack::Stack8080;
use crate::program::Program8080;
use crate::registers::*;
use crate::instructions::*;

pub struct State {
    pub m: Memory8080,
    pub s: Stack8080,
    pub p: Program8080,
    pub r: Registers8080
}

impl State {
    pub fn new() -> State {
        State {
            m: Memory8080::new(),
            s: Stack8080::new(),
            p: Program8080::new(),
            r: Registers8080::new()
        }
    }

// MOV operations

    pub fn mov_rr8(&mut self, dest: Name8, src: Name8) {
        self.r.set8(dest, self.r.get8(src));
    }

    pub fn mov_ri8(&mut self, dest: Name8, val: u8) {
        self.r.set8(dest, val);
    }

    pub fn mov_rp8(&mut self, dest: Name8, src: Name16) {
        self.r.set8(dest, self.m.get_byte(self.r.get16(src)));
    }

    pub fn mov_ra8(&mut self, dest: Name8, src: u16) {
        self.r.set8(dest, self.m.get_byte(src));
    }

    pub fn mov_pr8(&mut self, dest: Name16, src: Name8) {
        self.m.set_byte(self.r.get16(dest), self.r.get8(src));
    }

    pub fn mov_ar8(&mut self, dest: u16, src: Name8) {
        self.m.set_byte(dest, self.r.get8(src));
    }

    pub fn mov_rr16(&mut self, dest: Name16, src: Name16) {
        self.r.set16(dest, self.r.get16(src));
    }

    pub fn mov_ri16(&mut self, dest: Name16, val: u16) {
        self.r.set16(dest, val);
    }

    pub fn mov_rp16(&mut self, dest: Name16, src: Name16) {
        self.r.set16(dest, self.m.get_word(self.r.get16(src)));
    }

    pub fn mov_ra16(&mut self, dest: Name16, src: u16) {
        self.r.set16(dest, self.m.get_word(src));
    }

    pub fn mov_pr16(&mut self, dest: Name16, src: Name16) {
        self.m.set_word(self.r.get16(dest), self.r.get16(src));
    }

    pub fn mov_ar16(&mut self, dest: u16, src: Name16) {
        self.m.set_word(dest, self.r.get16(src));
    }

    // CONTROL FLOW
    pub fn test_flags(&self, predicate: impl Fn(&Flags8080) -> bool) -> bool {
        predicate(&self.r.cc)
    }

    pub fn jump_a(&mut self, addr: u16) {
        self.p.jump(addr);
    }

    pub fn call_a(&mut self, addr: u16) {
        self.p.call(&mut self.m, &mut self.s, addr);
    }

    pub fn ret(&mut self) {
        self.p.ret(&mut self.m, &mut self.s);
    }

    // BINARY OPERATIONS

    pub fn logical_operation_rr(&mut self, src: Name8, operation: impl Fn(u8, u8) -> u8) {
        let accumulator = self.r.get8(Name8::A);
        let operand = self.r.get8(src);

        let result = operation(accumulator, operand);
        self.r.cc.set_flags_no_carry(result);
        self.r.set8(Name8::A, result);
    }
}
