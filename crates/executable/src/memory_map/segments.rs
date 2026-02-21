use seaside_core::{AddressRange, traits::Contains};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::{SegmentInfo, StackAndHeap, assert_no_overlap};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Validate)]
#[validate(schema(function = "Self::validate_with_schema"))]
pub struct Segments {
    pub text: SegmentInfo,
    pub ktext: SegmentInfo,
    pub r#extern: SegmentInfo,
    pub data: SegmentInfo,
    pub kdata: SegmentInfo,
    pub stack_and_heap: StackAndHeap,
    pub mmio: SegmentInfo,
}

impl Segments {
    pub(super) fn validate_user_space(
        &self,
        user_space: AddressRange,
    ) -> Result<(), ValidationError> {
        assert_contains(
            &user_space,
            &self.text.range,
            ".text is not entirely contained in user-space",
        )?;
        assert_contains(
            &user_space,
            &self.r#extern.range,
            ".extern is not entirely contained in user-space",
        )?;
        assert_contains(
            &user_space,
            &self.data.range,
            ".data is not entirely contained in user-space",
        )?;
        assert_contains(
            &user_space,
            &self.stack_and_heap.range,
            "space dedicated to stack and heap is not entirely contained in user-space",
        )
    }

    pub(super) fn validate_kernel_space(
        &self,
        kernel_space: AddressRange,
    ) -> Result<(), ValidationError> {
        assert_contains(
            &kernel_space,
            &self.ktext.range,
            ".ktext is not entirely contained in kernel-space",
        )?;
        assert_contains(
            &kernel_space,
            &self.kdata.range,
            ".kdata is not entirely contained in kernel-space",
        )?;
        assert_contains(
            &kernel_space,
            &self.mmio.range,
            "MMIO is not entirely contained in kernel-space",
        )
    }

    fn validate_with_schema(&self) -> Result<(), ValidationError> {
        assert_no_overlap(
            &self.text.range,
            &self.ktext.range,
            ".text overlaps with .ktext",
        )?;
        assert_no_overlap(
            &self.text.range,
            &self.r#extern.range,
            ".text overlaps with .extern",
        )?;
        assert_no_overlap(
            &self.text.range,
            &self.data.range,
            ".text overlaps with .data",
        )?;
        assert_no_overlap(
            &self.text.range,
            &self.kdata.range,
            ".text overlaps with .kdata",
        )?;
        assert_no_overlap(
            &self.text.range,
            &self.stack_and_heap.range,
            ".text overlaps with stack and/or heap",
        )?;
        assert_no_overlap(
            &self.text.range,
            &self.mmio.range,
            ".text overlaps with MMIO",
        )?;
        assert_no_overlap(
            &self.ktext.range,
            &self.r#extern.range,
            ".ktext overlaps with .extern",
        )?;
        assert_no_overlap(
            &self.ktext.range,
            &self.data.range,
            ".ktext overlaps with .data",
        )?;
        assert_no_overlap(
            &self.ktext.range,
            &self.kdata.range,
            ".ktext overlaps with .kdata",
        )?;
        assert_no_overlap(
            &self.ktext.range,
            &self.stack_and_heap.range,
            ".ktext overlaps with stack and/or heap",
        )?;
        assert_no_overlap(
            &self.ktext.range,
            &self.mmio.range,
            ".ktext overlaps with MMIO",
        )?;
        assert_no_overlap(
            &self.r#extern.range,
            &self.data.range,
            ".extern overlaps with .data",
        )?;
        assert_no_overlap(
            &self.r#extern.range,
            &self.kdata.range,
            ".extern overlaps with .kdata",
        )?;
        assert_no_overlap(
            &self.r#extern.range,
            &self.stack_and_heap.range,
            ".extern overlaps with stack and/or heap",
        )?;
        assert_no_overlap(
            &self.r#extern.range,
            &self.mmio.range,
            ".extern overlaps with MMIO",
        )?;
        assert_no_overlap(
            &self.data.range,
            &self.kdata.range,
            ".data overlaps with .kdata",
        )?;
        assert_no_overlap(
            &self.data.range,
            &self.stack_and_heap.range,
            ".data overlaps with stack and/or heap",
        )?;
        assert_no_overlap(
            &self.data.range,
            &self.mmio.range,
            ".data overlaps with MMIO",
        )?;
        assert_no_overlap(
            &self.kdata.range,
            &self.stack_and_heap.range,
            ".kdata overlaps with stack and/or heap",
        )?;
        assert_no_overlap(
            &self.kdata.range,
            &self.mmio.range,
            ".kdata overlaps with MMIO",
        )?;
        assert_no_overlap(
            &self.stack_and_heap.range,
            &self.mmio.range,
            "stack and/or heap overlaps with MMIO",
        )
    }
}

fn assert_contains(
    container: &AddressRange,
    containee: &AddressRange,
    message: &'static str,
) -> Result<(), ValidationError> {
    if container.contains(containee) {
        Ok(())
    } else {
        Err(ValidationError::new("must_contain").with_message(message.into()))
    }
}
