use core::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use num_derive::FromPrimitive;
use strum::EnumIter;

use super::ParseError;

#[derive(Clone, Copy, Debug, EnumIter, Eq, FromPrimitive, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Coprocessor0Register {
    VirtualAddr = 8,
    Status = 12,
    Cause = 13,
    ErrorPc = 14,
}

impl Coprocessor0Register {
    pub const VADDR: u8 = Self::VirtualAddr as u8;
    pub const STATUS: u8 = Self::Status as u8;
    pub const CAUSE: u8 = Self::Cause as u8;
    pub const EPC: u8 = Self::ErrorPc as u8;

    pub const N_REGISTERS: usize = 4;
    pub const NAMES: [&str; Self::N_REGISTERS] = [
        Self::VADDR_NAME,
        Self::STATUS_NAME,
        Self::CAUSE_NAME,
        Self::EPC_NAME,
    ];

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

    const VADDR_NAME: &str = "vaddr";
    const STATUS_NAME: &str = "status";
    const CAUSE_NAME: &str = "cause";
    const EPC_NAME: &str = "epc";
}

impl Display for Coprocessor0Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
            Self::VADDR_NAME | "8" => Ok(Self::VirtualAddr),
            Self::STATUS_NAME | "12" => Ok(Self::Status),
            Self::CAUSE_NAME | "13" => Ok(Self::Cause),
            Self::EPC_NAME | "14" => Ok(Self::ErrorPc),
            "" => Err(ParseError::Empty),
            _ => Err(ParseError::BadValue),
        }
    }
}
