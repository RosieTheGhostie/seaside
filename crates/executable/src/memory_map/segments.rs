use seaside_address_range::sized::SizedAddressRange;

use super::SegmentInfo;
use crate::Location;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

crate::ser::fixed_size::r#impl!(
    for Segments;

    // `text`, `ktext`, `extern`, `data`, `kdata`, and `mmio`
    6 * SegmentInfo,

    // location data for each of the static segments
    5 * Location,

    // `stack` and `heap`
    2 * SizedAddressRange,
);
