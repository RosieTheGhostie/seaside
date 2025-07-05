use super::ParseError;
use core::{
    fmt::{Display, Formatter, Result as FmtResult, Write},
    str::FromStr,
};
use num_derive::FromPrimitive;
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Clone, Copy, Debug, EnumIter, Eq, FromPrimitive, Ord, PartialEq, PartialOrd)]
pub enum Coprocessor0Register {
    VirtualAddr = 8,
    Status = 12,
    Cause = 13,
    ErrorPc = 14,
}

impl Display for Coprocessor0Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            f.write_char('$')?;
        }
        f.write_str(self.name())
    }
}

impl FromStr for Coprocessor0Register {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix('$').unwrap_or(s) {
            "vaddr" => Ok(Self::VirtualAddr),
            "status" => Ok(Self::Status),
            "cause" => Ok(Self::Cause),
            "epc" => Ok(Self::ErrorPc),
            "" => Err(ParseError::Empty),
            _ => Err(ParseError::BadValue),
        }
    }
}

impl Coprocessor0Register {
    pub const VADDR: u8 = Self::VirtualAddr as u8;
    pub const STATUS: u8 = Self::Status as u8;
    pub const CAUSE: u8 = Self::Cause as u8;
    pub const EPC: u8 = Self::ErrorPc as u8;

    pub const N_REGISTERS: usize = 4;
    pub const NAMES: [&str; Self::N_REGISTERS] = ["vaddr", "status", "cause", "epc"];

    pub const fn name(&self) -> &'static str {
        Self::NAMES[self.into_index()]
    }

    pub const fn into_index(self) -> usize {
        match self {
            Self::VirtualAddr => 0,
            Self::Status => 1,
            Self::Cause => 2,
            Self::ErrorPc => 3,
        }
    }
}
