use serde::{self, Deserialize, Serialize};

pub type Address = u32;

#[derive(Serialize, Deserialize)]
pub struct AddressRange {
    pub base: Address,
    pub limit: Option<Address>,
}

#[derive(Serialize, Deserialize)]
pub struct MemoryMap {
    pub user_space: AddressRange,
    pub kernel_space: AddressRange,
    pub exception_handler: Address,
    pub segments: Segments,
}

#[derive(Serialize, Deserialize)]
pub struct Segments {
    pub text: Segment,
    pub r#extern: Segment,
    pub data: Segment,
    pub heap: Segment,
    pub stack: Segment,
    pub ktext: Segment,
    pub kdata: Segment,
    pub mmio: Segment,
}

#[derive(Serialize, Deserialize)]
pub struct Segment {
    /// The range of addresses within this segment.
    #[serde(flatten)]
    pub address_range: AddressRange,
    /// How many bytes to allocate for this segment.
    pub allocate: u32,
}
