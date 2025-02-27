use super::{
    traits::{Contains, Overlapping},
    AddressRange,
};
use seaside_int_utils::AllZeroes;

/// Specifies the memory addresses associated with a given segment.
pub struct Segment {
    /// The inclusive range of addresses within this segment.
    pub address_range: AddressRange,
    /// The maximum number of bytes to allocate for this segment.
    pub allocate: u32,
}

impl Contains<Segment> for AddressRange {
    fn contains(&self, value: &Segment) -> bool {
        self.contains(&value.address_range)
    }
}

impl Overlapping<Segment> for Segment {
    fn overlapping(&self, other: &Self) -> bool {
        self.address_range.overlapping(&other.address_range)
    }
}

impl AllZeroes for Segment {
    fn all_zeroes() -> Self {
        Self {
            address_range: AddressRange::all_zeroes(),
            allocate: 0,
        }
    }
}
