pub mod assembler;

pub use assembler::AssemblerOptions;

use seaside_constants::Services;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::primitive_defaults::{r#false, r#true};

/// Customizes the features available to the seaside engine.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Validate)]
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

    /// Allow `sbrk` to free memory when given a negative argument.
    #[serde(default = "r#false")]
    pub freeable_heap_allocations: bool,

    /// Enables displaying a crash handler when an unhandled exception is thrown.
    #[serde(default = "r#true")]
    pub show_crash_handler: bool,

    /// Set features available to assembler.
    pub assembler: AssemblerOptions,

    /// Set system services available to interpreter.
    #[serde(alias = "syscalls")]
    #[validate(nested)]
    pub services: Services,
}
