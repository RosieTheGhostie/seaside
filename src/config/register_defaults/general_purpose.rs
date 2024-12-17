use super::{register_set::make_registers_format, RegisterSet};
use num_traits::{FromPrimitive, ToPrimitive};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

pub enum GeneralPurposeRegister {
    Zero,
    AssemblerTemporary,
    Value(u8),
    Argument(u8),
    Temporary(u8),
    Saved(u8),
    Kernel(u8),
    GlobalPointer,
    StackPointer,
    FramePointer,
    ReturnAddress,
}

impl FromStr for GeneralPurposeRegister {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "zero" {
            return Ok(Self::Zero);
        }
        if s.len() != 2 {
            return Err(Error::from(ErrorKind::InvalidInput));
        }
        match s {
            "at" => return Ok(Self::AssemblerTemporary),
            "gp" => return Ok(Self::GlobalPointer),
            "sp" => return Ok(Self::StackPointer),
            "fp" => return Ok(Self::FramePointer),
            "ra" => return Ok(Self::ReturnAddress),
            _ => {}
        }
        let (prefix, n) = {
            let mut chars = s.chars();
            let prefix = chars.nth(0).unwrap(); // guaranteed to exist due to earlier length check
            if let Some(n) = chars.nth(1).and_then(|n| n.to_digit(10)) {
                (prefix, n as u8)
            } else {
                return Err(Error::from(ErrorKind::InvalidInput));
            }
        };
        match (prefix, n) {
            ('v', 0..=1) => Ok(Self::Value(n)),
            ('a', 0..=3) => Ok(Self::Argument(n)),
            ('t', _) => Ok(Self::Temporary(n)),
            ('s', 0..=7) => Ok(Self::Saved(n)),
            ('k', 0..=1) => Ok(Self::Kernel(n)),
            _ => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }
}

impl FromPrimitive for GeneralPurposeRegister {
    fn from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Self::Zero),
            1 => Some(Self::AssemblerTemporary),
            2..=3 => Some(Self::Value(n - 2)),
            4..=7 => Some(Self::Argument(n - 4)),
            8..=15 => Some(Self::Temporary(n - 8)),
            16..=23 => Some(Self::Saved(n - 16)),
            24..=25 => Some(Self::Temporary(n - 24)),
            26..=27 => Some(Self::Kernel(n - 26)),
            28 => Some(Self::GlobalPointer),
            29 => Some(Self::StackPointer),
            30 => Some(Self::FramePointer),
            31 => Some(Self::ReturnAddress),
            32.. => None,
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        if let Ok(n) = n.try_into() {
            Self::from_u8(n)
        } else {
            None
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        if let Ok(n) = n.try_into() {
            Self::from_u8(n)
        } else {
            None
        }
    }
}

impl ToPrimitive for GeneralPurposeRegister {
    fn to_u8(&self) -> Option<u8> {
        match self {
            Self::Zero => Some(0),
            Self::AssemblerTemporary => Some(1),
            Self::Value(n) => Some(n + 2),
            Self::Argument(n) => Some(n + 4),
            Self::Temporary(n) => Some(n + if *n <= 7 { 8 } else { 24 }),
            Self::Saved(n) => Some(n + 16),
            Self::Kernel(n) => Some(n + 26),
            Self::GlobalPointer => Some(28),
            Self::StackPointer => Some(29),
            Self::FramePointer => Some(30),
            Self::ReturnAddress => Some(31),
        }
    }

    fn to_i64(&self) -> Option<i64> {
        self.to_u8().map(|n| n as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_u8().map(|n| n as u64)
    }
}

impl RegisterSet for GeneralPurposeRegister {
    const NUM_REGISTERS: usize = 32;
    const REGISTER_NAMES: &'static [&'static str] = &[
        "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
        "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp",
        "fp", "ra",
    ];
}

make_registers_format!(GeneralPurposeRegister);
