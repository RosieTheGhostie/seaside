pub mod register_set;
pub mod registers;

pub use register_set::RegisterSet;
pub use registers::Registers;

use crate::{prefix, prefixed, EditFromBinary, ToBinary};
use anyhow::{anyhow, Result};
use seaside_constants::register;
use std::io::{Read, Write};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RegisterDefaults {
    pub hi: u32,
    pub lo: u32,
    pub general_purpose: Registers<32>,
    pub coprocessor_0: Registers<4>,
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
            (coprocessor_1::ID, i) if i < 32 => self.coprocessor_1[i as usize] = value,
            _ => return Err(anyhow!("unknown property id: {}", u32::from_be_bytes(ids))),
        }
        Ok(())
    }
}

impl ToBinary<1> for RegisterDefaults {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        let mut id_buffer = prefix!(register_defaults::general_purpose).to_le_bytes();
        for i in 1..32 {
            id_buffer[0] = i as u8;
            stream.write(&id_buffer)?;
            self.general_purpose[i].to_binary(stream)?;
        }

        prefixed!(register_defaults::coprocessor_0[VADDR]).to_binary(stream)?;
        self.coprocessor_0[0].to_binary(stream)?;
        prefixed!(register_defaults::coprocessor_0[STATUS]).to_binary(stream)?;
        self.coprocessor_0[1].to_binary(stream)?;
        prefixed!(register_defaults::coprocessor_0[CAUSE]).to_binary(stream)?;
        self.coprocessor_0[2].to_binary(stream)?;
        prefixed!(register_defaults::coprocessor_0[EPC]).to_binary(stream)?;
        self.coprocessor_0[3].to_binary(stream)?;

        id_buffer = prefix!(register_defaults::coprocessor_1).to_le_bytes();
        for i in 0..32 {
            id_buffer[0] = i as u8;
            stream.write(&id_buffer)?;
            self.coprocessor_1[i].to_binary(stream)?;
        }

        Ok(())
    }
}
