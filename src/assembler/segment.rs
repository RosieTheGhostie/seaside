use super::error::Error;
use crate::{config::Endian, type_aliases::address::Address};
use std::{fs::write, path::PathBuf};

pub struct SegmentBuildInfo {
    pub base: Address,
    pub next: Address,
    bytes: Vec<u8>,
}

impl SegmentBuildInfo {
    pub fn new(base: Address) -> Self {
        Self {
            base,
            next: base,
            bytes: vec![],
        }
    }

    pub fn export(&self, path: PathBuf) -> Result<(), Error> {
        write(path, &self.bytes).map_err(Error::Io)
    }

    pub fn jump_ahead_to(&mut self, address: Address) -> Result<(), Error> {
        match address.checked_sub(self.next) {
            Some(n) => {
                self.jump_ahead_by(n);
                Ok(())
            }
            None => Err(Error::JumpBehind),
        }
    }

    pub fn jump_ahead_by(&mut self, n: u32) {
        self.next += n;
        let mut nuls = vec![0u8; n as usize];
        self.bytes.append(&mut nuls);
    }

    pub fn append(&mut self, bytes: &mut Vec<u8>) {
        self.next += bytes.len() as Address;
        self.bytes.append(bytes);
    }

    pub fn append_i8(&mut self, bytes: Vec<i8>) {
        let n_bytes = bytes.len();
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for byte in bytes {
            self.bytes.push(byte as u8);
        }
    }

    pub fn append_i16(&mut self, halves: Vec<i16>, endian: Endian) {
        let n_bytes = halves.len() << 1;
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for half in halves {
            let bytes = match endian {
                Endian::Little => half.to_le_bytes(),
                Endian::Big => half.to_be_bytes(),
            };
            for byte in bytes {
                self.bytes.push(byte);
            }
        }
    }

    pub fn append_i32(&mut self, words: Vec<i32>, endian: Endian) {
        let n_bytes = words.len() << 2;
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for word in words {
            let bytes = match endian {
                Endian::Little => word.to_le_bytes(),
                Endian::Big => word.to_be_bytes(),
            };
            for byte in bytes {
                self.bytes.push(byte);
            }
        }
    }

    pub fn append_f32(&mut self, floats: Vec<f32>, endian: Endian) {
        let n_bytes = floats.len() << 2;
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for float in floats {
            let bytes = match endian {
                Endian::Little => float.to_le_bytes(),
                Endian::Big => float.to_be_bytes(),
            };
            for byte in bytes {
                self.bytes.push(byte);
            }
        }
    }

    pub fn append_f64(&mut self, doubles: Vec<f64>, endian: Endian) {
        let n_bytes = doubles.len() << 3;
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for double in doubles {
            let bytes = match endian {
                Endian::Little => double.to_le_bytes(),
                Endian::Big => double.to_be_bytes(),
            };
            for byte in bytes {
                self.bytes.push(byte);
            }
        }
    }

    pub fn overwrite_u32(
        &mut self,
        address: Address,
        word: u32,
        endian: Endian,
    ) -> Result<(), Error> {
        let index = address
            .checked_sub(self.base)
            .ok_or(Error::InternalLogicIssue)? as usize;
        let old_bytes = self
            .bytes
            .get_mut(index..index + 4)
            .ok_or(Error::InternalLogicIssue)?;
        let new_bytes = match endian {
            Endian::Little => word.to_le_bytes(),
            Endian::Big => word.to_be_bytes(),
        };
        old_bytes[..4].copy_from_slice(&new_bytes);
        Ok(())
    }

    pub fn align(&mut self, alignment: u8) {
        if alignment == 0 {
            return;
        }
        let divisor = (1 << alignment) as usize;
        let modulus = self.next as usize & (divisor - 1);
        if modulus != 0 {
            self.append_i8(vec![0; divisor - modulus]);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
