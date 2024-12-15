use super::AddressRange;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RuntimeData {
    /// The inclusive range of addresses within this segment.
    #[serde(flatten)]
    pub address_range: AddressRange,
    /// The number of bytes to allocate for the heap.
    heap_size: u32,
    /// The number of bytes to allocate for the stack.
    stack_size: u32,
}
