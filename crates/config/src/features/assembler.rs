use seaside_int_utils::AllZeroes;
use serde::{Deserialize, Serialize};

use crate::primitive_defaults::r#true;

/// Customizes the assembler's behavior.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AssemblerOptions {
    /// Allow use of pseudo-instructions and formats.
    #[serde(default = "r#true")]
    pub pseudo_instructions: bool,
}

impl AllZeroes for AssemblerOptions {
    fn all_zeroes() -> Self {
        Self {
            pseudo_instructions: false,
        }
    }
}
