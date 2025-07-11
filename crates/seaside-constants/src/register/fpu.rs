use super::{IndexedRegister, ParseError};
use core::{
    fmt::{Display, Formatter, Result as FmtResult, Write},
    str::FromStr,
};
use num_traits::FromPrimitive;
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Clone, Copy, Debug, EnumIter, Eq, Ord, PartialEq, PartialOrd)]
pub enum FpuRegister {
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}

impl Display for FpuRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            f.write_char('$')?;
        }
        write!(f, "f{}", *self as u8)
    }
}

impl FromStr for FpuRegister {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('$').unwrap_or(s);
        if s.is_empty() {
            return Err(ParseError::Empty);
        }
        let index = s.strip_prefix('f').ok_or(ParseError::BadValue)?;
        if index.starts_with('0') {
            return if index.len() == 1 {
                Ok(Self::F0)
            } else {
                Err(ParseError::BadValue)
            };
        }
        let index: u8 = index.parse().map_err(|_| ParseError::BadValue)?;
        if index < 32 {
            Ok(unsafe { core::mem::transmute::<_, Self>(index) })
        } else {
            Err(ParseError::BadValue)
        }
    }
}

impl FromPrimitive for FpuRegister {
    fn from_u8(n: u8) -> Option<Self> {
        (n < 32).then(|| unsafe { core::mem::transmute(n) })
    }

    fn from_u64(n: u64) -> Option<Self> {
        if let Ok(n) = n.try_into() {
            Self::from_u8(n)
        } else {
            None
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        if let Ok(n) = n.try_into() {
            Self::from_u8(n)
        } else {
            None
        }
    }
}

impl FpuRegister {
    pub const N_REGISTERS: usize = 32;
    pub const NAMES: [&str; Self::N_REGISTERS] = [
        "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12", "f13",
        "f14", "f15", "f16", "f17", "f18", "f19", "f20", "f21", "f22", "f23", "f24", "f25", "f26",
        "f27", "f28", "f29", "f30", "f31",
    ];

    pub const fn is_double_aligned(&self) -> bool {
        (*self as u8) % 2 == 0
    }

    pub fn parse_indexed(s: &str) -> Result<Self, ParseError> {
        if let Ok(indexed) = IndexedRegister::from_str(s) {
            Ok(indexed.to_fpu())
        } else {
            s.parse()
        }
    }
}
