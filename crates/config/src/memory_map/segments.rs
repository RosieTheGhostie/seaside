use super::{
    Overlapping, RuntimeData, Segment,
    address_range::address_range,
    segment::{allocate, segment},
};
use crate::Validate;
use anyhow::{Error, Result};
use seaside_error::EngineError;
use seaside_int_utils::AllZeroes;
use serde::{Deserialize, Serialize};

/// Collection of segments in the [`MemoryMap`](super::MemoryMap).
#[derive(Clone, Debug, Deserialize, Serialize)]
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
                range: address_range![0x10040000..0x80000000],
                heap_size: allocate!(128 KiB),
                stack_size: allocate!(4 MiB),
            },
            ktext: segment!(0x80000000..0x90000000, 1 MiB),
            kdata: segment!(0x90000000..0xffff0000, 1 MiB),
            mmio: segment!(0xffff0000..=0xffffffff, 4 KiB),
        }
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
