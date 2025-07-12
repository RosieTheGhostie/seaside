use crate::primitive_defaults::r#true;
use seaside_int_utils::AllZeroes;
use serde::{Deserialize, Serialize};

/// Customizes the assembler's behavior.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
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
