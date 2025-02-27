pub mod traits;

pub use traits::{EditFromBinary, FromBinary, ToBinary};

use anyhow::{anyhow, Result};
use seaside_int_utils::Endian;
use semver::Version;
use std::io::{Read, Write};

impl FromBinary for bool {
    fn from_binary<R: Read>(stream: &mut R) -> Result<Self> {
        let mut buffer = [0u8];

        stream.read_exact(&mut buffer)?;
        Ok(buffer[0] != 0)
    }
}

impl ToBinary<1> for bool {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        <u8 as ToBinary<1>>::to_binary(&(*self as u8), stream)
    }
}

impl FromBinary for u8 {
    fn from_binary<R: Read>(stream: &mut R) -> Result<Self> {
        let mut buffer = [0u8];

        stream.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }
}

impl ToBinary<1> for u8 {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        stream.write(&[*self])?;
        Ok(())
    }
}

impl FromBinary for u16 {
    fn from_binary<R: Read>(stream: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 2];

        stream.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes([buffer[0], buffer[1]]))
    }
}

impl ToBinary<1> for u16 {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        stream.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl FromBinary for u32 {
    fn from_binary<R: Read>(stream: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 4];

        stream.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
        ]))
    }
}

impl ToBinary<1> for u32 {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        stream.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl FromBinary for u64 {
    fn from_binary<R: Read>(stream: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 8];

        stream.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7],
        ]))
    }
}

impl ToBinary<1> for u64 {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        stream.write(&self.to_le_bytes())?;
        Ok(())
    }
}

impl FromBinary for Endian {
    fn from_binary<R: Read>(stream: &mut R) -> Result<Self> {
        let mut buffer = [0u8];

        stream.read_exact(&mut buffer)?;
        match buffer[0] {
            0 => Ok(Self::Little),
            1 => Ok(Self::Big),
            byte => Err(anyhow!("{byte} does not correspond to a valid byte order")),
        }
    }
}

impl ToBinary<1> for Endian {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        // I decided to be a tad more verbose in case I were to add another Endian variant.
        // That's unlikely given the context, but it's more robust, so whatever.
        stream.write(&[match self {
            Self::Little => 0,
            Self::Big => 1,
        }])?;
        Ok(())
    }
}

impl FromBinary for Version {
    fn from_binary<R: Read>(stream: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 4];

        stream.read_exact(&mut buffer)?;
        let patch = u16::from_le_bytes([buffer[0], buffer[1]]);
        let (minor, major) = (buffer[2], buffer[3]);

        Ok(Self::new(major as u64, minor as u64, patch as u64))
    }
}

impl ToBinary<1> for Version {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()> {
        let major: u8 = self.major.try_into()?;
        let minor: u8 = self.minor.try_into()?;
        let patch: [u8; 2] = {
            let patch: u16 = self.patch.try_into()?;
            patch.to_le_bytes()
        };

        stream.write(&[patch[0], patch[1], minor, major])?;
        Ok(())
    }
}
