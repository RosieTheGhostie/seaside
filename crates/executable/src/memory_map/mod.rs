pub mod segment_info;
pub mod segments;

pub use segment_info::{SegmentInfo, StackAndHeap};
pub use segments::Segments;

use seaside_core::{AddressRange, prelude::*, traits::Overlapping};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Validate)]
#[validate(schema(function = "Self::validate_with_schema"))]
pub struct MemoryMap {
    pub exception_handler: Option<Address>,
    pub user_space: AddressRange,
    pub kernel_space: AddressRange,

    #[validate(nested)]
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

        assert_no_overlap(
            &self.user_space,
            &self.kernel_space,
            "user-space overlaps with kernel-space",
        )?;

        self.segments.validate_user_space(self.user_space)?;
        self.segments.validate_kernel_space(self.kernel_space)
    }
}

fn assert_no_overlap(
    range_0: &AddressRange,
    range_1: &AddressRange,
    message: &'static str,
) -> Result<(), ValidationError> {
    if !range_0.overlapping(range_1) {
        Ok(())
    } else {
        Err(ValidationError::new("no_overlap").with_message(message.into()))
    }
}
