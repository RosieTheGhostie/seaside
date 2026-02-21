use core::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use num_traits::FromPrimitive;
use strum::EnumIter;

use super::{IndexedRegister, ParseError};

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
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

impl FpuRegister {
    pub const MAX: Self = Self::F31;
    pub const MAX_RAW: u8 = Self::MAX as _;
    pub const N_REGISTERS: usize = Self::MAX_RAW as usize + 1;
    pub const NAMES: [&str; Self::N_REGISTERS] = [
        "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12", "f13",
        "f14", "f15", "f16", "f17", "f18", "f19", "f20", "f21", "f22", "f23", "f24", "f25", "f26",
        "f27", "f28", "f29", "f30", "f31",
    ];

    /// Attempts to construct an [`FpuRegister`] from its raw index value.
    ///
    /// This will return [`None`] if `index` is greater than [`FpuRegister::MAX_RAW`].
    ///
    /// # See Also
    ///
    /// - [`from_raw_unchecked`](Self::from_raw_unchecked)
    pub const fn from_raw(index: u8) -> Option<Self> {
        if index <= Self::MAX_RAW {
            Some(unsafe { Self::from_raw_unchecked(index) })
        } else {
            None
        }
    }

    /// Constructs an [`FpuRegister`] from its raw index value.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring `index` is less than or equal to
    /// [`FpuRegister::MAX_RAW`]. Failure to do so may result in undefined behavior.
    ///
    /// # See Also
    ///
    /// - [`from_raw`](Self::from_raw)
    pub const unsafe fn from_raw_unchecked(index: u8) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(index) }
    }

    pub fn parse_indexed(s: &str) -> Result<Self, ParseError> {
        if let Ok(indexed) = IndexedRegister::from_str(s) {
            Ok(indexed.to_fpu())
        } else {
            s.parse()
        }
    }

    pub const fn into_index(self) -> usize {
        self as usize
    }

    pub const fn is_double_aligned(&self) -> bool {
        (*self as u8).is_multiple_of(2)
    }
}

impl Display for FpuRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
            Ok(unsafe { core::mem::transmute::<u8, Self>(index) })
        } else {
            Err(ParseError::BadValue)
        }
    }
}

impl FromPrimitive for FpuRegister {
    fn from_u8(n: u8) -> Option<Self> {
        Self::from_raw(n)
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
