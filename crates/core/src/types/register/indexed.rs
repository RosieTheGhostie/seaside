use core::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use num_traits::FromPrimitive;
use strum::EnumIter;

use super::ParseError;

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
#[repr(u8)]
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

impl IndexedRegister {
    pub const MAX: Self = Self::_31;
    pub const MAX_RAW: u8 = Self::MAX as _;

    /// Attempts to construct an [`IndexedRegister`] from its raw index value.
    ///
    /// This will return [`None`] if `index` is greater than [`IndexedRegister::MAX_RAW`].
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

    /// Constructs an [`IndexedRegister`] from its raw index value.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring `index` is less than or equal to
    /// [`IndexedRegister::MAX_RAW`]. Failure to do so may result in undefined behavior.
    ///
    /// # See Also
    ///
    /// - [`from_raw`](Self::from_raw)
    pub const unsafe fn from_raw_unchecked(index: u8) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(index) }
    }
}

impl Display for IndexedRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
        Self::from_raw(index).ok_or(ParseError::BadValue)
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
