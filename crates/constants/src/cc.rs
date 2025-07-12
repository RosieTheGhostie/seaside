use core::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::FromPrimitive;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConditionCode {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

impl ConditionCode {
    pub const unsafe fn from_u8_unchecked(n: u8) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(n) }
    }
}

impl FromPrimitive for ConditionCode {
    fn from_u8(n: u8) -> Option<Self> {
        (n < 8).then(|| unsafe { Self::from_u8_unchecked(n) })
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

impl Display for ConditionCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u8)
    }
}
