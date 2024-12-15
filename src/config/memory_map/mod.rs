pub mod address;
pub mod runtime_data;
pub mod segment;

pub use address::{Address, AddressRange};
pub use runtime_data::RuntimeData;
pub use segment::Segment;
use serde::{Deserialize, Serialize};

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
    pub runtime_data: RuntimeData,
    pub ktext: Segment,
    pub kdata: Segment,
    pub mmio: Segment,
}
