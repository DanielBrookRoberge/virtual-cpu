use crate::flags::Flags8080;
use virtual_cpu_core::bytes::*;
use virtual_cpu_core::{Registers16, Registers8};

#[derive(Clone, Copy)]
pub enum Name8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
pub enum Name16 {
    BC,
    DE,
    HL,
}

#[derive(Debug, Default)]
pub struct Registers8080 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub cc: Flags8080,
}

impl Registers8080 {
    pub fn new() -> Registers8080 {
        Registers8080::default()
    }

    pub fn set_flags_from_r8(&mut self, reg: Name8) {
        self.cc.set_flags_no_carry(self.get8(reg));
    }
}

impl Registers8 for Registers8080 {
    type Name = Name8;

    fn get8(&self, reg: Name8) -> u8 {
        match reg {
            Name8::A => self.a,
            Name8::B => self.b,
            Name8::C => self.c,
            Name8::D => self.d,
            Name8::E => self.e,
            Name8::H => self.h,
            Name8::L => self.l,
        }
    }

    fn set8(&mut self, reg: Name8, val: u8) {
        match reg {
            Name8::A => self.a = val,
            Name8::B => self.b = val,
            Name8::C => self.c = val,
            Name8::D => self.d = val,
            Name8::E => self.e = val,
            Name8::H => self.h = val,
            Name8::L => self.l = val,
        }
    }
}

impl Registers16 for Registers8080 {
    type Name = Name16;

    fn get16(&self, reg: Name16) -> u16 {
        match reg {
            Name16::BC => assemble_word(self.b, self.c),
            Name16::DE => assemble_word(self.d, self.e),
            Name16::HL => assemble_word(self.h, self.l),
        }
    }

    fn set16(&mut self, reg: Name16, val: u16) {
        match reg {
            Name16::BC => {
                self.b = high_order_byte(val);
                self.c = low_order_byte(val);
            }
            Name16::DE => {
                self.d = high_order_byte(val);
                self.e = low_order_byte(val);
            }
            Name16::HL => {
                self.h = high_order_byte(val);
                self.l = low_order_byte(val);
            }
        }
    }
}
