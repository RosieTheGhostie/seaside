use super::{
    address_range::address_range,
    segment::{allocate, segment},
    Overlapping, RuntimeData, Segment,
};
use crate::{EditFromBinary, FromBinary, Validate};
use anyhow::{anyhow, Error, Result};
use seaside_error::EngineError;
use seaside_int_utils::AllZeroes;
use seaside_type_aliases::Address;
use std::io::Read;

/// Collection of segments in the [`MemoryMap`].
///
/// [`MemoryMap`]: super::MemoryMap
#[derive(Clone, Debug)]
pub struct Segments {
    pub text: Segment,
    pub r#extern: Segment,
    pub data: Segment,
    pub runtime_data: RuntimeData,
    pub ktext: Segment,
    pub kdata: Segment,
    pub mmio: Segment,
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

impl Default for Segments {
    fn default() -> Self {
        Self {
            text: segment!(0x00400000..0x10000000, 8 MiB),
            r#extern: segment!(0x10000000..0x10010000, 64 KiB),
            data: segment!(0x10010000..0x10040000, 192 KiB),
            runtime_data: RuntimeData {
                address_range: address_range![0x10040000..0x80000000],
                heap_size: allocate!(128 KiB),
                stack_size: allocate!(4 MiB),
            },
            ktext: segment!(0x80000000..0x90000000, 1 MiB),
            kdata: segment!(0x90000000..0xffff0000, 1 MiB),
            mmio: segment!(0xffff0000..=0xffffffff, 4 KiB),
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
