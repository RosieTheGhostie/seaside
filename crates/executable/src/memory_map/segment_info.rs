use seaside_address_range::AddressRange;
use seaside_type_aliases::Size;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SegmentInfo {
    pub range: AddressRange,
    pub allocate: Size,
}
