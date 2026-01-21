use seaside_int_utils::AllZeroes;
use seaside_type_aliases::Size;
use serde::{Deserialize, Serialize};

use super::{
    AddressRange, Segment,
    traits::{Contains, Overlapping},
};

/// Specifies the memory addresses associated with the heap and stack.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RuntimeData {
    /// The inclusive range of addresses within this segment.
    pub range: AddressRange,
    /// The number of bytes to allocate for the heap.
    pub heap_size: Size,
    /// The number of bytes to allocate for the stack.
    pub stack_size: Size,
}

impl Contains<RuntimeData> for AddressRange {
    fn contains(&self, value: &RuntimeData) -> bool {
        self.contains(&value.range)
    }
}

impl Contains<RuntimeData> for Segment {
    fn contains(&self, value: &RuntimeData) -> bool {
        self.range.contains(&value.range)
    }
}

impl Overlapping<RuntimeData> for AddressRange {
    fn overlapping(&self, value: &RuntimeData) -> bool {
        self.overlapping(&value.range)
    }
}

impl Overlapping<RuntimeData> for Segment {
    fn overlapping(&self, value: &RuntimeData) -> bool {
        self.range.overlapping(&value.range)
    }
}

impl AllZeroes for RuntimeData {
    fn all_zeroes() -> Self {
        Self {
            range: AddressRange::all_zeroes(),
            heap_size: 0,
            stack_size: 0,
        }
    }
}
