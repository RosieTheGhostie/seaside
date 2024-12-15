use super::AddressRange;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Segment {
    /// The inclusive range of addresses within this segment.
    #[serde(flatten)]
    pub address_range: AddressRange,
    /// The maximum number of bytes to allocate for this segment.
    allocate: u32,
}
