use super::{
    AddressRange,
    traits::{Contains, Overlapping},
};
use seaside_int_utils::AllZeroes;
use serde::{Deserialize, Serialize};

/// Specifies the memory addresses associated with a given segment.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Segment {
    /// The inclusive range of addresses within this segment.
    pub range: AddressRange,
    /// The maximum number of bytes to allocate for this segment.
    pub allocate: u32,
}

impl Contains<Segment> for AddressRange {
    fn contains(&self, value: &Segment) -> bool {
        self.contains(&value.range)
    }
}

impl Overlapping<Segment> for Segment {
    fn overlapping(&self, other: &Self) -> bool {
        self.range.overlapping(&other.range)
    }
}

impl AllZeroes for Segment {
    fn all_zeroes() -> Self {
        Self {
            range: AddressRange::all_zeroes(),
            allocate: 0,
        }
    }
}

macro_rules! allocate {
    (@internal $n:expr) => {
        $n
    };
    ($n:literal $(B)?) => {
        $crate::memory_map::segment::allocate!(@internal $n)
    };
    ($n:literal KiB) => {
        $crate::memory_map::segment::allocate!(@internal $n * 1024)
    };
    ($n:literal MiB) => {
        $crate::memory_map::segment::allocate!(@internal $n * 1_048_576)
    };
    ($n:literal GiB) => {
        $crate::memory_map::segment::allocate!(@internal $n * 1_073_741_824)
    };
}
pub(crate) use allocate;

macro_rules! segment {
    ($base:literal..$limit:literal, $allocate:literal $($units:ident)?) => {
        $crate::memory_map::Segment {
            range: $crate::memory_map::address_range::address_range![$base..$limit],
            allocate: $crate::memory_map::segment::allocate!($allocate $($units)?),
        }
    };
    ($base:literal..=$limit:literal, $allocate:literal $($units:ident)?) => {
        $crate::memory_map::Segment {
            range: $crate::memory_map::address_range::address_range![$base..=$limit],
            allocate: $crate::memory_map::segment::allocate!($allocate $($units)?),
        }
    };
}
pub(crate) use segment;
