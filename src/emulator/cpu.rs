use super::data_types::{self, CYCLE_TABLE};

pub struct CPU {
    pub state: data_types::State8080,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            state: data_types::State8080::default(),
        }
    }

    pub fn init_rom(&mut self, rom: Vec<u8>) {
        for (index, &byte) in rom.iter().enumerate() {
            self.state.memory[index] = byte;
        }
    }

    pub fn run(&mut self) {
        loop {
            emulate_8080_op(&mut self.state);
        }
    }

    pub fn get_video_memory(&self) -> &[u8] {
        &self.state.memory[0x2400..0x4000]
    }

    pub fn get_state(&self) -> &data_types::State8080 {
        &self.state
    }
}

pub fn unimplemented_instruction(instruction: u8) {
    panic!("Error: Unimplemented instruction 0x{:02X}\n", instruction);
}

pub fn emulate_8080_op(state: &mut data_types::State8080) -> u8 {
    let op_codes = &state.memory[state.pc as usize..];
    let op_code = op_codes[0];
    // print!("PC: {:04X}  ", state.pc); // Print the address in hex
    // println!("{:02X} ", op_code); // Print the opcode in hex

    match op_code {
        0x00 => nop(state),
        0x01 => lxi(state, data_types::RegisterPair::BC),
        0x02 => stax(state, data_types::RegisterPair::BC),
        0x03 => inx(state, &data_types::RegisterPair::BC),
        0x04 => inr(state, data_types::Register::B),
        0x05 => dcr(state, data_types::Register::B),
        0x06 => mvi(state, data_types::Register::B),
        0x07 => rlc(state),
        0x08 => unimplemented_instruction(op_code),
        0x09 => dad(state, &data_types::RegisterPair::BC),
        0x0A => ldax(state, data_types::RegisterPair::BC),
        0x0B => dcx(state, &data_types::RegisterPair::BC),
        0x0C => inr(state, data_types::Register::C),
        0x0D => dcr(state, data_types::Register::C),
        0x0E => mvi(state, data_types::Register::C),
        0x0F => rrc(state),
        0x10 => unimplemented_instruction(op_code),
        0x11 => lxi(state, data_types::RegisterPair::DE),
        0x12 => stax(state, data_types::RegisterPair::DE),
        0x13 => inx(state, &data_types::RegisterPair::DE),
        0x14 => inr(state, data_types::Register::D),
        0x15 => dcr(state, data_types::Register::D),
        0x16 => mvi(state, data_types::Register::D),
        0x17 => ral(state),
        0x18 => unimplemented_instruction(op_code),
        0x19 => dad(state, &data_types::RegisterPair::DE),
        0x1A => ldax(state, data_types::RegisterPair::DE),
        0x1B => dcx(state, &data_types::RegisterPair::DE),
        0x1C => inr(state, data_types::Register::E),
        0x1D => dcr(state, data_types::Register::E),
        0x1E => mvi(state, data_types::Register::E),
        0x1F => rar(state),
        0x20 => unimplemented_instruction(op_code),
        0x21 => lxi(state, data_types::RegisterPair::HL),
        0x22 => shld(state),
        0x23 => inx(state, &data_types::RegisterPair::HL),
        0x24 => inr(state, data_types::Register::H),
        0x25 => dcr(state, data_types::Register::H),
        0x26 => mvi(state, data_types::Register::H),
        0x27 => daa(state),
        0x28 => unimplemented_instruction(op_code),
        0x29 => dad(state, &data_types::RegisterPair::HL),
        0x2A => lhld(state),
        0x2B => dcx(state, &data_types::RegisterPair::HL),
        0x2C => inr(state, data_types::Register::L),
        0x2D => dcr(state, data_types::Register::L),
        0x2E => mvi(state, data_types::Register::L),
        0x2F => cma(state),
        0x30 => unimplemented_instruction(op_code),
        0x31 => lxi(state, data_types::RegisterPair::SP),
        0x32 => sta(state),
        0x33 => inx(state, &data_types::RegisterPair::SP),
        0x34 => inr(state, data_types::Register::M),
        0x35 => dcr_m(state),
        0x36 => mvi_m(state),
        0x37 => stc(state),
        0x38 => unimplemented_instruction(op_code),
        0x39 => dad(state, &data_types::RegisterPair::SP),
        0x3A => lda(state),
        0x3B => dcx(state, &data_types::RegisterPair::SP),
        0x3C => inr(state, data_types::Register::A),
        0x3D => dcr(state, data_types::Register::A),
        0x3E => mvi(state, data_types::Register::A),
        0x3F => cmc(state),
        // Handle MOV and HLT instructions (0x40 to 0x7F)
        0x40..=0x7F => {
            if op_codes[0] == 0x76 {
                hlt(state);
            } else {
                let dest = (op_codes[0] >> 3) & 0x07;
                let src = op_codes[0] & 0x07;
                let dest_reg = data_types::Register::from_u8(dest);
                let src_reg = data_types::Register::from_u8(src);
                mov(state, dest_reg, src_reg);
            }
        }
        // ADD, ADC instructions
        0x80..=0x87 => {
            let src = op_codes[0] & 0x07;
            let src_reg = data_types::Register::from_u8(src);
            add(state, src_reg);
        }
        0x88..=0x8F => {
            let src = op_codes[0] & 0x07;
            let src_reg = data_types::Register::from_u8(src);
            adc(state, src_reg);
        }
        // SUB, SBB instructions
        0x90..=0x97 => {
            let src = op_codes[0] & 0x07;
            let src_reg = data_types::Register::from_u8(src);
            sub(state, src_reg);
        }
        0x98..=0x9F => {
            let src = op_codes[0] & 0x07;
            let src_reg = data_types::Register::from_u8(src);
            sbb(state, src_reg);
        }
        // ANA, XRA, ORA, CMP instructions
        0xA0..=0xA7 => {
            let src = op_codes[0] & 0x07;
            let src_reg = data_types::Register::from_u8(src);
            ana(state, src_reg);
        }
        0xA8..=0xAF => {
            let src = op_codes[0] & 0x07;
            let src_reg = data_types::Register::from_u8(src);
            xra(state, src_reg);
        }
        0xB0..=0xB7 => {
            let src = op_codes[0] & 0x07;
            let src_reg = data_types::Register::from_u8(src);
            ora(state, src_reg);
        }
        0xB8..=0xBF => {
            let src = op_codes[0] & 0x07;
            let src_reg = data_types::Register::from_u8(src);
            cmp(state, src_reg);
        }
        // Control and other instructions
        0xC0 => rnz(state),
        0xC1 => pop(state, data_types::RegisterPair::BC),
        0xC2 => jnz(state),
        0xC3 => jmp(state),
        0xC4 => cnz(state),
        0xC5 => push(state, data_types::RegisterPair::BC),
        0xC6 => adi(state),
        0xC7 => rst(state, 0),
        0xC8 => rz(state),
        0xC9 => ret(state),
        0xCA => jz(state),
        0xCB => unimplemented_instruction(op_code),
        0xCC => cz(state),
        0xCD => call(state),
        0xCE => aci(state),
        0xCF => rst(state, 1),
        0xD0 => rnc(state),
        0xD1 => pop(state, data_types::RegisterPair::DE),
        0xD2 => jnc(state),
        0xD3 => out(state),
        0xD4 => cnc(state),
        0xD5 => push(state, data_types::RegisterPair::DE),
        0xD6 => sui(state),
        0xD7 => rst(state, 2),
        0xD8 => rc(state),
        0xD9 => unimplemented_instruction(op_code),
        0xDA => jc(state),
        0xDB => inp(state),
        0xDC => cc(state),
        0xDD => unimplemented_instruction(op_code),
        0xDE => sbi(state),
        0xDF => rst(state, 3),
        0xE0 => rpo(state),
        0xE1 => pop(state, data_types::RegisterPair::HL),
        0xE2 => jpo(state),
        0xE3 => xthl(state),
        0xE4 => cpo(state),
        0xE5 => push(state, data_types::RegisterPair::HL),
        0xE6 => ani(state),
        0xE7 => rst(state, 4),
        0xE8 => rpe(state),
        0xE9 => pchl(state),
        0xEA => jpe(state),
        0xEB => xchg(state),
        0xEC => cpe(state),
        0xED => unimplemented_instruction(op_code),
        0xEE => xri(state),
        0xEF => rst(state, 5),
        0xF0 => rp(state),
        0xF1 => pop_psw(state),
        0xF2 => jp(state),
        0xF3 => di(state),
        0xF4 => cp(state),
        0xF5 => push_psw(state),
        0xF6 => ori(state),
        0xF7 => rst(state, 6),
        0xF8 => rm(state),
        0xF9 => sphl(state),
        0xFA => jm(state),
        0xFB => ei(state),
        0xFC => cm(state),
        0xFD => unimplemented_instruction(op_code),
        0xFE => cpi(state),
        0xFF => rst(state, 7),
    }
    return CYCLE_TABLE[op_code as usize];
}

#[inline(always)]
fn nop(state: &mut data_types::State8080) {
    // No operation
    state.pc += 1;
}

#[inline(always)]
fn lxi(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    let low: u8 = state.memory[(state.pc + 1) as usize];
    let high = state.memory[(state.pc + 2) as usize];
    match register_pair {
        data_types::RegisterPair::BC => {
            state.c = low;
            state.b = high;
        }
        data_types::RegisterPair::DE => {
            state.e = low;
            state.d = high;
        }
        data_types::RegisterPair::HL => {
            state.l = low;
            state.h = high;
        }
        data_types::RegisterPair::SP => {
            state.sp = ((high as u16) << 8) | (low as u16);
        }
        data_types::RegisterPair::PSW => {
            panic!("PSW line called in lxi instruction");
        }
    }
    state.pc += 3;
}

#[inline(always)]
fn stax(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    let addr = match register_pair {
        data_types::RegisterPair::BC => get_bc(state),
        data_types::RegisterPair::DE => get_de(state),
        _ => panic!("Invalid register pair for STAX instruction"),
    } as usize;
    state.memory[addr] = state.a;
    state.pc += 1;
}

#[inline(always)]
fn inx(state: &mut data_types::State8080, register_pair: &data_types::RegisterPair) {
    let value = get_register_pair_value(state, register_pair);
    let result = value.wrapping_add(1);
    set_register_pair_value(state, register_pair, result);
    state.pc += 1;
}

#[inline(always)]
fn inr(state: &mut data_types::State8080, register: data_types::Register) {
    match register {
        data_types::Register::A => {
            let result = state.a.wrapping_add(1);
            set_flags_inr(state, state.a, result);
            state.a = result;
        }
        data_types::Register::B => {
            let result = state.b.wrapping_add(1);
            set_flags_inr(state, state.b, result);
            state.b = result;
        }
        data_types::Register::C => {
            let result = state.c.wrapping_add(1);
            set_flags_inr(state, state.c, result);
            state.c = result;
        }
        data_types::Register::D => {
            let result = state.d.wrapping_add(1);
            set_flags_inr(state, state.d, result);
            state.d = result;
        }
        data_types::Register::E => {
            let result = state.e.wrapping_add(1);
            set_flags_inr(state, state.e, result);
            state.e = result;
        }
        data_types::Register::H => {
            let result = state.h.wrapping_add(1);
            set_flags_inr(state, state.h, result);
            state.h = result;
        }
        data_types::Register::L => {
            let result = state.l.wrapping_add(1);
            set_flags_inr(state, state.l, result);
            state.l = result;
        }
        data_types::Register::M => {
            let addr = get_memory_address(state);
            let value = state.memory[addr];
            let result = value.wrapping_add(1);
            set_flags_inr(state, value, result);
            state.memory[addr] = result;
        }
    }
    state.pc += 1;
}

#[inline(always)]
fn dcr(state: &mut data_types::State8080, register: data_types::Register) {
    let value = match register {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr]
        }
    };

    let result = value.wrapping_sub(1);

    match register {
        data_types::Register::A => state.a = result,
        data_types::Register::B => state.b = result,
        data_types::Register::C => state.c = result,
        data_types::Register::D => state.d = result,
        data_types::Register::E => state.e = result,
        data_types::Register::H => state.h = result,
        data_types::Register::L => state.l = result,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr] = result;
        }
    };

    set_flags_dcr(state, value, result);
    state.pc += 1;
}

#[inline(always)]
fn mvi(state: &mut data_types::State8080, register: data_types::Register) {
    let val = state.memory[(state.pc + 1) as usize];
    match register {
        data_types::Register::A => state.a = val,
        data_types::Register::B => state.b = val,
        data_types::Register::C => state.c = val,
        data_types::Register::D => state.d = val,
        data_types::Register::E => state.e = val,
        data_types::Register::H => state.h = val,
        data_types::Register::L => state.l = val,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr] = val;
        }
    }
    state.pc += 2;
}

#[inline(always)]
fn rlc(state: &mut data_types::State8080) {
    // TODO: Double check this implementation
    let x = state.a;
    state.a = (x << 1) | (x >> 7);
    state.cc.cy = (x & 0x80) == 0x80;
    state.pc += 1;
}

#[inline(always)]
fn dad(state: &mut data_types::State8080, register_pair: &data_types::RegisterPair) {
    let hl = get_hl(state);
    let rp_value = get_register_pair_value(state, register_pair);
    let result = hl.wrapping_add(rp_value);
    set_hl(state, result);

    // Set Carry flag if there's an overflow from bit 15
    state.cc.cy = (hl as u32 + rp_value as u32) > 0xFFFF;
    state.pc += 1;
}

#[inline(always)]
fn ldax(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    let addr = match register_pair {
        data_types::RegisterPair::BC => get_bc(state),
        data_types::RegisterPair::DE => get_de(state),
        _ => panic!("Invalid register pair for LDAX instruction"),
    } as usize;
    state.a = state.memory[addr];
    state.pc += 1;
}

#[inline(always)]
fn dcx(state: &mut data_types::State8080, register_pair: &data_types::RegisterPair) {
    let value = get_register_pair_value(state, register_pair);
    let result = value.wrapping_sub(1);
    set_register_pair_value(state, register_pair, result);
    // Note: DCX does not affect any flags
    state.pc += 1;
}

#[inline(always)]
fn rrc(state: &mut data_types::State8080) {
    let x = state.a;
    state.a = (x >> 1) | (x << 7); // Rotate right through carry
    state.cc.cy = (x & 0x01) == 0x01; // Carry is the bit shifted out
    state.pc += 1;
}

#[inline(always)]
fn ral(state: &mut data_types::State8080) {
    let x = state.a;
    state.a = (x << 1) | (state.cc.cy as u8);
    state.cc.cy = (x & 0x80) == 0x80;
    state.pc += 1;
}

#[inline(always)]
fn rar(state: &mut data_types::State8080) {
    let x = state.a;
    state.a = (x >> 1) | (state.cc.cy as u8) << 7;
    state.cc.cy = (x & 1) == 1;
    state.pc += 1;
}

#[inline(always)]
fn shld(state: &mut data_types::State8080) {
    let addr = get_jmp_target_address(state);
    state.memory[addr as usize] = state.l;
    state.memory[(addr + 1) as usize] = state.h;
    state.pc += 3;
}

#[inline(always)]
fn daa(state: &mut data_types::State8080) {
    let mut temp: u16 = state.a as u16;
    if state.cc.ac || (temp & 0x0F) > 9 {
        temp = temp.wrapping_add(6);
    }
    if state.cc.cy || temp > 0x9F {
        temp = temp.wrapping_add(0x60);
        state.cc.cy = true;
    }
    state.a = (temp & 0xFF) as u8;
    set_flags_daa(state, temp);
    state.pc += 1;
}

#[inline(always)]
fn lhld(state: &mut data_types::State8080) {
    let addr = get_jmp_target_address(state);
    state.l = state.memory[addr as usize];
    state.h = state.memory[(addr + 1) as usize];
    state.pc += 3;
}

#[inline(always)]
fn cma(state: &mut data_types::State8080) {
    state.a = !state.a;
    state.pc += 1;
}

#[inline(always)]
fn sta(state: &mut data_types::State8080) {
    state.memory[get_jmp_target_address(state) as usize] = state.a;
    state.pc += 3;
}

#[inline(always)]
fn dcr_m(state: &mut data_types::State8080) {
    let addr = get_memory_address(state);
    let val = state.memory[addr];
    let res = val.wrapping_sub(1);
    state.memory[addr] = res;
    set_flags_dcr(state, val, res);
    state.pc += 1;
}

#[inline(always)]
fn mvi_m(state: &mut data_types::State8080) {
    let val = state.memory[(state.pc + 1) as usize];
    let addr = get_memory_address(state);
    state.memory[addr] = val;
    state.pc += 2;
}

#[inline(always)]
fn stc(state: &mut data_types::State8080) {
    state.cc.cy = true;
    state.pc += 1;
}

#[inline(always)]
fn lda(state: &mut data_types::State8080) {
    state.a = state.memory[get_jmp_target_address(state) as usize];
    state.pc += 3;
}

#[inline(always)]
fn cmc(state: &mut data_types::State8080) {
    state.cc.cy = !state.cc.cy;
    state.pc += 1;
}

#[inline(always)]
fn hlt(state: &mut data_types::State8080) {
    unimplemented_instruction(state.memory[state.pc as usize]);
}

#[inline(always)]
fn mov(state: &mut data_types::State8080, dest: data_types::Register, src: data_types::Register) {
    let src_value = match src {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let offset = (state.h as u16) << 8 | state.l as u16;
            state.memory[offset as usize]
        }
    };
    match dest {
        data_types::Register::A => state.a = src_value,
        data_types::Register::B => state.b = src_value,
        data_types::Register::C => state.c = src_value,
        data_types::Register::D => state.d = src_value,
        data_types::Register::E => state.e = src_value,
        data_types::Register::H => state.h = src_value,
        data_types::Register::L => state.l = src_value,
        data_types::Register::M => {
            let offset = (state.h as u16) << 8 | state.l as u16;
            state.memory[offset as usize] = src_value;
        }
    };
    state.pc += 1;
}

#[inline(always)]
fn add(state: &mut data_types::State8080, src: data_types::Register) {
    let val = match src {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr]
        }
    };
    let answer = state.a as u16 + val as u16;
    set_flag_add(state, answer);
    state.a = (answer & 0xff) as u8;
    state.pc += 1;
}

#[inline(always)]
fn adc(state: &mut data_types::State8080, src: data_types::Register) {
    let val = match src {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr]
        }
    };
    let answer = state.a as u16 + val as u16 + state.cc.cy as u16;
    set_flags_adc(state, answer);
    state.a = (answer & 0xff) as u8;
    state.pc += 1;
}

#[inline(always)]
fn sub(state: &mut data_types::State8080, src: data_types::Register) {
    let val = match src {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr]
        }
    };
    let result = state.a.wrapping_sub(val);
    state.cc.z = result == 0;
    state.cc.s = (result & 0x80) != 0;
    state.cc.p = parity(result);
    state.cc.cy = state.a < val;
    state.cc.ac = (state.a & 0x0F) < (val & 0x0F);
    state.a = result;
    state.pc += 1;
}

#[inline(always)]
fn sbb(state: &mut data_types::State8080, src: data_types::Register) {
    let val = match src {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr]
        }
    };
    state.a = state.a.wrapping_sub(val).wrapping_sub(state.cc.cy as u8);
    state.cc.z = state.a == 0;
    state.cc.s = (state.a & 0x80) != 0;
    state.cc.p = parity(state.a);
    state.cc.cy = state.a < val;
    state.cc.ac = (state.a & 0x0F) < (val & 0x0F);
    state.pc += 1;
}

#[inline(always)]
fn ana(state: &mut data_types::State8080, src: data_types::Register) {
    let val = match src {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr]
        }
    };
    state.a &= val;
    state.cc.z = state.a == 0;
    state.cc.s = (state.a & 0x80) != 0;
    state.cc.p = parity(state.a);
    state.cc.cy = false;
    state.cc.ac = (state.a & 0x10) != 0;
    state.pc += 1;
}

#[inline(always)]
fn xra(state: &mut data_types::State8080, src: data_types::Register) {
    let val = match src {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr]
        }
    };
    state.a ^= val;
    // Update the condition flags
    state.cc.z = state.a == 0;
    state.cc.s = (state.a & 0x80) != 0;
    state.cc.p = parity(state.a);
    state.cc.cy = false; // Carry flag is reset
    state.cc.ac = false; // Auxiliary Carry flag is reset
    state.pc += 1;
}

#[inline(always)]
fn ora(state: &mut data_types::State8080, src: data_types::Register) {
    let val = match src {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr]
        }
    };
    state.a |= val;
    state.cc.z = state.a == 0;
    state.cc.s = (state.a & 0x80) != 0;
    state.cc.p = parity(state.a);
    state.cc.cy = false; // Carry flag is reset
    state.cc.ac = false; // Auxiliary Carry flag is reset
    state.pc += 1;
}

#[inline(always)]
fn cmp(state: &mut data_types::State8080, src: data_types::Register) {
    let value = match src {
        data_types::Register::A => state.a,
        data_types::Register::B => state.b,
        data_types::Register::C => state.c,
        data_types::Register::D => state.d,
        data_types::Register::E => state.e,
        data_types::Register::H => state.h,
        data_types::Register::L => state.l,
        data_types::Register::M => {
            let addr = get_memory_address(state);
            state.memory[addr]
        }
    };

    let result = state.a.wrapping_sub(value);

    state.cc.z = result == 0;
    state.cc.s = (result & 0x80) != 0;
    state.cc.p = parity(result);
    state.cc.cy = state.a < value;
    state.cc.ac = (state.a & 0x0F) < (value & 0x0F);
    state.pc += 1;
}

#[inline(always)]
fn rnz(state: &mut data_types::State8080) {
    if !state.cc.z {
        let return_address = pop_stack(state);
        state.pc = return_address;
    } else {
        state.pc += 1;
    }
}

#[inline(always)]
fn pop(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    let low = state.memory[state.sp as usize];
    let high = state.memory[(state.sp + 1) as usize];
    match register_pair {
        data_types::RegisterPair::BC => {
            state.b = high;
            state.c = low;
        }
        data_types::RegisterPair::DE => {
            state.d = high;
            state.e = low;
        }
        data_types::RegisterPair::HL => {
            state.h = high;
            state.l = low;
        }
        data_types::RegisterPair::PSW => {
            state.a = high;
            state.set_flags_from_byte(low); // Restore the flags from the stack
        }
        _ => {
            panic!("Invalid register call");
        }
    }
    state.sp += 2;
    state.pc += 1;
}

#[inline(always)]
fn jnz(state: &mut data_types::State8080) {
    if !state.cc.z {
        state.pc = get_jmp_target_address(state); // Jump if Zero flag is clear
    } else {
        state.pc += 3; // Skip the two bytes of the target address if no jump
    }
}

#[inline(always)]
fn jmp(state: &mut data_types::State8080) {
    state.pc = get_jmp_target_address(state);
}

#[inline(always)]
fn cnz(state: &mut data_types::State8080) {
    // also need to double check
    if !state.cc.z {
        call(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn push(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    let high: u8;
    let low: u8;
    match register_pair {
        data_types::RegisterPair::BC => {
            high = state.b;
            low = state.c;
        }
        data_types::RegisterPair::DE => {
            high = state.d;
            low = state.e;
        }
        data_types::RegisterPair::HL => {
            high = state.h;
            low = state.l;
        }
        data_types::RegisterPair::PSW => {
            high = state.a;
            low = state.get_flags_as_byte(); // Save PSW (Processor Status Word)
        }
        _ => {
            panic!("Invalid register call");
        }
    }
    state.memory[(state.sp - 1) as usize] = high;
    state.memory[(state.sp - 2) as usize] = low;
    state.sp -= 2;
    state.pc += 1;
}

#[inline(always)]
fn adi(state: &mut data_types::State8080) {
    // ADI instruction add next immediate value in memory (from what i understand)
    let answer: u16 = state.a as u16 + state.memory[(state.pc + 1) as usize] as u16;
    set_flag_add(state, answer);
    state.a = (answer & 0xff) as u8;
    state.pc += 2;
}

#[inline(always)]
fn rst(state: &mut data_types::State8080, num: u8) {
    let return_address = state.pc + 1;
    state.memory[(state.sp - 1) as usize] = ((return_address >> 8) & 0xff) as u8; // High byte
    state.memory[(state.sp - 2) as usize] = (return_address & 0xff) as u8; // Low byte
    state.sp -= 2;
    state.pc = num as u16 * 8;
}

#[inline(always)]
fn rz(state: &mut data_types::State8080) {
    if state.cc.z == true {
        let return_address = pop_stack(state);
        state.pc = return_address;
    } else {
        state.pc += 1;
    }
}

#[inline(always)]
fn ret(state: &mut data_types::State8080) {
    let low = state.memory[state.sp as usize] as u16;
    let high = state.memory[(state.sp + 1) as usize] as u16;
    state.pc = (high << 8) | low; // Pop the return address from the stack
    state.sp += 2;
}

#[inline(always)]
fn jz(state: &mut data_types::State8080) {
    let addr = get_jmp_target_address(state);
    if state.cc.z {
        state.pc = addr; // Jump if Zero flag is set
    } else {
        state.pc += 3; // Skip the two bytes of the target address if no jump
    }
}

#[inline(always)]
fn cz(state: &mut data_types::State8080) {
    if state.cc.z {
        call(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn call(state: &mut data_types::State8080) {
    let addr = get_jmp_target_address(state);
    let ret_addr = state.pc + 3; // Address after CALL
    state.memory[(state.sp - 1) as usize] = ((ret_addr >> 8) & 0xff) as u8; // High byte
    state.memory[(state.sp - 2) as usize] = (ret_addr & 0xff) as u8; // Low byte
    state.sp -= 2;
    state.pc = addr; // Jump to the target address
}

#[inline(always)]
fn aci(state: &mut data_types::State8080) {
    let imm = state.memory[(state.pc + 1) as usize];
    let carry = if state.cc.cy { 1 } else { 0 };
    let result = state.a.wrapping_add(imm).wrapping_add(carry);
    state.cc.z = result == 0;
    state.cc.s = (result & 0x80) != 0;
    state.cc.p = parity(result);
    state.cc.cy = (state.a as u16) + (imm as u16) + (carry as u16) > 0xFF;
    state.a = result;
    state.pc += 2;
}

#[inline(always)]
fn rnc(state: &mut data_types::State8080) {
    if !state.cc.cy {
        let return_address = pop_stack(state);
        state.pc = return_address;
    } else {
        state.pc += 1;
    }
}

#[inline(always)]
fn out(state: &mut data_types::State8080) {
    // TODO: examine this instruction
    let port = state.memory[(state.pc + 1) as usize];
    handle_out(state, port, state.a);
    // state.out_port3 = port;
    state.pc += 2;
}

#[inline(always)]
fn cnc(state: &mut data_types::State8080) {
    if !state.cc.cy {
        call(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn jnc(state: &mut data_types::State8080) {
    let addr = get_jmp_target_address(state);
    if !state.cc.cy {
        state.pc = addr; // Jump if Carry flag is clear
    } else {
        state.pc += 3; // Skip the two bytes of the target address if no jump
    }
}

#[inline(always)]
fn sui(state: &mut data_types::State8080) {
    let imm = state.memory[(state.pc + 1) as usize];
    let result = state.a.wrapping_sub(imm);
    state.cc.z = result == 0;
    state.cc.s = (result & 0x80) != 0;
    state.cc.p = parity(result);
    state.cc.cy = state.a < imm; // Set carry if there's a borrow
    state.a = result;
    state.pc += 2;
}

#[inline(always)]
fn rc(state: &mut data_types::State8080) {
    if state.cc.cy {
        let return_address = pop_stack(state);
        state.pc = return_address;
    } else {
        state.pc += 1;
    }
}

#[inline(always)]
fn jc(state: &mut data_types::State8080) {
    let addr = get_jmp_target_address(state);
    if state.cc.cy {
        state.pc = addr; // Jump if Carry flag is set
    } else {
        state.pc += 3; // Skip the two bytes of the target address if no jump
    }
}

#[inline(always)]
fn inp(state: &mut data_types::State8080) {
    let port = state.memory[(state.pc + 1) as usize];
    state.a = handle_in(state, port);
    // state.in_port1 = port;
    state.pc += 2;
}

#[inline(always)]
fn cc(state: &mut data_types::State8080) {
    if state.cc.cy {
        call(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn sbi(state: &mut data_types::State8080) {
    let imm = state.memory[(state.pc + 1) as usize];
    let borrow = if state.cc.cy { 1 } else { 0 };
    let result = state.a.wrapping_sub(imm).wrapping_sub(borrow);

    state.cc.z = result == 0; // Zero flag
    state.cc.s = (result & 0x80) != 0; // Sign flag
    state.cc.p = parity(result); // Parity flag
    state.cc.cy = (state.a as u16) < (imm as u16 + borrow as u16); // Carry flag (if borrow)

    state.a = result;
    state.pc += 2; // Increment the program counter by 2 because `SBI` uses the immediate value
}

#[inline(always)]
fn rpo(state: &mut data_types::State8080) {
    if !state.cc.p {
        let return_address = pop_stack(state);
        state.pc = return_address;
    } else {
        state.pc += 1;
    }
}

#[inline(always)]
fn xthl(state: &mut data_types::State8080) {
    let temp_l = state.l;
    let temp_h = state.h;
    state.l = state.memory[state.sp as usize];
    state.h = state.memory[(state.sp + 1) as usize];
    state.memory[state.sp as usize] = temp_l;
    state.memory[(state.sp + 1) as usize] = temp_h;
    state.pc += 1;
}

#[inline(always)]
fn cpo(state: &mut data_types::State8080) {
    if !state.cc.p {
        call(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn ani(state: &mut data_types::State8080) {
    let imm = state.memory[(state.pc + 1) as usize];
    state.a &= imm;
    state.cc.z = state.a == 0;
    state.cc.s = (state.a & 0x80) != 0;
    state.cc.p = parity(state.a);
    state.cc.cy = false;
    state.pc += 2;
}

#[inline(always)]
fn rpe(state: &mut data_types::State8080) {
    if state.cc.p {
        let return_address = pop_stack(state);
        state.pc = return_address;
    } else {
        state.pc += 1;
    }
}

#[inline(always)]
fn pchl(state: &mut data_types::State8080) {
    state.pc = get_hl(state);
}

#[inline(always)]
fn jpe(state: &mut data_types::State8080) {
    if state.cc.p {
        state.pc = get_jmp_target_address(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn xchg(state: &mut data_types::State8080) {
    let temp_d = state.d;
    let temp_e = state.e;
    state.d = state.h;
    state.e = state.l;
    state.h = temp_d;
    state.l = temp_e;
    state.pc += 1;
}

#[inline(always)]
fn cpe(state: &mut data_types::State8080) {
    if state.cc.p {
        call(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn xri(state: &mut data_types::State8080) {
    let imm = state.memory[(state.pc + 1) as usize];
    state.a ^= imm;
    flags_zsp(state, state.a);
    state.cc.cy = false;
    state.pc += 2;
}

#[inline(always)]
fn rp(state: &mut data_types::State8080) {
    if !state.cc.s {
        let return_address = pop_stack(state);
        state.pc = return_address;
    } else {
        state.pc += 1;
    }
}

#[inline(always)]
fn pop_psw(state: &mut data_types::State8080) {
    let flags_byte = state.memory[state.sp as usize]; // Pop Flags
    let accumulator = state.memory[(state.sp + 1) as usize]; // Pop Accumulator
    state.set_flags_from_byte(flags_byte);
    state.a = accumulator;
    state.sp += 2;
    state.pc += 1;
}

#[inline(always)]
fn jp(state: &mut data_types::State8080) {
    if state.cc.s {
        state.pc = get_jmp_target_address(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn di(state: &mut data_types::State8080) {
    state.int_enable = false;
    state.pc += 1;
}

#[inline(always)]
fn cp(state: &mut data_types::State8080) {
    if state.cc.s {
        call(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn push_psw(state: &mut data_types::State8080) {
    // The flags are packed into one byte
    let flags_byte = state.get_flags_as_byte();
    state.memory[(state.sp - 1) as usize] = state.a; // Push Accumulator
    state.memory[(state.sp - 2) as usize] = flags_byte; // Push Flags
    state.sp -= 2;
    state.pc += 1;
}

#[inline(always)]
fn ori(state: &mut data_types::State8080) {
    let x = state.memory[(state.pc + 1) as usize] | state.a;
    flags_zsp(state, x);
    state.cc.cy = false;
    state.a = x;
    state.pc += 2;
}

#[inline(always)]
fn rm(state: &mut data_types::State8080) {
    if state.cc.s {
        state.pc += 1;
    } else {
        let return_address = pop_stack(state);
        state.pc = return_address;
    }
}

#[inline(always)]
fn sphl(state: &mut data_types::State8080) {
    state.sp = get_hl(state);
    state.pc += 1;
}

#[inline(always)]
fn jm(state: &mut data_types::State8080) {
    if state.cc.s {
        state.pc = get_jmp_target_address(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn ei(state: &mut data_types::State8080) {
    state.int_enable = true; // Enable interrupts
    state.pc += 1;
}

#[inline(always)]
fn cm(state: &mut data_types::State8080) {
    if state.cc.s {
        call(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn cpi(state: &mut data_types::State8080) {
    let imm = state.memory[(state.pc + 1) as usize];
    let result = state.a.wrapping_sub(imm);
    state.cc.z = result == 0;
    state.cc.s = (result & 0x80) != 0;
    state.cc.p = parity(result);
    state.cc.cy = state.a < imm; // Set carry if there's a borrow
    state.pc += 2;
}

#[inline(always)]
fn jpo(state: &mut data_types::State8080) {
    if !state.cc.p {
        state.pc = get_jmp_target_address(state);
    } else {
        state.pc += 3;
    }
}

#[inline(always)]
fn set_flag_add(state: &mut data_types::State8080, res: u16) {
    state.cc.z = (res & 0xff) == 0;
    state.cc.s = (res & 0x80) != 0;
    state.cc.cy = res > 0xff;
    state.cc.p = parity((res & 0xff) as u8);
}

#[inline(always)]
fn parity(value: u8) -> bool {
    value.count_ones() % 2 == 0
}

#[inline(always)]
fn get_memory_address(state: &mut data_types::State8080) -> usize {
    (((state.h as u16) << 8) | (state.l as u16)) as usize
}

#[inline(always)]
fn set_flags_dcr(state: &mut data_types::State8080, value_before: u8, result: u8) {
    // Zero Flag
    state.cc.z = result == 0;

    // Sign Flag
    state.cc.s = (result & 0x80) != 0;

    // Parity Flag
    state.cc.p = parity(result);

    // Auxiliary Carry Flag
    state.cc.ac = (value_before & 0x0F) == 0x00;

    // Carry Flag is not affected
}

#[inline(always)]
fn get_register_pair_value(
    state: &data_types::State8080,
    register_pair: &data_types::RegisterPair,
) -> u16 {
    match register_pair {
        data_types::RegisterPair::BC => get_bc(state),
        data_types::RegisterPair::DE => get_de(state),
        data_types::RegisterPair::HL => get_hl(state),
        data_types::RegisterPair::SP => get_sp(state),
        _ => panic!("Invalid register pair"),
    }
}

#[inline(always)]
fn set_register_pair_value(
    state: &mut data_types::State8080,
    register_pair: &data_types::RegisterPair,
    value: u16,
) {
    match register_pair {
        data_types::RegisterPair::BC => set_bc(state, value),
        data_types::RegisterPair::DE => set_de(state, value),
        data_types::RegisterPair::HL => set_hl(state, value),
        data_types::RegisterPair::SP => set_sp(state, value),
        _ => panic!("Invalid register pair"),
    }
}

#[inline(always)]
fn get_bc(state: &data_types::State8080) -> u16 {
    ((state.b as u16) << 8) | (state.c as u16)
}

#[inline(always)]
fn get_de(state: &data_types::State8080) -> u16 {
    ((state.d as u16) << 8) | (state.e as u16)
}

#[inline(always)]
fn get_hl(state: &data_types::State8080) -> u16 {
    ((state.h as u16) << 8) | (state.l as u16)
}

#[inline(always)]
fn get_sp(state: &data_types::State8080) -> u16 {
    state.sp
}

#[inline(always)]
fn set_bc(state: &mut data_types::State8080, value: u16) {
    state.b = (value >> 8) as u8;
    state.c = value as u8;
}

#[inline(always)]
fn set_de(state: &mut data_types::State8080, value: u16) {
    state.d = (value >> 8) as u8;
    state.e = value as u8;
}

#[inline(always)]
fn set_hl(state: &mut data_types::State8080, value: u16) {
    state.h = (value >> 8) as u8;
    state.l = value as u8;
}

#[inline(always)]
fn set_sp(state: &mut data_types::State8080, value: u16) {
    state.sp = value;
}

#[inline(always)]
fn get_jmp_target_address(state: &data_types::State8080) -> u16 {
    (state.memory[(state.pc + 2) as usize] as u16) << 8
        | state.memory[(state.pc + 1) as usize] as u16
}

#[inline(always)]
fn handle_in(state: &mut data_types::State8080, port: u8) -> u8 {
    match port {
        0 => 0xf,
        1 => state.in_port1,
        2 => 0,
        3 => {
            // Return result from shift register (shift1 << 8 | shift0) >> shift_offset
            let shift_val = (state.shift1 as u16) << 8 | state.shift0 as u16;
            (shift_val >> (8 - state.shift_offset)) as u8
        }
        _ => {
            // If the port is not implemented, return 0
            0
        }
    }
}

#[inline(always)]
fn handle_out(state: &mut data_types::State8080, port: u8, value: u8) {
    match port {
        2 => {
            // Set the shift register's offset
            state.shift_offset = value & 0x7;
        }
        4 => {
            // Load shift register: move shift1 to shift0, and load value into shift1
            state.shift0 = state.shift1;
            state.shift1 = value;
        }
        _ => {
            // Handle other ports (e.g., sound-related ports, debug ports)
        }
    }
}

#[inline(always)]
pub fn generate_interrupt(state: &mut data_types::State8080, interrupt_num: u16) {
    // Only generate the interrupt if interrupts are enabled
    if state.int_enable {
        // Push the current PC onto the stack
        state.memory[state.sp.wrapping_sub(1) as usize] = ((state.pc & 0xff00) >> 8) as u8; // High byte
        state.memory[state.sp.wrapping_sub(2) as usize] = (state.pc & 0xff) as u8; // Low byte

        state.sp = state.sp.wrapping_sub(2);

        // Set the PC to the interrupt vector (interrupt_num * 8)
        state.pc = 8 * interrupt_num;

        // state.pc += 1;

        // Disable further interrupts until an EI instruction is executed
        state.int_enable = false;
    }
}

fn set_flags_inr(state: &mut data_types::State8080, value_before: u8, result: u8) {
    // Zero Flag
    state.cc.z = result == 0;

    // Sign Flag
    state.cc.s = (result & 0x80) != 0;

    // Parity Flag
    state.cc.p = parity(result);

    // Auxiliary Carry Flag
    state.cc.ac = (value_before & 0x0F) == 0x0F;

    // Carry Flag is not affected
}

fn pop_stack(state: &mut data_types::State8080) -> u16 {
    let low_byte = state.memory[state.sp as usize];
    let high_byte = state.memory[(state.sp + 1) as usize];
    state.sp += 2;
    ((high_byte as u16) << 8) | (low_byte as u16)
}

fn flags_zsp(state: &mut data_types::State8080, value: u8) {
    state.cc.z = value == 0;
    state.cc.s = (value & 0x80) != 0;
    state.cc.p = parity(value);
}

fn set_flags_daa(state: &mut data_types::State8080, result: u16) {
    state.cc.z = (result & 0xff) == 0;
    state.cc.s = (result & 0x80) != 0;
    state.cc.p = parity(result as u8);
    state.cc.cy = result > 0xff;
}

fn set_flags_adc(state: &mut data_types::State8080, result: u16) {
    state.cc.z = (result & 0xff) == 0;
    state.cc.s = (result & 0x80) != 0;
    state.cc.p = parity(result as u8);
    state.cc.cy = result > 0xff;
}
