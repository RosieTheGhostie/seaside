#![allow(dead_code)]
pub mod assembler;
pub mod syscalls;

use super::{
    primitive_defaults::{r#false, r#true},
    validate::Validate,
};
use crate::engine::Error;
use assembler::AssemblerOptions;
use serde::{Deserialize, Serialize};
use syscalls::Syscalls;

/// Customizes the features available to the seaside engine.
#[derive(Serialize, Deserialize)]
pub struct Features {
    /// Allow users to provide code and/or data relating to kernel space.
    #[serde(default = "r#true")]
    pub kernel_space_accessible: bool,
    /// Enable run-time modification of text segments.
    #[serde(default = "r#false")]
    pub self_modifying_code: bool,
    /// Simulate the delay slot.
    #[serde(default = "r#false")]
    pub delay_slot: bool,
    /// Enables displaying a crash handler when an unhandled exception is thrown.
    #[serde(default = "r#true")]
    pub show_crash_handler: bool,
    /// Set features available to assembler.
    pub assembler: AssemblerOptions,
    /// Set syscalls available to interpreter.
    pub syscalls: Syscalls,
}

impl Validate for Features {
    fn validate(&self) -> Result<(), Error> {
        self.syscalls.validate()
    }
}
