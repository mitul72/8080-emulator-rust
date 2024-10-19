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

    pub shift0: u8,
    pub shift1: u8,
    pub shift_offset: u8,

    pub in_port1: u8,

    pub memory: [u8; 0x10000], // 64KB memory
    pub cc: ConditionCodes,
    pub int_enable: bool,
    pub cycles: i32,
}

impl State8080 {
    pub fn get_flags_as_byte(&self) -> u8 {
        let mut flags = 0;
        if self.cc.z {
            flags |= 0x40;
        } // Zero flag
        if self.cc.s {
            flags |= 0x80;
        } // Sign flag
        if self.cc.p {
            flags |= 0x04;
        } // Parity flag
        if self.cc.cy {
            flags |= 0x01;
        } // Carry flag
        if self.cc.ac {
            flags |= 0x10;
        } // Auxiliary Carry flag
        flags
    }

    pub fn set_flags_from_byte(&mut self, flags: u8) {
        self.cc.z = (flags & 0x40) != 0;
        self.cc.s = (flags & 0x80) != 0;
        self.cc.p = (flags & 0x04) != 0;
        self.cc.cy = (flags & 0x01) != 0;
        self.cc.ac = (flags & 0x10) != 0;
    }
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

            shift0: 0,
            shift1: 0,
            shift_offset: 0,
            in_port1: 0,

            memory: [0; 0x10000],
            cc: ConditionCodes::default(),
            int_enable: false,
            cycles: 0,
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

pub enum Keys {
    Start,
    Up,
    Down,
    Left,
    Right,
}

pub const CYCLE_TABLE: [u8; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5,
    7, 4, 4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 13, 5, 10, 10, 10, 4, 4, 10,
    13, 5, 5, 5, 7, 4, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5,
    5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 7, 7, 7, 7, 7, 7, 7, 7, 5,
    5, 5, 5, 5, 5, 7, 5, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4,
    4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4,
    4, 4, 4, 4, 4, 7, 4, 5, 10, 10, 10, 11, 11, 7, 11, 5, 10, 10, 10, 11, 17, 7, 11, 5, 10, 10, 10,
    11, 11, 7, 11, 5, 10, 10, 10, 11, 17, 7, 11, 5, 10, 10, 18, 11, 11, 7, 11, 5, 5, 10, 4, 11, 17,
    7, 11, 5, 10, 10, 4, 11, 11, 7, 11, 5, 5, 10, 4, 11, 17, 7, 11,
];
