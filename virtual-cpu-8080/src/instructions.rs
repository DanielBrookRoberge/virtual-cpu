use virtual_cpu_core::bytes::*;

use crate::flags::Flags8080;
use crate::registers::Name8;

pub fn register_for_code(code: u8) -> Name8 {
    match code & 0x07 {
        0x0 => Name8::B,
        0x1 => Name8::C,
        0x2 => Name8::D,
        0x3 => Name8::E,
        0x4 => Name8::H,
        0x5 => Name8::L,
        0x6 => panic!("0x6 needs special handling"),
        0x7 => Name8::A,
        _ => panic!("shouldn't happen"),
    }
}

pub fn predicate_for(opcode: u8) -> impl (Fn(&Flags8080) -> bool) {
    match (opcode >> 3) & 0x07 {
        0x0 => Flags8080::is_nz,
        0x1 => Flags8080::is_z,
        0x2 => Flags8080::is_nc,
        0x3 => Flags8080::is_c,
        0x4 => Flags8080::is_parity_odd,
        0x5 => Flags8080::is_parity_even,
        0x6 => Flags8080::is_plus,
        0x7 => Flags8080::is_minus,
        _ => panic!("shouldn't happen"),
    }
}

pub fn word_arg_from(instruction: &[u8]) -> u16 {
    assemble_word(instruction[2], instruction[1])
}

pub fn byte_arg_from(instruction: &[u8]) -> u8 {
    instruction[1]
}

pub fn apply_offset(base: u16, offset: u8) -> u16 {
    ((base as i32) + (offset as i8 as i32)) as u16
}

pub fn and8(a: u8, b: u8) -> u8 {
    a & b
}

pub fn xor8(a: u8, b: u8) -> u8 {
    a ^ b
}

pub fn or8(a: u8, b: u8) -> u8 {
    a | b
}

pub fn inc8(n: u8) -> u8 {
    n.wrapping_add(1)
}

pub fn inc16(n: u16) -> u16 {
    n.wrapping_add(1)
}

pub fn dec8(n: u8) -> u8 {
    n.wrapping_sub(1)
}

pub fn dec16(n: u16) -> u16 {
    n.wrapping_sub(1)
}
