use super::{
    coprocessor_0::{deserialize_coprocessor_0_registers, serialize_coprocessor_0_registers},
    floating_point::{deserialize_floating_point_registers, serialize_floating_point_registers},
    general_purpose::{deserialize_general_purpose_registers, serialize_general_purpose_registers},
    Coprocessor0Register, FloatingPointRegister, GeneralPurposeRegister,
    _registers::Registers,
};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Default, Deserialize, Serialize)]
pub struct RegisterDefaults {
    #[serde(
        serialize_with = "serialize_general_purpose_registers",
        deserialize_with = "deserialize_general_purpose_registers",
        default = "Registers::<34>::default",
        skip_serializing_if = "Registers::<34>::is_default"
    )]
    pub general_purpose: Registers<34>,
    #[serde(
        serialize_with = "serialize_floating_point_registers",
        deserialize_with = "deserialize_floating_point_registers",
        default = "Registers::<32>::default",
        skip_serializing_if = "Registers::<32>::is_default"
    )]
    pub floating_point: Registers<32>,
    #[serde(
        serialize_with = "serialize_coprocessor_0_registers",
        deserialize_with = "deserialize_coprocessor_0_registers",
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
