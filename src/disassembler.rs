pub struct Disassembler;

impl Disassembler {
    pub fn disassemble_8080_op(buffer: &[u8], pc: usize) -> usize {
        let code = &buffer[pc..];
        let mut opbytes = 1; // Default instruction size is 1 byte

        print!("{:04X} ", pc); // Print the address in hex

        match code[0] {
            0x00 => println!("NOP"),
            0x01 => {
                println!("LXI    B,#${:02x}{:02x}", code[2], code[1]);
                opbytes = 3;
            }
            0x02 => println!("STAX   B"),
            0x03 => println!("INX    B"),
            0x04 => println!("INR    B"),
            0x05 => println!("DCR    B"),
            0x06 => {
                println!("MVI    B,#${:02x}", code[1]);
                opbytes = 2;
            }
            0x07 => println!("RLC"),
            0x08 => println!("NOP"),
            0x09 => println!("DAD    B"),
            0x0A => println!("LDAX   B"),
            0x0B => println!("DCX    B"),
            0x0C => println!("INR    C"),
            0x0D => println!("DCR    C"),
            0x0E => {
                println!("MVI    C,#${:02x}", code[1]);
                opbytes = 2;
            }
            0x0F => println!("RRC"),
            0x10 => println!("NOP"),
            0x11 => {
                println!("LXI    D,#${:02x}{:02x}", code[2], code[1]);
                opbytes = 3;
            }
            0x12 => println!("STAX   D"),
            0x13 => println!("INX    D"),
            0x14 => println!("INR    D"),
            0x15 => println!("DCR    D"),
            0x16 => {
                println!("MVI    D,#${:02x}", code[1]);
                opbytes = 2;
            }
            0x17 => println!("RAL"),
            0x18 => println!("NOP"),
            0x19 => println!("DAD    D"),
            0x1A => println!("LDAX   D"),
            0x1B => println!("DCX    D"),
            0x1C => println!("INR    E"),
            0x1D => println!("DCR    E"),
            0x1E => {
                println!("MVI    E,#${:02x}", code[1]);
                opbytes = 2;
            }
            0x1F => println!("RAR"),
            0x20 => println!("NOP"),
            0x21 => {
                println!("LXI    H,#${:02x}{:02x}", code[2], code[1]);
                opbytes = 3;
            }
            0x22 => {
                println!("SHLD   ${:02x}{:02x}", code[2], code[1]);
                opbytes = 3;
            }
            0x23 => println!("INX    H"),
            0x24 => println!("INR    H"),
            0x25 => println!("DCR    H"),
            0x26 => {
                println!("MVI    H,#${:02x}", code[1]);
                opbytes = 2;
            }
            0x27 => println!("DAA"),
            0x28 => println!("NOP"),
            0x29 => println!("DAD    H"),
            0x2A => {
                println!("LHLD   ${:02x}{:02x}", code[2], code[1]);
                opbytes = 3;
            }
            0x2B => println!("DCX    H"),
            0x2C => println!("INR    L"),
            0x2D => println!("DCR    L"),
            0x2E => {
                println!("MVI    L,#${:02x}", code[1]);
                opbytes = 2;
            }
            0x2F => println!("CMA"),
            0x30 => println!("NOP"),
            0x31 => {
                println!("LXI    SP,#${:02x}{:02x}", code[2], code[1]);
                opbytes = 3;
            }
            0x32 => {
                println!("STA    ${:02x}{:02x}", code[2], code[1]);
                opbytes = 3;
            }
            0x33 => println!("INX    SP"),
            0x34 => println!("INR    M"),
            0x35 => println!("DCR    M"),
            0x36 => {
                println!("MVI    M,#${:02x}", code[1]);
                opbytes = 2;
            }
            0x37 => println!("STC"),
            0x38 => println!("NOP"),
            0x39 => println!("DAD    SP"),
            0x3A => {
                println!("LDA    ${:02x}{:02x}", code[2], code[1]);
                opbytes = 3;
            }
            0x3B => println!("DCX    SP"),
            0x3C => println!("INR    A"),
            0x3D => println!("DCR    A"),
            0x3E => {
                println!("MVI    A,#${:02x}", code[1]);
                opbytes = 2;
            }
            0x3F => println!("CMC"),
            // MOV instructions
            0x40..=0x7F => {
                let dest = match (code[0] >> 3) & 0x07 {
                    0x00 => "B",
                    0x01 => "C",
                    0x02 => "D",
                    0x03 => "E",
                    0x04 => "H",
                    0x05 => "L",
                    0x06 => "M",
                    0x07 => "A",
                    _ => unreachable!(),
                };
                let src = match code[0] & 0x07 {
                    0x00 => "B",
                    0x01 => "C",
                    0x02 => "D",
                    0x03 => "E",
                    0x04 => "H",
                    0x05 => "L",
                    0x06 => "M",
                    0x07 => "A",
                    _ => unreachable!(),
                };
                if code[0] == 0x76 {
                    println!("HLT");
                } else {
                    println!("MOV    {},{}", dest, src);
                }
            }
            // Arithmetic and logic instructions
            0x80..=0xBF => {
                let mnemonic = match code[0] & 0xF8 {
                    0x80 => "ADD",
                    0x88 => "ADC",
                    0x90 => "SUB",
                    0x98 => "SBB",
                    0xA0 => "ANA",
                    0xA8 => "XRA",
                    0xB0 => "ORA",
                    0xB8 => "CMP",
                    _ => "UNKNOWN",
                };
                let src = match code[0] & 0x07 {
                    0x00 => "B",
                    0x01 => "C",
                    0x02 => "D",
                    0x03 => "E",
                    0x04 => "H",
                    0x05 => "L",
                    0x06 => "M",
                    0x07 => "A",
                    _ => "UNKNOWN",
                };
                println!("{}    {}", mnemonic, src);
            }
            // Immediate instructions
            0xC6 | 0xCE | 0xD3 | 0xD6 | 0xDB | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => {
                let mnemonic = match code[0] {
                    0xC6 => "ADI",
                    0xCE => "ACI",
                    0xD3 => "OUT",
                    0xD6 => "SUI",
                    0xDB => "IN",
                    0xDE => "SBI",
                    0xE6 => "ANI",
                    0xEE => "XRI",
                    0xF6 => "ORI",
                    0xFE => "CPI",
                    _ => "UNKNOWN",
                };
                println!("{}    #${:02x}", mnemonic, code[1]);
                opbytes = 2;
            }
            // Jump and call instructions
            0xC2 | 0xC3 | 0xC4 | 0xCA | 0xCC | 0xCD | 0xD2 | 0xDA | 0xE2 | 0xEA | 0xF2 | 0xFA => {
                let mnemonic = match code[0] {
                    0xC2 => "JNZ",
                    0xC3 => "JMP",
                    0xC4 => "CNZ",
                    0xCA => "JZ",
                    0xCC => "CZ",
                    0xCD => "CALL",
                    0xD2 => "JNC",
                    0xDA => "JC",
                    0xE2 => "JPO",
                    0xEA => "JPE",
                    0xF2 => "JP",
                    0xFA => "JM",
                    _ => "UNKNOWN",
                };
                println!("{}    ${:02x}{:02x}", mnemonic, code[2], code[1]);
                opbytes = 3;
            }
            // Return instructions
            0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xE0 | 0xE8 | 0xF0 | 0xF8 => {
                let mnemonic = match code[0] {
                    0xC0 => "RNZ",
                    0xC8 => "RZ",
                    0xD0 => "RNC",
                    0xD8 => "RC",
                    0xE0 => "RPO",
                    0xE8 => "RPE",
                    0xF0 => "RP",
                    0xF8 => "RM",
                    _ => "UNKNOWN",
                };
                println!("{}", mnemonic);
            }
            // Single-byte instructions
            0xC9 => println!("RET"),
            0xD9 => println!("RET"),
            0xE9 => println!("PCHL"),
            0xF9 => println!("SPHL"),
            0xFB => println!("EI"),
            0xF3 => println!("DI"),
            0xEB => println!("XCHG"),
            0xE3 => println!("XTHL"),
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                let rst_num = (code[0] - 0xC7) / 8;
                println!("RST    {}", rst_num);
            }
            // PUSH and POP instructions
            0xC1 | 0xC5 | 0xD1 | 0xD5 | 0xE1 | 0xE5 | 0xF1 | 0xF5 => {
                let rp = match (code[0] >> 4) & 0x03 {
                    0x00 => "B",
                    0x01 => "D",
                    0x02 => "H",
                    0x03 => "PSW",
                    _ => "UNKNOWN",
                };
                let mnemonic = if code[0] & 0x04 == 0 { "POP" } else { "PUSH" };
                println!("{}    {}", mnemonic, rp);
            }
            // Default case
            _ => println!("Unknown opcode"),
        }

        opbytes // Return the number of bytes used by the instruction
    }

    pub fn disassemble(buffer: &[u8]) {
        let mut pc = 0;

        // Disassemble the entire buffer
        while pc < buffer.len() {
            pc += Self::disassemble_8080_op(buffer, pc);
        }
    }
}
