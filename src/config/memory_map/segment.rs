use super::{Address, AddressRange};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Segment {
    /// The range of addresses within this segment.
    #[serde(flatten)]
    pub address_range: AddressRange,
    /// The maximum number of bytes to allocate for this segment.
    allocate: u32,
}

pub struct SegmentAllocationInfo {
    pub low_address: Address,
    pub bytes_to_allocate: u32,
}

impl Segment {
    fn unpack_members(&self) -> (Address, Option<Address>, u32) {
        (
            self.address_range.base,
            self.address_range.limit,
            self.allocate,
        )
    }

    pub fn get_allocation_info(&self, base_is_low_address: bool) -> SegmentAllocationInfo {
        let (base, limit, allocate) = self.unpack_members();
        let capacity = match (base_is_low_address, limit) {
            (true, Some(limit)) => limit - base + 1,
            (true, None) => u32::MAX - base,
            (false, _) => base - limit.unwrap_or(0) + 1,
        };
        // Don't ask me why this part works; I just fiddled around in Desmos until I got something
        // that seems to produce the right output.
        let low_address = if base_is_low_address || allocate == 0 {
            base
        } else {
            Address::max(
                limit.unwrap_or(0),
                u32::checked_sub(base, allocate - 1).unwrap_or(0),
            )
        };
        SegmentAllocationInfo {
            low_address,
            bytes_to_allocate: u32::min(allocate, capacity),
        }
    }
}
