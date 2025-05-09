pub mod assembler;
pub mod services;

pub use assembler::AssemblerOptions;
pub use services::{Service, Services};

use crate::{Validate, primitive_defaults};
use anyhow::Result;
use seaside_int_utils::AllZeroes;
use serde::{Deserialize, Serialize};

/// Customizes the features available to the seaside engine.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Features {
    /// Allow users to provide code and/or data relating to kernel space.
    #[serde(default = "primitive_defaults::r#true")]
    pub kernel_space_accessible: bool,
    /// Enable run-time modification of text segments.
    #[serde(default = "primitive_defaults::r#false")]
    pub self_modifying_code: bool,
    /// Simulate the delay slot.
    #[serde(default = "primitive_defaults::r#false")]
    pub delay_slot: bool,
    /// Allow `sbrk` to free memory when given a negative argument.
    #[serde(default = "primitive_defaults::r#false")]
    pub freeable_heap_allocations: bool,
    /// Enables displaying a crash handler when an unhandled exception is thrown.
    #[serde(default = "primitive_defaults::r#true")]
    pub show_crash_handler: bool,
    /// Set features available to assembler.
    pub assembler: AssemblerOptions,
    /// Set system services available to interpreter.
    #[serde(alias = "syscalls")]
    pub services: Services,
}

impl Validate for Features {
    fn validate(&self) -> Result<()> {
        self.services.validate()
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
            assembler: AssemblerOptions::all_zeroes(),
            services: Services::all_zeroes(),
        }
    }
}
