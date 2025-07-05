use num_traits::FromPrimitive;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConditionCode {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

impl FromPrimitive for ConditionCode {
    fn from_u8(n: u8) -> Option<Self> {
        (n < 8).then(|| unsafe { core::mem::transmute(n) })
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
