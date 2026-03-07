use seaside_core::{AddressRange, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SegmentInfo {
    pub range: AddressRange,
    pub allocate: Size,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StackAndHeap {
    pub range: AddressRange,
    pub allocate_stack: Size,
    pub allocate_heap: Size,
}
