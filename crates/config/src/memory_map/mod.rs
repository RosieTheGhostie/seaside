pub mod address_range;
pub mod runtime_data;
pub mod segment;
pub mod segments;
pub mod traits;

pub use address_range::AddressRange;
pub use runtime_data::RuntimeData;
pub use segment::Segment;
pub use segments::Segments;

use crate::Validate;
use anyhow::{Error, Result};
use seaside_error::EngineError;
use seaside_int_utils::AllZeroes;
use seaside_type_aliases::Address;
use serde::{Deserialize, Serialize};
use traits::{Contains, Overlapping};

/// Maps various memory regions to [`AddressRange`]s.
///
/// This information is crucial for initializing the interpreter.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemoryMap {
    pub user_space: AddressRange,
    pub kernel_space: AddressRange,
    pub exception_handler: Option<Address>,
    pub segments: Segments,
}

impl AllZeroes for MemoryMap {
    fn all_zeroes() -> Self {
        Self {
            user_space: AddressRange::all_zeroes(),
            kernel_space: AddressRange::all_zeroes(),
            exception_handler: None,
            segments: Segments::all_zeroes(),
        }
    }
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self {
            user_space: AddressRange {
                base: 0x00000000,
                limit: 0x7fffffff,
            },
            kernel_space: AddressRange {
                base: 0x80000000,
                limit: 0xffffffff,
            },
            exception_handler: Some(0x80000180),
            segments: Segments::default(),
        }
    }
}

impl Validate for MemoryMap {
    fn validate(&self) -> Result<()> {
        let error_msg = if AddressRange::overlapping(&self.user_space, &self.kernel_space) {
            Some("user space and kernel space cannot overlap")
        } else if let Some(exception_handler) = self.exception_handler {
            (!self.kernel_space.contains(&exception_handler))
                .then_some("exception handler must be in kernel space")
        } else if !self.user_space.contains(&self.segments.text) {
            Some("text segment must be entirely within user space")
        } else if !self.user_space.contains(&self.segments.r#extern) {
            Some("extern segment must be entirely within user space")
        } else if !self.user_space.contains(&self.segments.data) {
            Some("data segment must be entirely within user space")
        } else if !self.user_space.contains(&self.segments.runtime_data) {
            Some("runtime data must be entirely within user space")
        } else if !self.kernel_space.contains(&self.segments.ktext) {
            Some("ktext segment must be entirely within kernel space")
        } else if !self.kernel_space.contains(&self.segments.kdata) {
            Some("kdata segment must be entirely within kernel space")
        } else if !self.kernel_space.contains(&self.segments.mmio) {
            Some("MMIO segment must be entirely within kernel space")
        } else {
            None
        };
        match error_msg {
            Some(msg) => Err(Error::new(EngineError::InvalidConfig).context(msg)),
            None => self.segments.validate(),
        }
    }
}
