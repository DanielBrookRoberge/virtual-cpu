use crate::instructions::*;
use crate::machine::Machine;
use crate::registers::{Name16, Name8};
use crate::state::State8080 as State;
use virtual_cpu_core::{Program, Registers16, Registers8, Stack};

static OPCODE_TIMING: [usize; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x00..0x0f
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x10..0x1f
    4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, //etc
    4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5,
    5, 7, 5, //0x40..0x4f
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7,
    4, //0x80..8x4f
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10,
    10, 17, 7, 11, //0xc0..0xcf
    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11, 11, 10, 10, 18, 17, 11, 7, 11,
    11, 5, 10, 5, 17, 17, 7, 11, 11, 10, 10, 4, 17, 11, 7, 11, 11, 5, 10, 4, 17, 17, 7, 11,
];

fn get_operand(state: &State, opcode: u8) -> u8 {
    let operand_code = opcode & 0x07;
    if operand_code == 0x06 {
        state.get_indirect8(Name16::HL)
    } else {
        state.r.get8(register_for_code(operand_code))
    }
}

fn mov_for(state: &mut State, opcode: u8) {
    let input_code = opcode & 0x07;
    let output_code = (opcode >> 3) & 0x07;

    match (input_code, output_code) {
        (0x06, 0x06) => unimplemented_instruction(state, opcode), // this is HLT
        (0x06, reg) => state.mov_rp8(register_for_code(reg), Name16::HL),
        (reg, 0x06) => state.mov_pr8(Name16::HL, register_for_code(reg)),
        (input, output) => state.mov_rr8(register_for_code(output), register_for_code(input)),
    }
}

fn operate8(state: &mut State, opcode: u8, operand: u8) {
    match (opcode >> 3) & 0x07 {
        0x0 => state.add_ri8(operand),
        0x1 => state.adc_ri8(operand),
        0x2 => state.sub_ri8(operand),
        0x3 => state.sbb_ri8(operand),
        0x4 => state.logical_operation_ri(operand, and8),
        0x5 => state.logical_operation_ri(operand, xor8),
        0x6 => state.logical_operation_ri(operand, or8),
        0x7 => state.cmp_ri8(operand),
        _ => panic!("Shouldn't happen"),
    }
}

fn unimplemented_instruction(s: &mut State, opcode: u8) {
    println!(
        "Error: unimplemented instruction 0x{:02x} at 0x{:04x}",
        opcode,
        s.p.get_pc()
    );
    println!("{:?}", s);
    panic!("unimplemented");
}

pub fn emulate_group0(instruction: &[u8], s: &mut State) {
    let opcode = instruction[0];

    match opcode & 0x3f {
        0x00 => (),                                                 // NOP
        0x01 => s.mov_ri16(Name16::BC, word_arg_from(instruction)), // LXI B,word
        0x02 => s.mov_pr8(Name16::BC, Name8::B),                    // STAX B
        0x03 => s.r.update16(Name16::BC, inc16),                    // INX B
        0x04 => s.unary_math_r8(Name8::B, inc8),                    // INR B
        0x05 => s.unary_math_r8(Name8::B, dec8),                    // DCR B
        0x06 => s.mov_ri8(Name8::B, byte_arg_from(instruction)),    // MVI B,byte
        0x07 => {
            // RLC
            let x = s.r.get8(Name8::A);
            s.r.set8(Name8::A, x.rotate_left(1));
            s.r.cc.cy = (x & 0x80) != 0;
        }
        0x08 => (),                                              // NOP
        0x09 => s.add_rr16(Name16::BC),                          // DAD B
        0x0a => s.mov_rp8(Name8::A, Name16::BC),                 // LDAX B
        0x0b => s.r.update16(Name16::BC, dec16),                 // DCX B
        0x0c => s.unary_math_r8(Name8::C, inc8),                 // INR C
        0x0d => s.unary_math_r8(Name8::C, dec8),                 // DCR C
        0x0e => s.mov_ri8(Name8::C, byte_arg_from(instruction)), // MVI C,byte
        0x0f => {
            // RRC
            let x = s.r.get8(Name8::A);
            s.r.set8(Name8::A, x.rotate_right(1));
            s.r.cc.cy = (x & 0x01) == 1;
        }

        0x10 => (),                                                 // NOP
        0x11 => s.mov_ri16(Name16::DE, word_arg_from(instruction)), // LXI D,word
        0x12 => s.mov_pr8(Name16::DE, Name8::A),                    // STAX D
        0x13 => s.r.update16(Name16::DE, inc16),                    // INX D
        0x14 => s.unary_math_r8(Name8::D, inc8),                    // INR D
        0x15 => s.unary_math_r8(Name8::D, dec8),                    // DCR D
        0x16 => s.mov_ri8(Name8::D, byte_arg_from(instruction)),    // MVI D,byte
        0x17 => {
            // RAL
            let x = s.r.get8(Name8::A);
            s.r.a = (s.r.cc.cy as u8) | (x << 1);
            s.r.cc.cy = (x & 0x80) != 0;
        }
        0x18 => (),                                              // NOP
        0x19 => s.add_rr16(Name16::DE),                          // DAD D
        0x1a => s.mov_rp8(Name8::A, Name16::DE),                 // LDAX D
        0x1b => s.r.update16(Name16::DE, dec16),                 // DCX D
        0x1c => s.unary_math_r8(Name8::E, inc8),                 // INR E
        0x1d => s.unary_math_r8(Name8::E, dec8),                 // DCR E
        0x1e => s.mov_ri8(Name8::E, byte_arg_from(instruction)), // MVI E,byte
        0x1f => {
            // RAR
            let x = s.r.get8(Name8::A);
            s.r.a = ((s.r.cc.cy as u8) << 7) | (x >> 1);
            s.r.cc.cy = (x & 0x01) == 1;
        }

        0x20 => (),                                                 // NOP
        0x21 => s.mov_ri16(Name16::HL, word_arg_from(instruction)), // LXI H,word
        0x22 => s.mov_ar16(word_arg_from(instruction), Name16::HL), // SHLD a16
        0x23 => s.r.update16(Name16::HL, inc16),                    // INX H
        0x24 => s.unary_math_r8(Name8::H, inc8),                    // INR H
        0x25 => s.unary_math_r8(Name8::H, dec8),                    // DCR H
        0x26 => s.mov_ri8(Name8::H, byte_arg_from(instruction)),    // MVI H,byte
        0x27 => {
            // DAA
            if (s.r.get8(Name8::A) & 0x0f) > 9 {
                s.r.a += 6;
            }
            if (s.r.get8(Name8::A) & 0xf0) > 0x90 {
                s.add_ri8(0x60);
            }
        }
        0x28 => (),                                                 // NOP
        0x29 => s.add_rr16(Name16::HL),                             // DAD H
        0x2a => s.mov_ra16(Name16::HL, word_arg_from(instruction)), // LHLD a16
        0x2b => s.r.update16(Name16::HL, dec16),                    // DCX H
        0x2c => s.unary_math_r8(Name8::L, inc8),                    // INR L
        0x2d => s.unary_math_r8(Name8::L, dec8),                    // DCR L
        0x2e => s.mov_ri8(Name8::L, byte_arg_from(instruction)),    // MVI L,byte
        0x2f => s.r.update8(Name8::A, |a| !a),                      // CMA

        0x30 => (),                                              // NOP
        0x31 => s.s.set_sp(word_arg_from(instruction)),          // LXI SP,word
        0x32 => s.mov_ar8(word_arg_from(instruction), Name8::A), // STA a16
        0x33 => {
            // INX SP
            s.s.set_sp(s.s.get_sp() + 1);
        }
        0x34 => {
            // INR M
            let new_value = s.get_indirect8(Name16::HL).wrapping_add(1);
            s.r.cc.set_flags_no_carry(new_value);
            s.mov_pi8(Name16::HL, new_value);
        }
        0x35 => {
            // DCR M
            let new_value = s.get_indirect8(Name16::HL).wrapping_sub(1);
            s.r.cc.set_flags_no_carry(new_value);
            s.mov_pi8(Name16::HL, new_value);
        }
        0x36 => s.mov_pi8(Name16::HL, byte_arg_from(instruction)), // MVI M,byte
        0x37 => s.r.cc.cy = true,                                  // STC
        0x38 => (),                                                // NOP
        0x39 => s.add_ri16(s.s.get_sp()),                          // DAD SP
        0x3a => s.mov_ra8(Name8::A, word_arg_from(instruction)),   // LDA a16
        0x3b => {
            // DCX SP
            s.s.set_sp(s.s.get_sp() - 1);
        }
        0x3c => s.unary_math_r8(Name8::A, inc8), // INR A
        0x3d => s.unary_math_r8(Name8::A, dec8), // DCR A
        0x3e => s.mov_ri8(Name8::A, byte_arg_from(instruction)), // MVI A,byte
        0x3f => s.r.cc.cy = !s.r.cc.cy,          // CMC
        _ => panic!("Unknown opcode"),
    }
}

fn emulate_group3(instruction: &[u8], s: &mut State, m: &mut impl Machine) {
    let opcode = instruction[0];
    match opcode & 0x7 {
        0x0 => s.ret_if(instruction),
        0x1 => match (opcode >> 3) & 0x7 {
            0x0 => s.pop_r16(Name16::BC),             // POP B
            0x1 | 0x3 => s.ret(),                     // RET
            0x2 => s.pop_r16(Name16::DE),             // POP D
            0x4 => s.pop_r16(Name16::HL),             // POP H
            0x5 => s.jump_a(s.r.get16(Name16::HL)),   // PCHL
            0x6 => s.pop_r16(Name16::AF),             // POP PSW
            0x7 => s.s.set_sp(s.r.get16(Name16::HL)), // SPHL

            _ => panic!("Shouldn't happen"),
        },
        0x2 => s.jump_if(instruction),
        0x3 => match (opcode >> 3) & 0x7 {
            0x0 | 0x1 => s.jump_a(word_arg_from(instruction)), // JMP a16
            0x2 => m.output(byte_arg_from(instruction), s.r.get8(Name8::A)), // OUT byte
            0x3 => s.mov_ri8(Name8::A, m.input(byte_arg_from(instruction))), // IN byte
            0x4 => {
                // XTHL
                let new_hl = s.pop_word();
                let old_hl = s.r.get16(Name16::HL);
                s.push_word(old_hl);
                s.r.set16(Name16::HL, new_hl);
            }
            0x5 => {
                // XCHG
                std::mem::swap(&mut s.r.d, &mut s.r.h);
                std::mem::swap(&mut s.r.e, &mut s.r.l);
            }
            0x6 => s.set_interrupt_flag(false), // DI
            0x7 => s.set_interrupt_flag(true),  // EI

            _ => panic!("Shouldn't happen"),
        },
        0x4 => s.call_if(instruction),
        0x5 => match (opcode >> 3) & 0x7 {
            0x0 => s.push_r16(Name16::BC), // PUSH B
            0x1 | 0x3 | 0x5 | 0x7 => s.call_a(word_arg_from(instruction)), // CALL a16
            0x2 => s.push_r16(Name16::DE), // PUSH D
            0x4 => s.push_r16(Name16::HL), // PUSH H
            0x6 => s.push_r16(Name16::AF), // PUSH PSW

            _ => panic!("Shouldn't happen"),
        },
        0x6 => operate8(s, opcode, byte_arg_from(instruction)),
        0x7 => s.call_a(u16::from(opcode & 0x38)),
        _ => panic!("Shouldn't happen"),
    }
}

pub fn emulate_instruction(s: &mut State, m: &mut impl Machine) -> usize {
    let instruction = s.get_instruction();
    let opcode = instruction[0];

    match opcode {
        0x00..=0x3f => emulate_group0(&instruction, s),
        0x40..=0x7f => mov_for(s, opcode),
        0x80..=0xbf => operate8(s, opcode, get_operand(s, opcode)),
        0xc0..=0xff => emulate_group3(&instruction, s, m),

        _ => unimplemented_instruction(s, opcode),
    }

    s.p.advance();
    OPCODE_TIMING[opcode as usize]
}
