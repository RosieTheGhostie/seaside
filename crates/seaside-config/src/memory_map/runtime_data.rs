use super::{AddressRange, Contains, Overlapping, Segment};
use seaside_int_utils::AllZeroes;
use serde::{Deserialize, Serialize};

/// Specifies the memory addresses associated with the heap and stack.
#[derive(Deserialize, Serialize)]
pub struct RuntimeData {
    /// The inclusive range of addresses within this segment.
    #[serde(flatten)]
    pub address_range: AddressRange,
    /// The number of bytes to allocate for the heap.
    pub heap_size: u32,
    /// The number of bytes to allocate for the stack.
    pub stack_size: u32,
}

impl Contains<RuntimeData> for AddressRange {
    fn contains(&self, value: &RuntimeData) -> bool {
        self.contains(&value.address_range)
    }
}

impl Contains<RuntimeData> for Segment {
    fn contains(&self, value: &RuntimeData) -> bool {
        self.address_range.contains(&value.address_range)
    }
}

impl Overlapping<RuntimeData> for AddressRange {
    fn overlapping(&self, value: &RuntimeData) -> bool {
        self.overlapping(&value.address_range)
    }
}

impl Overlapping<RuntimeData> for Segment {
    fn overlapping(&self, value: &RuntimeData) -> bool {
        self.address_range.overlapping(&value.address_range)
    }
}

impl AllZeroes for RuntimeData {
    fn all_zeroes() -> Self {
        Self {
            address_range: AddressRange::all_zeroes(),
            heap_size: 0,
            stack_size: 0,
        }
    }
}
