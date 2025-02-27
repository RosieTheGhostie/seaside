pub mod address_range;
pub mod runtime_data;
pub mod segment;
pub mod traits;

pub use address_range::AddressRange;
pub use runtime_data::RuntimeData;
pub use segment::Segment;

use crate::ToBinary;

use super::{
    binary::traits::{EditFromBinary, FromBinary},
    validate::Validate,
};
use anyhow::{anyhow, Error, Result};
use seaside_error::EngineError;
use seaside_int_utils::AllZeroes;
use seaside_type_aliases::Address;
use std::io::{Read, Write};
use traits::{Contains, Overlapping};

/// Maps various memory regions to [`AddressRange`]s.
///
/// This information is crucial for initializing the interpreter.
pub struct MemoryMap {
    pub user_space: AddressRange,
    pub kernel_space: AddressRange,
    pub exception_handler: Option<Address>,
    pub segments: Segments,
}

/// Collection of segments in the [`MemoryMap`].
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

impl EditFromBinary<1> for MemoryMap {
    fn edit_from_binary<R: Read>(&mut self, ids: [u8; 4], stream: &mut R) -> Result<()> {
        use crate::properties::memory_map::*;

        match (ids[1], ids[3]) {
            (0x00, EXCEPTION_HANDLER) => {
                self.exception_handler = Some(Address::from_binary(stream)?)
            }
            (user_space::ID, user_space::BASE) => {
                self.user_space.base = Address::from_binary(stream)?
            }
            (user_space::ID, user_space::LIMIT) => {
                self.user_space.limit = Address::from_binary(stream)?
            }
            (kernel_space::ID, kernel_space::BASE) => {
                self.kernel_space.base = Address::from_binary(stream)?
            }
            (kernel_space::ID, kernel_space::LIMIT) => {
                self.kernel_space.limit = Address::from_binary(stream)?
            }
            (segments::ID, _) => {
                <Segments as EditFromBinary<1>>::edit_from_binary(&mut self.segments, ids, stream)?
            }
            _ => return Err(anyhow!("unknown property id: {}", u32::from_be_bytes(ids))),
        }
        Ok(())
    }
}

impl Validate for Segments {
    fn validate(&self) -> Result<()> {
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
            Some(msg) => Err(Error::new(EngineError::InvalidConfig).context(msg)),
            None => Ok(()),
        }
    }
}

impl AllZeroes for Segments {
    fn all_zeroes() -> Self {
        Self {
            text: Segment::all_zeroes(),
            r#extern: Segment::all_zeroes(),
            data: Segment::all_zeroes(),
            runtime_data: RuntimeData::all_zeroes(),
            ktext: Segment::all_zeroes(),
            kdata: Segment::all_zeroes(),
            mmio: Segment::all_zeroes(),
        }
    }
}

impl EditFromBinary<1> for Segments {
    fn edit_from_binary<R: Read>(&mut self, ids: [u8; 4], stream: &mut R) -> Result<()> {
        use crate::properties::memory_map::segments::*;

        match (ids[2], ids[3]) {
            (text::ID, text::BASE) => self.text.address_range.base = Address::from_binary(stream)?,
            (text::ID, text::LIMIT) => {
                self.text.address_range.limit = Address::from_binary(stream)?
            }
            (text::ID, text::ALLOCATE) => self.text.allocate = u32::from_binary(stream)?,
            (r#extern::ID, r#extern::BASE) => {
                self.r#extern.address_range.base = Address::from_binary(stream)?
            }
            (r#extern::ID, r#extern::LIMIT) => {
                self.r#extern.address_range.limit = Address::from_binary(stream)?
            }
            (r#extern::ID, r#extern::ALLOCATE) => {
                self.r#extern.allocate = u32::from_binary(stream)?
            }
            (data::ID, data::BASE) => self.data.address_range.base = Address::from_binary(stream)?,
            (data::ID, data::LIMIT) => {
                self.data.address_range.limit = Address::from_binary(stream)?
            }
            (data::ID, data::ALLOCATE) => self.data.allocate = u32::from_binary(stream)?,
            (runtime_data::ID, runtime_data::BASE) => {
                self.runtime_data.address_range.base = Address::from_binary(stream)?
            }
            (runtime_data::ID, runtime_data::LIMIT) => {
                self.runtime_data.address_range.limit = Address::from_binary(stream)?
            }
            (runtime_data::ID, runtime_data::HEAP_SIZE) => {
                self.runtime_data.heap_size = u32::from_binary(stream)?
            }
            (runtime_data::ID, runtime_data::STACK_SIZE) => {
                self.runtime_data.stack_size = u32::from_binary(stream)?
            }
            (ktext::ID, ktext::BASE) => {
                self.ktext.address_range.base = Address::from_binary(stream)?
            }
            (ktext::ID, ktext::LIMIT) => {
                self.ktext.address_range.limit = Address::from_binary(stream)?
            }
            (ktext::ID, ktext::ALLOCATE) => self.ktext.allocate = u32::from_binary(stream)?,
            (kdata::ID, kdata::BASE) => {
                self.kdata.address_range.base = Address::from_binary(stream)?
            }
            (kdata::ID, kdata::LIMIT) => {
                self.kdata.address_range.limit = Address::from_binary(stream)?
            }
            (kdata::ID, kdata::ALLOCATE) => self.kdata.allocate = u32::from_binary(stream)?,
            (mmio::ID, mmio::BASE) => self.mmio.address_range.base = Address::from_binary(stream)?,
            (mmio::ID, mmio::LIMIT) => {
                self.mmio.address_range.limit = Address::from_binary(stream)?
            }
            (mmio::ID, mmio::ALLOCATE) => self.mmio.allocate = u32::from_binary(stream)?,
            _ => return Err(anyhow!("unknown property id: {}", u32::from_be_bytes(ids))),
        }
        Ok(())
    }
}

impl ToBinary<1> for MemoryMap {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        use crate::properties::prefixed::memory_map::*;

        if let Some(exception_handler) = self.exception_handler {
            EXCEPTION_HANDLER.to_binary(stream)?;
            exception_handler.to_binary(stream)?;
        }
        user_space::BASE.to_binary(stream)?;
        self.user_space.base.to_binary(stream)?;
        user_space::LIMIT.to_binary(stream)?;
        self.user_space.limit.to_binary(stream)?;
        kernel_space::BASE.to_binary(stream)?;
        self.kernel_space.base.to_binary(stream)?;
        kernel_space::LIMIT.to_binary(stream)?;
        self.kernel_space.limit.to_binary(stream)?;

        segments::text::BASE.to_binary(stream)?;
        self.segments.text.address_range.base.to_binary(stream)?;
        segments::text::LIMIT.to_binary(stream)?;
        self.segments.text.address_range.limit.to_binary(stream)?;
        segments::text::ALLOCATE.to_binary(stream)?;
        self.segments.text.allocate.to_binary(stream)?;

        segments::r#extern::BASE.to_binary(stream)?;
        self.segments
            .r#extern
            .address_range
            .base
            .to_binary(stream)?;
        segments::r#extern::LIMIT.to_binary(stream)?;
        self.segments
            .r#extern
            .address_range
            .limit
            .to_binary(stream)?;
        segments::r#extern::ALLOCATE.to_binary(stream)?;
        self.segments.r#extern.allocate.to_binary(stream)?;

        segments::data::BASE.to_binary(stream)?;
        self.segments.data.address_range.base.to_binary(stream)?;
        segments::data::LIMIT.to_binary(stream)?;
        self.segments.data.address_range.limit.to_binary(stream)?;
        segments::data::ALLOCATE.to_binary(stream)?;
        self.segments.data.allocate.to_binary(stream)?;

        segments::runtime_data::BASE.to_binary(stream)?;
        self.segments
            .runtime_data
            .address_range
            .base
            .to_binary(stream)?;
        segments::runtime_data::LIMIT.to_binary(stream)?;
        self.segments
            .runtime_data
            .address_range
            .limit
            .to_binary(stream)?;
        segments::runtime_data::HEAP_SIZE.to_binary(stream)?;
        self.segments.runtime_data.heap_size.to_binary(stream)?;
        segments::runtime_data::STACK_SIZE.to_binary(stream)?;
        self.segments.runtime_data.stack_size.to_binary(stream)?;

        segments::ktext::BASE.to_binary(stream)?;
        self.segments.ktext.address_range.base.to_binary(stream)?;
        segments::ktext::LIMIT.to_binary(stream)?;
        self.segments.ktext.address_range.limit.to_binary(stream)?;
        segments::ktext::ALLOCATE.to_binary(stream)?;
        self.segments.ktext.allocate.to_binary(stream)?;

        segments::kdata::BASE.to_binary(stream)?;
        self.segments.kdata.address_range.base.to_binary(stream)?;
        segments::kdata::LIMIT.to_binary(stream)?;
        self.segments.kdata.address_range.limit.to_binary(stream)?;
        segments::kdata::ALLOCATE.to_binary(stream)?;
        self.segments.kdata.allocate.to_binary(stream)?;

        segments::mmio::BASE.to_binary(stream)?;
        self.segments.mmio.address_range.base.to_binary(stream)?;
        segments::mmio::LIMIT.to_binary(stream)?;
        self.segments.mmio.address_range.limit.to_binary(stream)?;
        segments::mmio::ALLOCATE.to_binary(stream)?;
        self.segments.mmio.allocate.to_binary(stream)
    }
}
