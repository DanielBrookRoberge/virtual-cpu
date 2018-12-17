use virtual_cpu_8080::instructions::*;
use virtual_cpu_8080::registers::{Name16, Name8};
use virtual_cpu_8080::state::State8080 as State;
use virtual_cpu_core::{Memory, Program, Registers16, Registers8, Stack};

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
        0x01 => s.mov_ri16(Name16::BC, word_arg_from(instruction)), // LD BC,nn
        0x02 => s.mov_pr8(Name16::BC, Name8::B),                    // LD (BC), A
        0x03 => s.r.update16(Name16::BC, inc16),                    // INC BC
        0x04 => s.unary_math_r8(Name8::B, inc8),                    // INC B
        0x05 => s.unary_math_r8(Name8::B, dec8),                    // DEC B
        0x06 => s.mov_ri8(Name8::B, byte_arg_from(instruction)),    // LD B,n
        0x07 => {
            // RLC A
            let x = s.r.get8(Name8::A);
            s.r.set8(Name8::A, x.rotate_left(1));
            s.r.cc.cy = (x & 0x80) != 0;
        }
        0x08 => s.m.set_word(word_arg_from(instruction), s.s.get_sp()), // LD (nn),SP
        0x09 => s.add_rr16(Name16::BC),                                 // ADD HL,BC
        0x0a => s.mov_rp8(Name8::A, Name16::BC),                        // LD A,(BC)
        0x0b => s.r.update16(Name16::BC, dec16),                        // DEC BC
        0x0c => s.unary_math_r8(Name8::C, inc8),                        // INC C
        0x0d => s.unary_math_r8(Name8::C, dec8),                        // DEC C
        0x0e => s.mov_ri8(Name8::C, byte_arg_from(instruction)),        // LD C,n
        0x0f => {
            // RRC
            let x = s.r.get8(Name8::A);
            s.r.set8(Name8::A, x.rotate_right(1));
            s.r.cc.cy = (x & 0x01) == 1;
        }

        0x10 => unimplemented_instruction(s, opcode), // STOP
        0x11 => s.mov_ri16(Name16::DE, word_arg_from(instruction)), // LD D,nn
        0x12 => s.mov_pr8(Name16::DE, Name8::A),      // LD (DE),A
        0x13 => s.r.update16(Name16::DE, inc16),      // INC DE
        0x14 => s.unary_math_r8(Name8::D, inc8),      // INC D
        0x15 => s.unary_math_r8(Name8::D, dec8),      // DEC D
        0x16 => s.mov_ri8(Name8::D, byte_arg_from(instruction)), // LD D,n
        0x17 => {
            // RL A
            let x = s.r.get8(Name8::A);
            s.r.a = (s.r.cc.cy as u8) | (x << 1);
            s.r.cc.cy = (x & 0x80) != 0;
        }
        0x18 => unimplemented_instruction(s, opcode), // JR n
        0x19 => s.add_rr16(Name16::DE),               // ADD HL,DE
        0x1a => s.mov_rp8(Name8::A, Name16::DE),      // LD A,(DE)
        0x1b => s.r.update16(Name16::DE, dec16),      // DEC DE
        0x1c => s.unary_math_r8(Name8::E, inc8),      // INC E
        0x1d => s.unary_math_r8(Name8::E, dec8),      // DEC E
        0x1e => s.mov_ri8(Name8::E, byte_arg_from(instruction)), // LD E,n
        0x1f => {
            // RR A
            let x = s.r.get8(Name8::A);
            s.r.a = ((s.r.cc.cy as u8) << 7) | (x >> 1);
            s.r.cc.cy = (x & 0x01) == 1;
        }

        0x20 => unimplemented_instruction(s, opcode), // JR NZ,n
        0x21 => s.mov_ri16(Name16::HL, word_arg_from(instruction)), // LD HL,nn
        0x22 => {
            // LDI (HL),A
            s.mov_pr8(Name16::HL, Name8::A);
            s.r.update16(Name16::HL, inc16);
        }
        0x23 => s.r.update16(Name16::HL, inc16), // INC HL
        0x24 => s.unary_math_r8(Name8::H, inc8), // INC H
        0x25 => s.unary_math_r8(Name8::H, dec8), // DEC H
        0x26 => s.mov_ri8(Name8::H, byte_arg_from(instruction)), // LD H,n
        0x27 => {
            // DAA
            if (s.r.get8(Name8::A) & 0x0f) > 9 {
                s.r.a += 6;
            }
            if (s.r.get8(Name8::A) & 0xf0) > 0x90 {
                s.add_ri8(0x60);
            }
        }
        0x28 => unimplemented_instruction(s, opcode), // JR Z,n
        0x29 => s.add_rr16(Name16::HL),               // ADD HL,HL
        0x2a => {
            // LDI A,(HL)
            s.mov_rp8(Name8::A, Name16::HL);
            s.r.update16(Name16::HL, inc16);
        }
        0x2b => s.r.update16(Name16::HL, dec16), // DEC HL
        0x2c => s.unary_math_r8(Name8::L, inc8), // INC L
        0x2d => s.unary_math_r8(Name8::L, dec8), // DEC L
        0x2e => s.mov_ri8(Name8::L, byte_arg_from(instruction)), // LD L,n
        0x2f => s.r.update8(Name8::A, |a| !a),   // CPL

        0x30 => unimplemented_instruction(s, opcode), // JR NC,n
        0x31 => s.s.set_sp(word_arg_from(instruction)), // LD SP,nn
        0x32 => {
            // LDD (HL),A
            s.mov_pr8(Name16::HL, Name8::A);
            s.r.update16(Name16::HL, dec16);
        }
        0x33 => s.s.set_sp(s.s.get_sp() + 1), // INC SP
        0x34 => {
            // INC (HL)
            let new_value = s.get_indirect8(Name16::HL).wrapping_add(1);
            s.r.cc.set_flags_no_carry(new_value);
            s.mov_pi8(Name16::HL, new_value);
        }
        0x35 => {
            // DEC (HL)
            let new_value = s.get_indirect8(Name16::HL).wrapping_sub(1);
            s.r.cc.set_flags_no_carry(new_value);
            s.mov_pi8(Name16::HL, new_value);
        }
        0x36 => s.mov_pi8(Name16::HL, byte_arg_from(instruction)), // LD (HL),n
        0x37 => s.r.cc.cy = true,                                  // SCF
        0x38 => unimplemented_instruction(s, opcode),              // JR C,n
        0x39 => s.add_ri16(s.s.get_sp()),                          // ADD HL,SP
        0x3a => {
            // LDD A,(HL)
            s.mov_rp8(Name8::A, Name16::HL);
            s.r.update16(Name16::HL, dec16);
        }
        0x3b => s.s.set_sp(s.s.get_sp() - 1),    // DEC SP
        0x3c => s.unary_math_r8(Name8::A, inc8), // INC A
        0x3d => s.unary_math_r8(Name8::A, dec8), // DEC A
        0x3e => s.mov_ri8(Name8::A, byte_arg_from(instruction)), // LD A,n
        0x3f => s.r.cc.cy = false,               // CCF
        _ => panic!("Unknown opcode"),
    }
}

fn emulate_group3(instruction: &[u8], s: &mut State) {
    let opcode = instruction[0];
    match opcode {
        0xc0 => s.ret_if(instruction),                 // RET NZ (implicitly)
        0xc1 => s.pop_r16(Name16::BC),                 // POP BC
        0xc2 => s.jump_if(instruction),                // JP NZ,nn (implicitly)
        0xc3 => s.jump_a(word_arg_from(instruction)),  // JP nn
        0xc4 => s.call_if(instruction),                // CALL NZ,nn (implicitly)
        0xc5 => s.push_r16(Name16::BC),                // PUSH BC
        0xc6 => s.add_ri8(byte_arg_from(instruction)), // ADD A,n
        0xc7 => s.call_a(0x0000),                      // RST 0
        0xc8 => s.ret_if(instruction),                 // RET Z (implicitly)
        0xc9 => s.ret(),                               // RET
        0xca => s.jump_if(instruction),                // JP Z,nn (implicitly)
        0xcb => unimplemented_instruction(s, opcode),  // Prefix
        0xcc => s.call_if(instruction),                // CALL Z,nn (implicitly)
        0xcd => s.call_a(word_arg_from(instruction)),  // CALL nn
        0xce => s.adc_ri8(byte_arg_from(instruction)), // ADC A,n
        0xcf => s.call_a(0x0008),                      // RST 8

        0xd0 => s.ret_if(instruction),  // RET NC (implicitly)
        0xd1 => s.pop_r16(Name16::DE),  // POP DE
        0xd2 => s.jump_if(instruction), // JP NC,nn (implicitly)
        0xd3 => unimplemented_instruction(s, opcode), // No instruction
        0xd4 => s.call_if(instruction), // CALL NC,nn (implicitly)
        0xd5 => s.push_r16(Name16::DE), // PUSH DE
        0xd6 => s.sub_ri8(byte_arg_from(instruction)), // SUB A,n
        0xd7 => s.call_a(0x0010),       // RST 10
        0xd8 => s.ret_if(instruction),  // RET C (implicitly)
        0xd9 => unimplemented_instruction(s, opcode), // RETI
        0xda => s.jump_if(instruction), // JP C,nn (implicitly)
        0xdb => unimplemented_instruction(s, opcode), // No instruction
        0xdc => s.call_if(instruction), // CALL C,nn (implicitly)
        0xdd => unimplemented_instruction(s, opcode), // No instruction
        0xde => s.sbb_ri8(byte_arg_from(instruction)), // SBC A,n
        0xdf => s.call_a(0x0018),       // RST 18

        0xe0 => unimplemented_instruction(s, opcode), // LDH (n),A
        0xe1 => s.pop_r16(Name16::HL),                // POP HL
        0xe2 => unimplemented_instruction(s, opcode), // LDH (C),A
        0xe3 => unimplemented_instruction(s, opcode), // No instruction
        0xe4 => unimplemented_instruction(s, opcode), // No instruction
        0xe5 => s.push_r16(Name16::HL),               // PUSH HL
        0xe6 => s.logical_operation_ri(byte_arg_from(instruction), and8), // AND n
        0xe7 => s.call_a(0x0020),                     // RST 20
        0xe8 => unimplemented_instruction(s, opcode), // ADD SP,d
        0xe9 => unimplemented_instruction(s, opcode), // JP (HL)
        0xea => s.mov_ar8(word_arg_from(instruction), Name8::A), // LD (nn),A
        0xeb => unimplemented_instruction(s, opcode), // No instruction
        0xec => unimplemented_instruction(s, opcode), // No instruction
        0xed => unimplemented_instruction(s, opcode), // No instruction
        0xee => s.logical_operation_ri(byte_arg_from(instruction), xor8), // XOR n
        0xef => s.call_a(0x0028),                     // RST 28

        0xf0 => unimplemented_instruction(s, opcode), // LDH A,(n)
        0xf1 => s.pop_r16(Name16::AF),                // POP AF
        0xf2 => unimplemented_instruction(s, opcode), // No instruction
        0xf3 => s.set_interrupt_flag(false),          // DI
        0xf4 => unimplemented_instruction(s, opcode), // No instruction
        0xf5 => s.push_r16(Name16::AF),               // PUSH AF
        0xf6 => s.logical_operation_ri(byte_arg_from(instruction), or8), // OR n
        0xf7 => s.call_a(0x0030),                     // RST 30
        0xf8 => unimplemented_instruction(s, opcode), // LDHL SP,d
        0xf9 => s.s.set_sp(s.r.get16(Name16::HL)),    // LD SP,HL
        0xfa => s.mov_ra8(Name8::A, word_arg_from(instruction)), // LD A,(nn)
        0xfb => s.set_interrupt_flag(true),           // EI
        0xfc => unimplemented_instruction(s, opcode), // No instruction
        0xfd => unimplemented_instruction(s, opcode), // No instruction
        0xfe => s.cmp_ri8(byte_arg_from(instruction)), // CP n
        0xff => s.call_a(0x0030),                     // RST 30

        _ => panic!("Shouldn't happen"),
    }
}

pub fn emulate_instruction(s: &mut State) -> usize {
    let instruction = s.get_instruction();
    let opcode = instruction[0];

    match opcode {
        0x00..=0x3f => emulate_group0(&instruction, s),
        0x40..=0x7f => mov_for(s, opcode),
        0x80..=0xbf => operate8(s, opcode, get_operand(s, opcode)),
        0xc0..=0xff => emulate_group3(&instruction, s),

        _ => unimplemented_instruction(s, opcode),
    }

    s.p.advance();
    OPCODE_TIMING[opcode as usize]
}
