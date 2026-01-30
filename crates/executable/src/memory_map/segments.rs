use seaside_address_range::sized::SizedAddressRange;
use serde::{Deserialize, Serialize};

use super::SegmentInfo;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Segments {
    pub text: SegmentInfo,
    pub ktext: SegmentInfo,
    pub r#extern: SegmentInfo,
    pub data: SegmentInfo,
    pub kdata: SegmentInfo,
    pub stack: SizedAddressRange<true>,
    pub heap: SizedAddressRange<false>,
    pub mmio: SegmentInfo,
}
