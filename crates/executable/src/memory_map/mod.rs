pub mod segment_info;
pub mod segments;

pub use segment_info::SegmentInfo;
pub use segments::Segments;

use seaside_address_range::AddressRange;
use seaside_type_aliases::Address;

use crate::{Tag, Tagged};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MemoryMap {
    pub exception_handler: Option<Address>,
    pub user_space: AddressRange,
    pub kernel_space: AddressRange,
    pub segments: Segments,
}

crate::ser::fixed_size::r#impl!(for MemoryMap; Option<Address>, 2 * AddressRange, Segments);

impl Tagged for MemoryMap {
    const TAG: Tag = *b"mmap";
}
