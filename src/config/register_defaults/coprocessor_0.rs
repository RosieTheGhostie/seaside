use super::{register_set::make_registers_format, RegisterSet};
use num_traits::{FromPrimitive, ToPrimitive};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

pub enum Coprocessor0Register {
    // I know the numeric values of these registers are incorrect, but it's more convenient to do it
    // like this because I don't have to use a bunch of padding in the register array.
    VirtualAddress,
    Status,
    Cause,
    ErrorProgramCounter,
}

impl FromStr for Coprocessor0Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vaddr" => Ok(Self::VirtualAddress),
            "status" => Ok(Self::Status),
            "cause" => Ok(Self::Cause),
            "epc" => Ok(Self::ErrorProgramCounter),
            _ => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }
}

impl FromPrimitive for Coprocessor0Register {
    /// Converts from an `i64` to the corresponding `Coprocessor0Register` as specified by the MIPS
    /// spec. If you want to use an array index, use `from_isize` or `from_usize` instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use num_traits::FromPrimitive;
    /// # use crate::registers::Coprocessor0Register;
    /// #
    /// assert_eq!(
    ///     Coprocessor0Register::from_i64(8),
    ///     Some(Coprocessor0Register::VirtualAddress)
    /// );
    /// assert_eq!(Coprocessor0Register::from_i64(12), Some(Coprocessor0Register::Status));
    /// assert_eq!(Coprocessor0Register::from_i64(13), Some(Coprocessor0Register::Cause));
    /// assert_eq!(
    ///     Coprocessor0Register::from_i64(14),
    ///     Some(Coprocessor0Register::ErrorProgramCounter)
    /// );
    /// ```
    fn from_i64(n: i64) -> Option<Self> {
        if let Ok(n) = n.try_into() {
            Self::from_u64(n)
        } else {
            None
        }
    }

    /// Converts from a `u64` to the corresponding `Coprocessor0Register` as specified by the MIPS
    /// spec. If you want to use an array index, use `from_isize` or `from_usize` instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use num_traits::FromPrimitive;
    /// # use crate::registers::Coprocessor0Register;
    /// #
    /// assert_eq!(
    ///     Coprocessor0Register::from_u64(8),
    ///     Some(Coprocessor0Register::VirtualAddress)
    /// );
    /// assert_eq!(Coprocessor0Register::from_u64(12), Some(Coprocessor0Register::Status));
    /// assert_eq!(Coprocessor0Register::from_u64(13), Some(Coprocessor0Register::Cause));
    /// assert_eq!(
    ///     Coprocessor0Register::from_u64(14),
    ///     Some(Coprocessor0Register::ErrorProgramCounter)
    /// );
    /// ```
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            8 => Some(Self::VirtualAddress),
            12 => Some(Self::Status),
            13 => Some(Self::Cause),
            14 => Some(Self::ErrorProgramCounter),
            _ => None,
        }
    }

    /// Converts a signed index in the range `0..=3` to the `Coprocessor0Register` mapped to that
    /// index. Because there are only four registers in Coprocessor 0, this mapping is a tad
    /// non-trivial. If you want to use the actual index specified by the MIPS spec, use any
    /// integral `FromPrimitive` method besides `from_isize` or `from_usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use num_traits::FromPrimitive;
    /// # use crate::registers::Coprocessor0Register;
    /// #
    /// assert_eq!(
    ///     Coprocessor0Register::from_isize(0),
    ///     Some(Coprocessor0Register::VirtualAddress)
    /// );
    /// assert_eq!(Coprocessor0Register::from_isize(1), Some(Coprocessor0Register::Status));
    /// assert_eq!(Coprocessor0Register::from_isize(2), Some(Coprocessor0Register::Cause));
    /// assert_eq!(
    ///     Coprocessor0Register::from_isize(3),
    ///     Some(Coprocessor0Register::ErrorProgramCounter)
    /// );
    /// ```
    fn from_isize(n: isize) -> Option<Self> {
        match n {
            0 => Some(Self::VirtualAddress),
            1 => Some(Self::Status),
            2 => Some(Self::Cause),
            3 => Some(Self::ErrorProgramCounter),
            _ => None,
        }
    }

    /// Converts an unsigned index in the range `0..=3` to the `Coprocessor0Register` mapped to that
    /// index. Because there are only four registers in Coprocessor 0, this mapping is a tad
    /// non-trivial. If you want to use the actual index specified by the MIPS spec, use any
    /// integral `FromPrimitive` method besides `from_isize` or `from_usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use num_traits::FromPrimitive;
    /// # use crate::registers::Coprocessor0Register;
    /// #
    /// assert_eq!(
    ///     Coprocessor0Register::from_usize(0),
    ///     Some(Coprocessor0Register::VirtualAddress)
    /// );
    /// assert_eq!(Coprocessor0Register::from_usize(1), Some(Coprocessor0Register::Status));
    /// assert_eq!(Coprocessor0Register::from_usize(2), Some(Coprocessor0Register::Cause));
    /// assert_eq!(
    ///     Coprocessor0Register::from_usize(3),
    ///     Some(Coprocessor0Register::ErrorProgramCounter)
    /// );
    /// ```
    fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(Self::VirtualAddress),
            1 => Some(Self::Status),
            2 => Some(Self::Cause),
            3 => Some(Self::ErrorProgramCounter),
            _ => None,
        }
    }
}

impl ToPrimitive for Coprocessor0Register {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Self::VirtualAddress => Some(8),
            Self::Status => Some(12),
            Self::Cause => Some(13),
            Self::ErrorProgramCounter => Some(14),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Self::VirtualAddress => Some(8),
            Self::Status => Some(12),
            Self::Cause => Some(13),
            Self::ErrorProgramCounter => Some(14),
        }
    }

    fn to_isize(&self) -> Option<isize> {
        match self {
            Self::VirtualAddress => Some(0),
            Self::Status => Some(1),
            Self::Cause => Some(2),
            Self::ErrorProgramCounter => Some(3),
        }
    }

    fn to_usize(&self) -> Option<usize> {
        match self {
            Self::VirtualAddress => Some(0),
            Self::Status => Some(1),
            Self::Cause => Some(2),
            Self::ErrorProgramCounter => Some(3),
        }
    }
}

impl RegisterSet for Coprocessor0Register {
    const NUM_REGISTERS: usize = 4;
    const REGISTER_NAMES: &'static [&'static str] = &["vaddr", "status", "cause", "epc"];
}

make_registers_format!(Coprocessor0Register);
