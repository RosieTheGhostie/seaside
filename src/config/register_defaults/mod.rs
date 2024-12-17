pub mod coprocessor_0;
pub mod floating_point;
pub mod general_purpose;
pub mod register_set;
pub mod registers;

pub use coprocessor_0::Coprocessor0Register;
pub use floating_point::FloatingPointRegister;
pub use general_purpose::GeneralPurposeRegister;
pub use register_set::RegisterSet;
pub use registers::Registers;

use num_traits::{ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Default, Deserialize, Serialize)]
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
        with = "floating_point::registers_format",
        default = "Registers::<32>::default",
        skip_serializing_if = "Registers::<32>::is_default"
    )]
    pub floating_point: Registers<32>,
    #[serde(
        with = "coprocessor_0::registers_format",
        default = "Registers::<4>::default",
        skip_serializing_if = "Registers::<4>::is_default"
    )]
    pub coprocessor_0: Registers<4>,
}

impl Index<GeneralPurposeRegister> for RegisterDefaults {
    type Output = u32;

    fn index(&self, index: GeneralPurposeRegister) -> &Self::Output {
        &self.general_purpose[index.to_usize().unwrap()]
    }
}

impl Index<FloatingPointRegister> for RegisterDefaults {
    type Output = u32;

    fn index(&self, index: FloatingPointRegister) -> &Self::Output {
        &self.floating_point[index.to_usize().unwrap()]
    }
}

impl Index<Coprocessor0Register> for RegisterDefaults {
    type Output = u32;

    fn index(&self, index: Coprocessor0Register) -> &Self::Output {
        &self.coprocessor_0[index.to_usize().unwrap()]
    }
}
