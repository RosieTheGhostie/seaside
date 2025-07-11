use super::ParseError;
use core::{
    fmt::{Display, Formatter, Result as FmtResult, Write},
    str::FromStr,
};
use num_traits::FromPrimitive;
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IndexedRegister {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
    _16,
    _17,
    _18,
    _19,
    _20,
    _21,
    _22,
    _23,
    _24,
    _25,
    _26,
    _27,
    _28,
    _29,
    _30,
    _31,
}

impl Display for IndexedRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            f.write_char('$')?;
        }
        write!(f, "{}", *self as u8)
    }
}

impl FromStr for IndexedRegister {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('$').unwrap_or(s);
        if s.is_empty() {
            return Err(ParseError::Empty);
        }
        if s.starts_with('0') {
            return if s.len() == 1 {
                Ok(Self::_0)
            } else {
                Err(ParseError::BadValue)
            };
        }
        let index: u8 = s.parse().map_err(|_| ParseError::BadValue)?;
        if index < 32 {
            Ok(unsafe { core::mem::transmute::<_, Self>(index) })
        } else {
            Err(ParseError::BadValue)
        }
    }
}

impl FromPrimitive for IndexedRegister {
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
