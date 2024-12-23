use super::super::{
    bitflags_addons::{impl_deserialize, impl_serialize},
    presets::{impl_bitflags_has_basic_presets, maybe_using_preset},
    red_flag_behavior::RedFlagBehavior,
};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// Customizes the assembler's behavior.
#[derive(Serialize, Deserialize)]
pub struct AssemblerOptions {
    /// Allow use of pseudo-instructions and formats.
    pub pseudo_instructions: bool,
    /// What to do when the code references register `$at`.
    pub on_spot_register_at: RedFlagBehavior,
    /// Which special directives to allow.
    #[serde(deserialize_with = "maybe_using_preset")]
    pub directives: SpecialDirectives,
}

bitflags! {
    /// Nontrivial assembler directives that can be enabled or disabled.
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
