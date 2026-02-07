use serde::{Deserialize, Serialize};

use crate::primitive_defaults::{r#false, r#true};

/// Customizes the assembler's behavior.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AssemblerOptions {
    /// Enable run-time modification of text segments.
    #[serde(default = "r#false")]
    pub self_modifying_code: bool,

    /// Simulate the delay slot.
    #[serde(default = "r#false")]
    pub delay_slot: bool,

    /// Allow `sbrk` to free memory when given a negative argument.
    #[serde(default = "r#false")]
    pub freeable_heap_allocations: bool,

    /// Allow use of pseudo-instructions and formats.
    #[serde(default = "r#true")]
    pub pseudo_instructions: bool,
}
