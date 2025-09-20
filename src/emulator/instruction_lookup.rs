use super::data_types::State8080;

// Simple instruction mnemonic lookup
pub fn get_instruction_mnemonic(opcode: u8, state: &State8080) -> String {
    match opcode {
        0x00 => "NOP".to_string(),
        0x01 => format!("LXI B,${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0x02 => "STAX B".to_string(),
        0x03 => "INX B".to_string(),
        0x04 => "INR B".to_string(),
        0x05 => "DCR B".to_string(),
        0x06 => format!("MVI B,${:02X}", state.memory[(state.pc + 1) as usize]),
        0x07 => "RLC".to_string(),
        0x09 => "DAD B".to_string(),
        0x0A => "LDAX B".to_string(),
        0x0B => "DCX B".to_string(),
        0x0C => "INR C".to_string(),
        0x0D => "DCR C".to_string(),
        0x0E => format!("MVI C,${:02X}", state.memory[(state.pc + 1) as usize]),
        0x0F => "RRC".to_string(),

        0x11 => format!("LXI D,${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0x12 => "STAX D".to_string(),
        0x13 => "INX D".to_string(),
        0x14 => "INR D".to_string(),
        0x15 => "DCR D".to_string(),
        0x16 => format!("MVI D,${:02X}", state.memory[(state.pc + 1) as usize]),
        0x17 => "RAL".to_string(),
        0x19 => "DAD D".to_string(),
        0x1A => "LDAX D".to_string(),
        0x1B => "DCX D".to_string(),
        0x1C => "INR E".to_string(),
        0x1D => "DCR E".to_string(),
        0x1E => format!("MVI E,${:02X}", state.memory[(state.pc + 1) as usize]),
        0x1F => "RAR".to_string(),

        0x21 => format!("LXI H,${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0x22 => format!("SHLD ${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0x23 => "INX H".to_string(),
        0x24 => "INR H".to_string(),
        0x25 => "DCR H".to_string(),
        0x26 => format!("MVI H,${:02X}", state.memory[(state.pc + 1) as usize]),
        0x27 => "DAA".to_string(),
        0x29 => "DAD H".to_string(),
        0x2A => format!("LHLD ${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0x2B => "DCX H".to_string(),
        0x2C => "INR L".to_string(),
        0x2D => "DCR L".to_string(),
        0x2E => format!("MVI L,${:02X}", state.memory[(state.pc + 1) as usize]),
        0x2F => "CMA".to_string(),

        0x31 => format!("LXI SP,${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0x32 => format!("STA ${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0x33 => "INX SP".to_string(),
        0x34 => "INR M".to_string(),
        0x35 => "DCR M".to_string(),
        0x36 => format!("MVI M,${:02X}", state.memory[(state.pc + 1) as usize]),
        0x37 => "STC".to_string(),
        0x39 => "DAD SP".to_string(),
        0x3A => format!("LDA ${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0x3B => "DCX SP".to_string(),
        0x3C => "INR A".to_string(),
        0x3D => "DCR A".to_string(),
        0x3E => format!("MVI A,${:02X}", state.memory[(state.pc + 1) as usize]),
        0x3F => "CMC".to_string(),

        // MOV instructions (0x40-0x7F)
        0x40..=0x7F if opcode != 0x76 => {
            let dst_reg = ["B", "C", "D", "E", "H", "L", "M", "A"][((opcode >> 3) & 7) as usize];
            let src_reg = ["B", "C", "D", "E", "H", "L", "M", "A"][(opcode & 7) as usize];
            format!("MOV {},{}", dst_reg, src_reg)
        },
        0x76 => "HLT".to_string(),

        // ADD instructions (0x80-0x87)
        0x80..=0x87 => {
            let reg = ["B", "C", "D", "E", "H", "L", "M", "A"][(opcode & 7) as usize];
            format!("ADD {}", reg)
        },

        // Common jump and call instructions
        0xC2 => format!("JNZ ${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0xC3 => format!("JMP ${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0xC6 => format!("ADI ${:02X}", state.memory[(state.pc + 1) as usize]),
        0xC9 => "RET".to_string(),
        0xCA => format!("JZ ${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0xCD => format!("CALL ${:04X}", u16::from_le_bytes([state.memory[(state.pc + 1) as usize], state.memory[(state.pc + 2) as usize]])),
        0xD3 => format!("OUT ${:02X}", state.memory[(state.pc + 1) as usize]),
        0xDB => format!("IN ${:02X}", state.memory[(state.pc + 1) as usize]),
        0xE6 => format!("ANI ${:02X}", state.memory[(state.pc + 1) as usize]),
        0xF3 => "DI".to_string(),
        0xFB => "EI".to_string(),
        0xFE => format!("CPI ${:02X}", state.memory[(state.pc + 1) as usize]),

        // Default case for unimplemented instructions
        _ => format!("DB ${:02X}", opcode),
    }
}