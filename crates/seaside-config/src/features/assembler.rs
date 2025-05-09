use crate::primitive_defaults::r#true;
use bitflags::bitflags;
use seaside_bitflags_serde::{impl_deserialize, impl_serialize};
use seaside_int_utils::AllZeroes;
use serde::{Deserialize, Serialize};

/// Customizes the assembler's behavior.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AssemblerOptions {
    /// Allow use of pseudo-instructions and formats.
    #[serde(default = "r#true")]
    pub pseudo_instructions: bool,
    /// Which special directives to allow.
    pub directives: SpecialDirectives,
}

impl AllZeroes for AssemblerOptions {
    fn all_zeroes() -> Self {
        Self {
            pseudo_instructions: false,
            directives: SpecialDirectives::empty(),
        }
    }
}

bitflags! {
    /// Nontrivial assembler directives that can be enabled or disabled.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct SpecialDirectives: u8 {
        /// `.asciiz`
        const Asciiz = 0x1;
        /// `.eqv`
        const Eqv = 0x2;
        /// `.global` and `.globl`
        const Global = 0x4;
        /// `.include`
        const Include = 0x8;
        /// `.macro` and `.end_macro`
        const Macros = 0x10;
        /// `.set`
        const Set = 0x20;
    }
}

impl_serialize!(SpecialDirectives);
impl_deserialize!(SpecialDirectives);
