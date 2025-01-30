pub mod address_range;
pub mod runtime_data;
pub mod segment;
pub mod traits;

pub use address_range::AddressRange;
pub use runtime_data::RuntimeData;
pub use segment::Segment;

use super::validate::Validate;
use crate::{
    engine::{Error, ErrorKind},
    type_aliases::Address,
};
use serde::{Deserialize, Serialize};
use traits::{Contains, Overlapping};

/// Maps various memory regions to [`AddressRange`]s.
///
/// This information is crucial for initializing the
/// [`Interpreter`][crate::interpreter::Interpreter].
#[derive(Serialize, Deserialize)]
pub struct MemoryMap {
    pub user_space: AddressRange,
    pub kernel_space: AddressRange,
    pub exception_handler: Option<Address>,
    pub segments: Segments,
}

/// Collection of segments in the [`MemoryMap`].
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

impl Validate for MemoryMap {
    fn validate(&self) -> Result<(), Error> {
        let error_msg = if AddressRange::overlapping(&self.user_space, &self.kernel_space) {
            Some("user space and kernel space cannot overlap")
        } else if let Some(exception_handler) = self.exception_handler {
            if !self.kernel_space.contains(&exception_handler) {
                Some("exception handler must be in kernel space")
            } else {
                None
            }
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
            Some(msg) => Err(Error::new(ErrorKind::InvalidConfig, msg)),
            None => self.segments.validate(),
        }
    }
}

impl Validate for Segments {
    fn validate(&self) -> Result<(), Error> {
        let error_msg = if self.text.overlapping(&self.r#extern) {
            Some("text segment cannot overlap with extern segment")
        } else if self.text.overlapping(&self.data) {
            Some("text segment cannot overlap with data segment")
        } else if self.text.overlapping(&self.runtime_data) {
            Some("text segment cannot overlap with runtime data")
        } else if self.r#extern.overlapping(&self.data) {
            Some("extern segment cannot overlap with data segment")
        } else if self.r#extern.overlapping(&self.runtime_data) {
            Some("extern segment cannot overlap with runtime data")
        } else if self.data.overlapping(&self.runtime_data) {
            Some("data segment cannot overlap with runtime data")
        } else if self.ktext.overlapping(&self.kdata) {
            Some("ktext segment cannot overlap with kdata segment")
        } else if self.ktext.overlapping(&self.mmio) {
            Some("ktext segment cannot overlap with MMIO segment")
        } else if self.kdata.overlapping(&self.mmio) {
            Some("kdata segment cannot overlap with MMIO segment")
        } else {
            None
        };
        match error_msg {
            Some(msg) => Err(Error::new(ErrorKind::InvalidConfig, msg)),
            None => Ok(()),
        }
    }
}
