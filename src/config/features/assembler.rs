use super::super::{
    bitflags_addons::{impl_deserialize, impl_serialize},
    presets::{impl_bitflags_has_basic_presets, maybe_using_preset},
    primitive_defaults::r#true,
};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// Customizes the assembler's behavior.
#[derive(Serialize, Deserialize)]
pub struct AssemblerOptions {
    /// Allow use of pseudo-instructions and formats.
    #[serde(default = "r#true")]
    pub pseudo_instructions: bool,
    /// Which special directives to allow.
    #[serde(deserialize_with = "maybe_using_preset")]
    pub directives: SpecialDirectives,
}

bitflags! {
    /// Nontrivial assembler directives that can be enabled or disabled.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl_bitflags_has_basic_presets!(
    SpecialDirectives,
    Self::Asciiz | Self::Global | Self::Include
);
