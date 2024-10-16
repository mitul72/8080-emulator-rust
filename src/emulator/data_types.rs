use core::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Default)]
pub struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
}

pub struct State8080 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub memory: [u8; 0x10000], // 64KB memory
    pub cc: ConditionCodes,
    pub int_enable: u8,
}

impl Default for State8080 {
    fn default() -> Self {
        State8080 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: [0; 0x10000],
            cc: ConditionCodes::default(),
            int_enable: 0,
        }
    }
}

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
}

impl Debug for Register {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
            Register::C => write!(f, "C"),
            Register::D => write!(f, "D"),
            Register::E => write!(f, "E"),
            Register::H => write!(f, "H"),
            Register::L => write!(f, "L"),
            Register::M => write!(f, "M"),
        }
    }
}

impl Register {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x00 => Register::B,
            0x01 => Register::C,
            0x02 => Register::D,
            0x03 => Register::E,
            0x04 => Register::H,
            0x05 => Register::L,
            0x06 => Register::M,
            0x07 => Register::A,
            _ => panic!("Invalid register number"),
        }
    }
}

pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
    PSW,
}

impl Debug for RegisterPair {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RegisterPair::BC => write!(f, "BC"),
            RegisterPair::DE => write!(f, "DE"),
            RegisterPair::HL => write!(f, "HL"),
            RegisterPair::SP => write!(f, "SP"),
            RegisterPair::PSW => write!(f, "PSW"),
        }
    }
}

pub enum Flags {
    Z,
    S,
    P,
    CY,
    AC,
}
