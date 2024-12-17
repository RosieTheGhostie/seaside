use super::{AddressRange, Contains, Overlapping};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Segment {
    /// The inclusive range of addresses within this segment.
    #[serde(flatten)]
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
