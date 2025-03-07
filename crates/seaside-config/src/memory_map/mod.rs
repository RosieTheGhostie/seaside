pub mod address_range;
pub mod runtime_data;
pub mod segment;
pub mod segments;
pub mod traits;

pub use address_range::AddressRange;
pub use runtime_data::RuntimeData;
pub use segment::Segment;
pub use segments::Segments;

use crate::{prefixed, EditFromBinary, FromBinary, ToBinary, Validate};
use anyhow::{anyhow, Error, Result};
use seaside_error::EngineError;
use seaside_int_utils::AllZeroes;
use seaside_type_aliases::Address;
use std::io::{Read, Write};
use traits::{Contains, Overlapping};

/// Maps various memory regions to [`AddressRange`]s.
///
/// This information is crucial for initializing the interpreter.
#[derive(Clone, Debug)]
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

impl ToBinary<1> for MemoryMap {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        if let Some(exception_handler) = self.exception_handler {
            prefixed!(memory_map[EXCEPTION_HANDLER]).to_binary(stream)?;
            exception_handler.to_binary(stream)?;
        }
        prefixed!(memory_map::user_space[BASE]).to_binary(stream)?;
        self.user_space.base.to_binary(stream)?;
        prefixed!(memory_map::user_space[LIMIT]).to_binary(stream)?;
        self.user_space.limit.to_binary(stream)?;

        prefixed!(memory_map::kernel_space[BASE]).to_binary(stream)?;
        self.kernel_space.base.to_binary(stream)?;
        prefixed!(memory_map::kernel_space[LIMIT]).to_binary(stream)?;
        self.kernel_space.limit.to_binary(stream)?;

        prefixed!(memory_map::segments::text[BASE]).to_binary(stream)?;
        self.segments.text.address_range.base.to_binary(stream)?;
        prefixed!(memory_map::segments::text[LIMIT]).to_binary(stream)?;
        self.segments.text.address_range.limit.to_binary(stream)?;
        prefixed!(memory_map::segments::text[ALLOCATE]).to_binary(stream)?;
        self.segments.text.allocate.to_binary(stream)?;

        prefixed!(memory_map::segments::r#extern[BASE]).to_binary(stream)?;
        self.segments
            .r#extern
            .address_range
            .base
            .to_binary(stream)?;
        prefixed!(memory_map::segments::r#extern[LIMIT]).to_binary(stream)?;
        self.segments
            .r#extern
            .address_range
            .limit
            .to_binary(stream)?;
        prefixed!(memory_map::segments::r#extern[ALLOCATE]).to_binary(stream)?;
        self.segments.r#extern.allocate.to_binary(stream)?;

        prefixed!(memory_map::segments::data[BASE]).to_binary(stream)?;
        self.segments.data.address_range.base.to_binary(stream)?;
        prefixed!(memory_map::segments::data[LIMIT]).to_binary(stream)?;
        self.segments.data.address_range.limit.to_binary(stream)?;
        prefixed!(memory_map::segments::data[ALLOCATE]).to_binary(stream)?;
        self.segments.data.allocate.to_binary(stream)?;

        prefixed!(memory_map::segments::runtime_data[BASE]).to_binary(stream)?;
        self.segments
            .runtime_data
            .address_range
            .base
            .to_binary(stream)?;
        prefixed!(memory_map::segments::runtime_data[LIMIT]).to_binary(stream)?;
        self.segments
            .runtime_data
            .address_range
            .limit
            .to_binary(stream)?;
        prefixed!(memory_map::segments::runtime_data[HEAP_SIZE]).to_binary(stream)?;
        self.segments.runtime_data.heap_size.to_binary(stream)?;
        prefixed!(memory_map::segments::runtime_data[STACK_SIZE]).to_binary(stream)?;
        self.segments.runtime_data.stack_size.to_binary(stream)?;

        prefixed!(memory_map::segments::ktext[BASE]).to_binary(stream)?;
        self.segments.ktext.address_range.base.to_binary(stream)?;
        prefixed!(memory_map::segments::ktext[LIMIT]).to_binary(stream)?;
        self.segments.ktext.address_range.limit.to_binary(stream)?;
        prefixed!(memory_map::segments::ktext[ALLOCATE]).to_binary(stream)?;
        self.segments.ktext.allocate.to_binary(stream)?;

        prefixed!(memory_map::segments::kdata[BASE]).to_binary(stream)?;
        self.segments.kdata.address_range.base.to_binary(stream)?;
        prefixed!(memory_map::segments::kdata[LIMIT]).to_binary(stream)?;
        self.segments.kdata.address_range.limit.to_binary(stream)?;
        prefixed!(memory_map::segments::kdata[ALLOCATE]).to_binary(stream)?;
        self.segments.kdata.allocate.to_binary(stream)?;

        prefixed!(memory_map::segments::mmio[BASE]).to_binary(stream)?;
        self.segments.mmio.address_range.base.to_binary(stream)?;
        prefixed!(memory_map::segments::mmio[LIMIT]).to_binary(stream)?;
        self.segments.mmio.address_range.limit.to_binary(stream)?;
        prefixed!(memory_map::segments::mmio[ALLOCATE]).to_binary(stream)?;
        self.segments.mmio.allocate.to_binary(stream)
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
