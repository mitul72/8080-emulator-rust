use super::data_types;

pub fn unimplemented_instruction() {
    println!("Error: Unimplemented instruction\n");
    std::process::exit(1);
}

// TODO: Implement the following functions
pub fn emulate_8080_op(state: &mut data_types::State8080) {
    let op_codes = &state.memory[state.pc as usize..];
    match op_codes[0] {
        0x00 => nop(),
        0x01 => lxi(state, data_types::RegisterPair::BC),
        0x02 => stax(state, data_types::RegisterPair::BC),
        0x03 => inx(state, data_types::RegisterPair::BC),
        0x04 => inr(state, data_types::Register::B),
        0x05 => dcr(state, data_types::Register::B),
        0x06 => mvi(state, data_types::Register::B),
        0x07 => rlc(state),
        0x08 => unimplemented_instruction(),
        0x09 => dad(state, data_types::RegisterPair::BC),
        0x0A => ldax(state, data_types::RegisterPair::BC),
        0x0B => dcx(state, data_types::RegisterPair::BC),
        0x0C => inr(state, data_types::Register::C),
        0x0D => dcr(state, data_types::Register::C),
        0x0E => mvi(state, data_types::Register::C),
        0x0F => rrc(state),
        0x10 => unimplemented_instruction(),
        0x11 => lxi(state, data_types::RegisterPair::DE),
        0x12 => stax(state, data_types::RegisterPair::DE),
        0x13 => inx(state, data_types::RegisterPair::DE),
        0x14 => inr(state, data_types::Register::D),
        0x15 => dcr(state, data_types::Register::D),
        0x16 => mvi(state, data_types::Register::D),
        0x17 => ral(state),
        0x18 => unimplemented_instruction(),
        0x19 => dad(state, data_types::RegisterPair::DE),
        0x1A => ldax(state, data_types::RegisterPair::DE),
        0x1B => dcx(state, data_types::RegisterPair::DE),
        0x1C => inr(state, data_types::Register::E),
        0x1D => dcr(state, data_types::Register::E),
        0x1E => mvi(state, data_types::Register::E),
        0x1F => rar(state),
        0x20 => unimplemented_instruction(),
        0x21 => lxi(state, data_types::RegisterPair::HL),
        0x22 => shld(state),
        0x23 => inx(state, data_types::RegisterPair::HL),
        0x24 => inr(state, data_types::Register::H),
        0x25 => dcr(state, data_types::Register::H),
        0x26 => mvi(state, data_types::Register::H),
        0x27 => daa(state),
        0x28 => unimplemented_instruction(),
        0x29 => dad(state, data_types::RegisterPair::HL),
        0x2A => lhld(state),
        0x2B => dcx(state, data_types::RegisterPair::HL),
        0x2C => inr(state, data_types::Register::L),
        0x2D => dcr(state, data_types::Register::L),
        0x2E => mvi(state, data_types::Register::L),
        0x2F => cma(state),
        0x30 => unimplemented_instruction(),
        0x31 => lxi(state, data_types::RegisterPair::SP),
        0x32 => sta(state),
        0x33 => inx(state, data_types::RegisterPair::SP),
        0x34 => inr_m(state),
        0x35 => dcr_m(state),
        0x36 => mvi_m(state),
        0x37 => stc(state),
        0x38 => unimplemented_instruction(),
        0x39 => dad(state, data_types::RegisterPair::SP),
        0x3A => lda(state),
        0x3B => dcx(state, data_types::RegisterPair::SP),
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
        0xCB => unimplemented_instruction(),
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
        0xD9 => unimplemented_instruction(),
        0xDA => jc(state),
        0xDB => inp(state),
        0xDC => cc(state),
        0xDD => unimplemented_instruction(),
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
        0xED => unimplemented_instruction(),
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
        0xFD => unimplemented_instruction(),
        0xFE => cpi(state),
        0xFF => rst(state, 7),
    }
    state.pc += 1;
}

fn nop() {
    // No operation
}

fn lxi(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    // Implement the LXI instruction
}

fn stax(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    // Implement the STAX instruction
}

fn inx(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    // Implement the INX instruction
}

fn inr(state: &mut data_types::State8080, register: data_types::Register) {
    // Implement the INR instruction
}

fn dcr(state: &mut data_types::State8080, register: data_types::Register) {
    // Implement the DCR instruction
}

fn mvi(state: &mut data_types::State8080, register: data_types::Register) {
    // Implement the MVI instruction
}

fn rlc(state: &mut data_types::State8080) {
    // Implement the RLC instruction
}

fn dad(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    // Implement the DAD instruction
}

fn ldax(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    // Implement the LDAX instruction
}

fn dcx(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    // Implement the DCX instruction
}

fn rrc(state: &mut data_types::State8080) {
    // Implement the RRC instruction
}

fn ral(state: &mut data_types::State8080) {
    // Implement the RAL instruction
}

fn rar(state: &mut data_types::State8080) {
    // Implement the RAR instruction
}

fn shld(state: &mut data_types::State8080) {
    // Implement the SHLD instruction
}

fn daa(state: &mut data_types::State8080) {
    // Implement the DAA instruction
}

fn lhld(state: &mut data_types::State8080) {
    // Implement the LHLD instruction
}

fn cma(state: &mut data_types::State8080) {
    // Implement the CMA instruction
}

fn sta(state: &mut data_types::State8080) {
    // Implement the STA instruction
}

fn inr_m(state: &mut data_types::State8080) {
    // Implement the INR M instruction
}

fn dcr_m(state: &mut data_types::State8080) {
    // Implement the DCR M instruction
}

fn mvi_m(state: &mut data_types::State8080) {
    // Implement the MVI M instruction
}

fn stc(state: &mut data_types::State8080) {
    // Implement the STC instruction
}

fn lda(state: &mut data_types::State8080) {
    // Implement the LDA instruction
}

fn cmc(state: &mut data_types::State8080) {
    // Implement the CMC instruction
}

fn hlt(state: &mut data_types::State8080) {
    // Implement the HLT instruction
}

fn mov(state: &mut data_types::State8080, dest: data_types::Register, src: data_types::Register) {
    // Implement the MOV instruction
}

fn add(state: &mut data_types::State8080, src: data_types::Register) {
    // Implement the ADD instruction
}

fn adc(state: &mut data_types::State8080, src: data_types::Register) {
    // Implement the ADC instruction
}

fn sub(state: &mut data_types::State8080, src: data_types::Register) {
    // Implement the SUB instruction
}

fn sbb(state: &mut data_types::State8080, src: data_types::Register) {
    // Implement the SBB instruction
}

fn ana(state: &mut data_types::State8080, src: data_types::Register) {
    // Implement the ANA instruction
}

fn xra(state: &mut data_types::State8080, src: data_types::Register) {
    // Implement the XRA instruction
}

fn ora(state: &mut data_types::State8080, src: data_types::Register) {
    // Implement the ORA instruction
}

fn cmp(state: &mut data_types::State8080, src: data_types::Register) {
    // Implement the CMP instruction
}

fn rnz(state: &mut data_types::State8080) {
    // Implement the RNZ instruction
}

fn pop(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    // Implement the POP instruction
}

fn jnz(state: &mut data_types::State8080) {
    // Implement the JNZ instruction
}

fn jmp(state: &mut data_types::State8080) {
    // Implement the JMP instruction
}

fn cnz(state: &mut data_types::State8080) {
    // Implement the CNZ instruction
}

fn push(state: &mut data_types::State8080, register_pair: data_types::RegisterPair) {
    // Implement the PUSH instruction
}

fn adi(state: &mut data_types::State8080) {
    // Implement the ADI instruction
}

fn rst(state: &mut data_types::State8080, num: u8) {
    // Implement the RST instruction
}

fn rz(state: &mut data_types::State8080) {
    // Implement the RZ instruction
}

fn ret(state: &mut data_types::State8080) {
    // Implement the RET instruction
}

fn jz(state: &mut data_types::State8080) {
    // Implement the JZ instruction
}

fn cz(state: &mut data_types::State8080) {
    // Implement the CZ instruction
}

fn call(state: &mut data_types::State8080) {
    // Implement the CALL instruction
}

fn aci(state: &mut data_types::State8080) {
    // Implement the ACI instruction
}

fn rnc(state: &mut data_types::State8080) {
    // Implement the RNC instruction
}

fn out(state: &mut data_types::State8080) {
    // Implement the OUT instruction
}

fn cnc(state: &mut data_types::State8080) {
    // Implement the CNC instruction
}

fn jnc(state: &mut data_types::State8080) {
    // Implement the JNC instruction
}

fn sui(state: &mut data_types::State8080) {
    // Implement the SUI instruction
}

fn rc(state: &mut data_types::State8080) {
    // Implement the RC instruction
}

fn jc(state: &mut data_types::State8080) {
    // Implement the JC instruction
}

fn inp(state: &mut data_types::State8080) {
    // Implement the IN instruction
}

fn cc(state: &mut data_types::State8080) {
    // Implement the CC instruction
}

fn sbi(state: &mut data_types::State8080) {
    // Implement the SBI instruction
}

fn rpo(state: &mut data_types::State8080) {
    // Implement the RPO instruction
}

fn xthl(state: &mut data_types::State8080) {
    // Implement the XTHL instruction
}

fn cpo(state: &mut data_types::State8080) {
    // Implement the CPO instruction
}

fn ani(state: &mut data_types::State8080) {
    // Implement the ANI instruction
}

fn rpe(state: &mut data_types::State8080) {
    // Implement the RPE instruction
}

fn pchl(state: &mut data_types::State8080) {
    // Implement the PCHL instruction
}

fn jpe(state: &mut data_types::State8080) {
    // Implement the JPE instruction
}

fn xchg(state: &mut data_types::State8080) {
    // Implement the XCHG instruction
}

fn cpe(state: &mut data_types::State8080) {
    // Implement the CPE instruction
}

fn xri(state: &mut data_types::State8080) {
    // Implement the XRI instruction
}

fn rp(state: &mut data_types::State8080) {
    // Implement the RP instruction
}

fn pop_psw(state: &mut data_types::State8080) {
    // Implement the POP PSW instruction
}

fn jp(state: &mut data_types::State8080) {
    // Implement the JP instruction
}

fn di(state: &mut data_types::State8080) {
    // Implement the DI instruction
}

fn cp(state: &mut data_types::State8080) {
    // Implement the CP instruction
}

fn push_psw(state: &mut data_types::State8080) {
    // Implement the PUSH PSW instruction
}

fn ori(state: &mut data_types::State8080) {
    // Implement the ORI instruction
}

fn rm(state: &mut data_types::State8080) {
    // Implement the RM instruction
}

fn sphl(state: &mut data_types::State8080) {
    // Implement the SPHL instruction
}

fn jm(state: &mut data_types::State8080) {
    // Implement the JM instruction
}

fn ei(state: &mut data_types::State8080) {
    // Implement the EI instruction
}

fn cm(state: &mut data_types::State8080) {
    // Implement the CM instruction
}

fn cpi(state: &mut data_types::State8080) {
    // Implement the CPI instruction
}

fn jpo(state: &mut data_types::State8080) {
    // Implement the JPO instruction
}
