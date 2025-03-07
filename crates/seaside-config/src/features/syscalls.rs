use crate::{prefix, service_id, EditFromBinary, ToBinary, Validate};
use anyhow::{anyhow, Error, Result};
use seaside_error::EngineError;
use seaside_int_utils::AllZeroes;
use std::{
    collections::{hash_map::Iter, HashMap},
    io::{Read, Write},
};

/// Controls which system services are available to the seaside engine.
///
/// If a program requests a disabled service, a `SyscallFailure` exception will be raised.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Syscalls(HashMap<u16, u32>);

impl Validate for Syscalls {
    fn validate(&self) -> Result<()> {
        if self.0.contains_key(&service_id!(mars_system[EXIT]))
            || self.0.contains_key(&service_id!(mars_system[EXIT_2]))
        {
            Ok(())
        } else {
            Err(Error::new(EngineError::InvalidConfig).context("missing a service to exit program"))
        }
    }
}

impl AllZeroes for Syscalls {
    fn all_zeroes() -> Self {
        Self(HashMap::new())
    }
}

impl EditFromBinary<1> for Syscalls {
    fn edit_from_binary<R: Read>(&mut self, ids: [u8; 4], stream: &mut R) -> Result<()> {
        let id = u16::from_be_bytes([ids[2], ids[3]]);
        let service_code: u32 = {
            let mut buffer = [0u8; 4];
            stream.read_exact(&mut buffer)?;
            u32::from_le_bytes(buffer)
        };
        match self.0.insert(id, service_code) {
            Some(old_code) => Err(anyhow!(
                "overwrote service code #{old_code} with #{service_code} (syscall_id: {id})"
            )),
            None => Ok(()),
        }
    }
}

impl ToBinary<1> for Syscalls {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        let mut id_buffer = prefix!(features::syscalls).to_le_bytes();
        for (&id, &service_code) in self.iter() {
            [id_buffer[0], id_buffer[1]] = id.to_le_bytes();
            stream.write(&id_buffer)?;
            service_code.to_binary(stream)?;
        }
        Ok(())
    }
}

impl Syscalls {
    pub fn iter(&self) -> Iter<'_, u16, u32> {
        self.0.iter()
    }
}
