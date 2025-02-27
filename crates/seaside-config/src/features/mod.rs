pub mod syscalls;

use crate::{EditFromBinary, FromBinary, ToBinary, Validate};
use anyhow::{anyhow, Result};
use seaside_int_utils::AllZeroes;
use std::io::{Read, Write};
use syscalls::Syscalls;

/// Customizes the features available to the seaside engine.
pub struct Features {
    /// Allow users to provide code and/or data relating to kernel space.
    pub kernel_space_accessible: bool,
    /// Enable run-time modification of text segments.
    pub self_modifying_code: bool,
    /// Simulate the delay slot.
    pub delay_slot: bool,
    /// Allow `sbrk` to free memory when given a negative argument.
    pub freeable_heap_allocations: bool,
    /// Enables displaying a crash handler when an unhandled exception is thrown.
    pub show_crash_handler: bool,
    /// Allow use of pseudo-instructions and formats.
    pub pseudo_instructions: bool,
    /// Set syscalls available to interpreter.
    pub syscalls: Syscalls,
}

impl Validate for Features {
    fn validate(&self) -> Result<()> {
        self.syscalls.validate()
    }
}

impl AllZeroes for Features {
    fn all_zeroes() -> Self {
        Self {
            kernel_space_accessible: false,
            self_modifying_code: false,
            delay_slot: false,
            freeable_heap_allocations: false,
            show_crash_handler: false,
            pseudo_instructions: false,
            syscalls: Syscalls::all_zeroes(),
        }
    }
}

impl EditFromBinary<1> for Features {
    fn edit_from_binary<R: Read>(&mut self, ids: [u8; 4], stream: &mut R) -> Result<()> {
        use crate::properties::features::*;

        match (ids[1], ids[3]) {
            (0x00, KERNEL_SPACE_ACCESSIBLE) => {
                self.kernel_space_accessible = bool::from_binary(stream)?
            }
            (0x00, SELF_MODIFYING_CODE) => self.self_modifying_code = bool::from_binary(stream)?,
            (0x00, DELAY_SLOT) => self.delay_slot = bool::from_binary(stream)?,
            (0x00, FREEABLE_HEAP_ALLOCATIONS) => {
                self.freeable_heap_allocations = bool::from_binary(stream)?
            }
            (0x00, SHOW_CRASH_HANDLER) => self.show_crash_handler = bool::from_binary(stream)?,
            (0x00, PSEUDO_INSTRUCTIONS) => self.pseudo_instructions = bool::from_binary(stream)?,
            (syscalls::ID, _) => {
                <Syscalls as EditFromBinary<1>>::edit_from_binary(&mut self.syscalls, ids, stream)?
            }
            _ => return Err(anyhow!("unknown property id: {}", u32::from_be_bytes(ids))),
        }
        Ok(())
    }
}

impl ToBinary<1> for Features {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        use crate::properties::prefixed::features::*;

        KERNEL_SPACE_ACCESSIBLE.to_binary(stream)?;
        self.kernel_space_accessible.to_binary(stream)?;
        SELF_MODIFYING_CODE.to_binary(stream)?;
        self.self_modifying_code.to_binary(stream)?;
        DELAY_SLOT.to_binary(stream)?;
        self.delay_slot.to_binary(stream)?;
        FREEABLE_HEAP_ALLOCATIONS.to_binary(stream)?;
        self.freeable_heap_allocations.to_binary(stream)?;
        SHOW_CRASH_HANDLER.to_binary(stream)?;
        self.show_crash_handler.to_binary(stream)?;
        PSEUDO_INSTRUCTIONS.to_binary(stream)?;
        self.pseudo_instructions.to_binary(stream)?;
        self.syscalls.to_binary(stream)
    }
}
