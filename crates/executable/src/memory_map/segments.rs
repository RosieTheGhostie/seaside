use serde::{Deserialize, Serialize};

use super::{SegmentInfo, StackAndHeap};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Segments {
    pub text: SegmentInfo,
    pub ktext: SegmentInfo,
    pub r#extern: SegmentInfo,
    pub data: SegmentInfo,
    pub kdata: SegmentInfo,
    pub stack_and_heap: StackAndHeap,
    pub mmio: SegmentInfo,
}
