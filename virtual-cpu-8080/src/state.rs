use virtual_cpu_core::registers::*;
use virtual_cpu_core::memory::Memory;
use virtual_cpu_core::program::Program;
use virtual_cpu_core::stack::Stack;

use crate::flags::Flags8080;
use crate::memory::Memory8080;
use crate::stack::Stack8080;
use crate::program::Program8080;
use crate::registers::*;
use crate::instructions::*;

#[derive(Debug)]
pub struct State8080 {
    pub m: Memory8080,
    pub s: Stack8080,
    pub p: Program8080,
    pub r: Registers8080
}

impl State8080 {
    pub fn new() -> State8080 {
        State8080 {
            m: Memory8080::new(),
            s: Stack8080::new(),
            p: Program8080::new(),
            r: Registers8080::new()
        }
    }

// MOV operations

    // MOV register FROM register
    pub fn mov_rr8(&mut self, dest: Name8, src: Name8) {
        self.r.set8(dest, self.r.get8(src));
    }

    // MOV register FROM immediate
    pub fn mov_ri8(&mut self, dest: Name8, val: u8) {
        self.r.set8(dest, val);
    }

    // MOV register FROM pointer (in register)
    pub fn mov_rp8(&mut self, dest: Name8, src: Name16) {
        self.r.set8(dest, self.m.get_byte(self.r.get16(src)));
    }

    // MOV register FROM address (in operand)
    pub fn mov_ra8(&mut self, dest: Name8, src: u16) {
        self.r.set8(dest, self.m.get_byte(src));
    }

    // MOV pointer FROM register
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

    pub fn add_ri8(&mut self, operand: u8) {
        let (result, carry) = self.r.get8(Name8::A).overflowing_add(operand);
        self.r.cc.set_flags_no_carry(result);
        self.r.cc.cy = carry;
        self.r.set8(Name8::A, result);
    }

    pub fn add_rr8(&mut self, src: Name8) {
        self.add_ri8(self.r.get8(src));
    }

    pub fn logical_operation_ri(&mut self, operand: u8, operation: impl Fn(u8, u8) -> u8) {
        let accumulator = self.r.get8(Name8::A);

        let result = operation(accumulator, operand);
        self.r.cc.set_flags_no_carry(result);
        self.r.set8(Name8::A, result);
    }

    pub fn logical_operation_rr(&mut self, src: Name8, operation: impl Fn(u8, u8) -> u8) {
        self.logical_operation_ri(self.r.get8(src), operation);
    }

    // STACK OPERATIONS

    pub fn push_r16(&mut self, src: Name16) {
        self.s.push_word(&mut self.m, self.r.get16(src));
    }

    pub fn pop_r16(&mut self, dest: Name16) {
        self.r.set16(dest, self.s.pop_word(&mut self.m));
    }

    // PROGRAM OPERATIONS

    pub fn get_instruction(&mut self) -> Vec<u8> {
        self.p.get_instruction(&mut self.m)
    }
}
