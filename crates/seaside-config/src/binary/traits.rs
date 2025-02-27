use anyhow::Result;
use std::io::{Read, Write};

pub trait FromBinary: Sized {
    fn from_binary<R: Read>(stream: &mut R) -> Result<Self>;
}

pub trait EditFromBinary<const VERSION: u32>: Sized {
    fn edit_from_binary<R: Read>(&mut self, ids: [u8; 4], stream: &mut R) -> Result<()>;
}

pub trait ToBinary<const VERSION: u32> {
    fn to_binary<W: Write>(&self, stream: &mut W) -> Result<()>;
}
