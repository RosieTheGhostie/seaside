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

use crate::{EditFromBinary, ToBinary};
use anyhow::{anyhow, Result};
use num_traits::ToPrimitive;
use std::{
    io::{Read, Write},
    ops::Index,
};

#[derive(Default)]
pub struct RegisterDefaults {
    pub hi: u32,
    pub lo: u32,
    pub general_purpose: Registers<32>,
    pub floating_point: Registers<32>,
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

impl EditFromBinary<1> for RegisterDefaults {
    fn edit_from_binary<R: Read>(&mut self, ids: [u8; 4], stream: &mut R) -> Result<()> {
        use crate::properties::register_defaults::*;

        let value: u32 = {
            let mut buffer = [0u8; 4];
            stream.read_exact(&mut buffer)?;
            u32::from_le_bytes(buffer)
        };
        match (ids[1], ids[3]) {
            (0x00, HI) => self.hi = value,
            (0x00, LO) => self.hi = value,
            (general_purpose::ID, i) if i < 32 => self.general_purpose[i as usize] = value,
            (coprocessor_0::ID, coprocessor_0::VADDR) => self.coprocessor_0[0] = value,
            (coprocessor_0::ID, coprocessor_0::STATUS) => self.coprocessor_0[1] = value,
            (coprocessor_0::ID, coprocessor_0::CAUSE) => self.coprocessor_0[2] = value,
            (coprocessor_0::ID, coprocessor_0::EPC) => self.coprocessor_0[3] = value,
            (coprocessor_1::ID, i) if i < 32 => self.floating_point[i as usize] = value,
            _ => return Err(anyhow!("unknown property id: {}", u32::from_be_bytes(ids))),
        }
        Ok(())
    }
}

impl ToBinary<1> for RegisterDefaults {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        use crate::properties::prefixed::register_defaults::*;

        let mut id_buffer = general_purpose::PREFIX.to_le_bytes();
        for i in 1..32 {
            id_buffer[0] = i as u8;
            stream.write(&id_buffer)?;
            self.general_purpose[i].to_binary(stream)?;
        }

        coprocessor_0::VADDR.to_binary(stream)?;
        self.coprocessor_0[0].to_binary(stream)?;
        coprocessor_0::STATUS.to_binary(stream)?;
        self.coprocessor_0[1].to_binary(stream)?;
        coprocessor_0::CAUSE.to_binary(stream)?;
        self.coprocessor_0[2].to_binary(stream)?;
        coprocessor_0::EPC.to_binary(stream)?;
        self.coprocessor_0[3].to_binary(stream)?;

        id_buffer = coprocessor_1::PREFIX.to_le_bytes();
        for i in 0..32 {
            id_buffer[0] = i as u8;
            stream.write(&id_buffer)?;
            self.floating_point[i].to_binary(stream)?;
        }

        Ok(())
    }
}
