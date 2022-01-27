use std::fmt::{Debug, Display, Formatter, Result};

pub type Label<'a> = std::borrow::Cow<'a, str>;

macro_rules! display_from_debug {
    ($implementor: ty) => {
        impl Debug for $implementor {
            fn fmt(&self, f: &mut Formatter) -> Result {
                Display::fmt(self, f)
            }
        }
    };
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Register {
    Zero,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    GP,
    SP,
    FP,
    RA,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str(match self {
            Self::Zero => "$zero",
            Self::V0 => "$v0",
            Self::V1 => "$v1",
            Self::A0 => "$a0",
            Self::A1 => "$a1",
            Self::A2 => "$a2",
            Self::A3 => "$a3",
            Self::T0 => "$t0",
            Self::T1 => "$t1",
            Self::T2 => "$t2",
            Self::T3 => "$t3",
            Self::T4 => "$t4",
            Self::T5 => "$t5",
            Self::T6 => "$t6",
            Self::T7 => "$t7",
            Self::S0 => "$s0",
            Self::S1 => "$s1",
            Self::S2 => "$s2",
            Self::S3 => "$s3",
            Self::S4 => "$s4",
            Self::S5 => "$s5",
            Self::S6 => "$s6",
            Self::S7 => "$s7",
            Self::T8 => "$t8",
            Self::T9 => "$t9",
            Self::GP => "$gp",
            Self::SP => "$sp",
            Self::FP => "$fp",
            Self::RA => "$ra",
        })
    }
}

display_from_debug!(Register);

#[repr(u8)]
pub enum MarsServiceNumber {
    PrintInteger = 1,
    PrintString = 4,
    Exit = 10,
}

/// Represents an instruction in the MIPS32 architecture.
pub enum Instruction<'a> {
    La(Register, Label<'a>),
    Li(Register, i32),
    /// Sets the contents of the first register to the second.
    Move(Register, Register),
    /// Issues a system call using the value in the `$v0` register.
    Syscall,
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::La(destination, label) => write!(f, "la {},{}", destination, label),
            Self::Li(destination, value) => write!(f, "li {},{:#X}", destination, value),
            Self::Move(destination, source) => write!(f, "move {},{}", destination, source),
            Self::Syscall => f.write_str("syscall"),
        }
    }
}

display_from_debug!(Instruction<'_>);
