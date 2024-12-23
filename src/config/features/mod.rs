#![allow(dead_code)]
pub mod assembler;
pub mod syscalls;

use super::validate::Validate;
use assembler::AssemblerOptions;
use serde::{Deserialize, Serialize};
use syscalls::Syscalls;

/// Customizes the features available to the seaside engine.
#[derive(Serialize, Deserialize)]
pub struct Features {
    /// Allow users to provide code and/or data relating to kernel space.
    pub kernel_space_accessible: bool,
    /// Enable run-time modification of text segments.
    pub self_modifying_code: bool,
    /// Simulate the delay slot.
    pub delay_slot: bool,
    /// Enables displaying a crash handler when an unhandled exception is thrown.
    pub show_crash_handler: bool,
    /// Set features available to assembler.
    pub assembler: AssemblerOptions,
    /// Set syscalls available to interpreter.
    pub syscalls: Syscalls,
}

impl Validate for Features {
    fn validate(&self) -> Result<(), crate::engine::Error> {
        self.syscalls.validate()
    }
}
