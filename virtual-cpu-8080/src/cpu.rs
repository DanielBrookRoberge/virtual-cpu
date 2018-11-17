use crate::instructions::*;
use crate::registers::*;
use crate::state::State8080;

pub fn execute(state: &mut State8080, instruction: &[u8]) {
    let opcode = instruction[0];

    match opcode {
        0x00 => (), // NOP
        0x01 => state.mov_ri16(Name16::BC, word_arg_from(instruction)), // LXI B,word
        0x02 => state.mov_pr8(Name16::BC, Name8::A), // STAX B

        0x06 => state.mov_ri8(Name8::B, byte_arg_from(instruction)), // MVI B,byte

        0x08 => (), // NOP

        0x0a => state.mov_rp8(Name8::A, Name16::BC), // LDAX B

        0x0e => state.mov_ri8(Name8::C, byte_arg_from(instruction)), // MVI C,byte

        0x10 => (), // NOP
        0x11 => state.mov_ri16(Name16::DE, word_arg_from(instruction)), // LXI D,word
        0x12 => state.mov_pr8(Name16::DE, Name8::A), // STAX D

        0x16 => state.mov_ri8(Name8::D, byte_arg_from(instruction)), // MVI D,byte

        0x18 => (), // NOP

        0x1a => state.mov_rp8(Name8::A, Name16::DE), // LDAX D

        0x1e => state.mov_ri8(Name8::E, byte_arg_from(instruction)), // MVI E,byte

        0x20 => (), // NOP
        0x21 => state.mov_ri16(Name16::HL, word_arg_from(instruction)), // LXI H,word
        0x22 => state.mov_ar16(word_arg_from(instruction), Name16::HL), // SHLD a16

        0x26 => state.mov_ri8(Name8::H, byte_arg_from(instruction)), // MVI H,byte

        0x28 => (), // NOP

        0x2a => state.mov_ra16(Name16::HL, word_arg_from(instruction)), // LHLD a16

        0x2e => state.mov_ri8(Name8::E, byte_arg_from(instruction)), // MVI E,byte

        0x76 => panic!("HLT reached"),
        0x70..=0x78 => state.mov_pr8(Name16::HL, register_for_code(opcode)),
        0x40..=0x7f => state.mov_rr8(register_for_code(opcode >> 3), register_for_code(opcode)),

        0x80..=0x88 => state.add_rr8(register_for_code(opcode)),
        0xa0..=0xa8 => state.logical_operation_rr(register_for_code(opcode), and8),
        0xa9..=0xaf => state.logical_operation_rr(register_for_code(opcode), xor8),
        0xb0..=0xb8 => state.logical_operation_rr(register_for_code(opcode), or8),

        _ => panic!("Unimplemented")
    }
}
