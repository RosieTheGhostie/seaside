pub mod segment_info;
pub mod segments;

pub use segment_info::{SegmentInfo, StackAndHeap};
pub use segments::Segments;

use seaside_address_range::{AddressRange, traits::Overlapping};
use seaside_type_aliases::Address;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Validate)]
#[validate(schema(function = "Self::validate_with_schema"))]
pub struct MemoryMap {
    pub exception_handler: Option<Address>,
    pub user_space: AddressRange,
    pub kernel_space: AddressRange,
    pub segments: Segments,
}

impl MemoryMap {
    fn validate_with_schema(&self) -> Result<(), ValidationError> {
        if let Some(exception_handler) = self.exception_handler
            && !self.kernel_space.contains_address(exception_handler)
        {
            return Err(ValidationError::new("contains")
                .with_message("exception handler is not within kernel-space".into()));
        }

        if self.user_space.overlapping(&self.kernel_space) {
            return Err(ValidationError::new("no_overlap")
                .with_message("user-space overlaps with kernel-space".into()));
        }

        Ok(())
    }
}
