#[derive(Default)]
pub struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
}

#[derive(Default)]
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
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: u8,
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

impl Register {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Register::B,
            1 => Register::C,
            2 => Register::D,
            3 => Register::E,
            4 => Register::H,
            5 => Register::L,
            6 => Register::M,
            7 => Register::A,
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

pub enum Flags {
    Z,
    S,
    P,
    CY,
    AC,
}
