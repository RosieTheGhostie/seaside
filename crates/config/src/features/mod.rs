pub mod assembler;

pub use assembler::AssemblerOptions;

use seaside_constants::Services;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::primitive_defaults::r#true;

/// Customizes the features available to the seaside engine.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Features {
    /// Enables displaying a crash handler when an unhandled exception is thrown.
    #[serde(default = "r#true")]
    pub show_crash_handler: bool,

    /// Set features available to assembler.
    #[serde(default)]
    pub assembler: AssemblerOptions,

    /// Set system services available to interpreter.
    #[serde(alias = "syscalls")]
    #[validate(nested)]
    pub services: Services,
}
