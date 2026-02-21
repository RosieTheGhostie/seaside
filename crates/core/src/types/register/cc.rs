use core::fmt::{self, Display, Formatter};

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
    pub const MAX: Self = Self::_7;
    pub const MAX_RAW: u8 = Self::MAX as _;

    /// Attempts to construct a [`ConditionCode`] from its raw index value.
    ///
    /// This will return [`None`] if `index` is greater than [`ConditionCode::MAX_RAW`].
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

    /// Constructs a [`ConditionCode`] from its raw index value.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring `index` is less than or equal to
    /// [`ConditionCode::MAX_RAW`]. Failure to do so may result in undefined behavior.
    ///
    /// # See Also
    ///
    /// - [`from_raw`](Self::from_raw)
    pub const unsafe fn from_raw_unchecked(index: u8) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(index) }
    }
}

impl FromPrimitive for ConditionCode {
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

impl Display for ConditionCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}
