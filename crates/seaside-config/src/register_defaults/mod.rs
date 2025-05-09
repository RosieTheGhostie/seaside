pub mod register_set;
pub mod registers;

mod coprocessor_0;
mod floating_point;
mod general_purpose;

pub use register_set::RegisterSet;
pub use registers::Registers;

use num_traits::Zero;
use seaside_constants::register;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RegisterDefaults {
    #[serde(default = "u32::default", skip_serializing_if = "u32::is_zero")]
    pub hi: u32,
    #[serde(default = "u32::default", skip_serializing_if = "u32::is_zero")]
    pub lo: u32,
    #[serde(
        with = "general_purpose::registers_format",
        default = "Registers::<32>::default",
        skip_serializing_if = "Registers::<32>::is_default"
    )]
    pub general_purpose: Registers<32>,
    #[serde(
        with = "coprocessor_0::registers_format",
        default = "Registers::<4>::default",
        skip_serializing_if = "Registers::<4>::is_default"
    )]
    pub coprocessor_0: Registers<4>,
    #[serde(
        with = "floating_point::registers_format",
        default = "Registers::<32>::default",
        skip_serializing_if = "Registers::<32>::is_default"
    )]
    pub coprocessor_1: Registers<32>,
}

impl Default for RegisterDefaults {
    fn default() -> Self {
        let mut general_purpose = Registers::<32>::default();
        general_purpose[register::GP as usize] = 0x10008000;
        general_purpose[register::SP as usize] = 0x7fffeffc; // check if this makes sense
        let mut coprocessor_0 = Registers::<4>::default();
        coprocessor_0[1] = 0x0000ff11; // `status` lives at index 1

        // I was going to use the `..Default::default()` syntax, but clippy said that'd would cause
        // an infinite recursion for some reason :L
        Self {
            hi: Default::default(),
            lo: Default::default(),
            general_purpose,
            coprocessor_0,
            coprocessor_1: Default::default(),
        }
    }
}
