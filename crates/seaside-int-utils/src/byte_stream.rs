//! Reading bytes as bigger integers made easy.
//!
//! Provides the iterator [`ByteStream`], which supports all built-in integer types.

use crate::endian::Endian;
use std::{marker::PhantomData, slice::ChunksExact};

/// An iterator that iterates over a slice of bytes as another integer type.
pub struct ByteStream<'a, T> {
    marker: PhantomData<fn() -> T>,
    chunks: ChunksExact<'a, u8>,
    endian: Endian,
}

macro_rules! impl_new_byte_stream {
    ($t:ty, size: $size:literal) => {
        impl<'a> ByteStream<'a, $t> {
            pub fn new(bytes: &'a [u8], endian: Endian) -> Self {
                Self {
                    marker: PhantomData,
                    chunks: bytes.chunks_exact($size),
                    endian,
                }
            }
        }
    };
}

impl_new_byte_stream!(i8, size: 1);
impl_new_byte_stream!(u8, size: 1);
impl_new_byte_stream!(i16, size: 2);
impl_new_byte_stream!(u16, size: 2);
impl_new_byte_stream!(i32, size: 4);
impl_new_byte_stream!(u32, size: 4);
impl_new_byte_stream!(i64, size: 8);
impl_new_byte_stream!(u64, size: 8);
impl_new_byte_stream!(i128, size: 16);
impl_new_byte_stream!(u128, size: 16);

impl Iterator for ByteStream<'_, i8> {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self.chunks.next()?.first()? as i8)
    }
}

impl Iterator for ByteStream<'_, u8> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self.chunks.next()?.first()?)
    }
}

impl Iterator for ByteStream<'_, i16> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunks.next()?;
        let bytes = [chunk[0], chunk[1]];
        Some(match self.endian {
            Endian::Little => i16::from_le_bytes(bytes),
            Endian::Big => i16::from_be_bytes(bytes),
        })
    }
}

impl Iterator for ByteStream<'_, u16> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunks.next()?;
        let bytes = [chunk[0], chunk[1]];
        Some(match self.endian {
            Endian::Little => u16::from_le_bytes(bytes),
            Endian::Big => u16::from_be_bytes(bytes),
        })
    }
}

impl Iterator for ByteStream<'_, i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunks.next()?;
        let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
        Some(match self.endian {
            Endian::Little => i32::from_le_bytes(bytes),
            Endian::Big => i32::from_be_bytes(bytes),
        })
    }
}

impl Iterator for ByteStream<'_, u32> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunks.next()?;
        let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
        Some(match self.endian {
            Endian::Little => u32::from_le_bytes(bytes),
            Endian::Big => u32::from_be_bytes(bytes),
        })
    }
}

impl Iterator for ByteStream<'_, i64> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunks.next()?;
        let bytes = [
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
        ];
        Some(match self.endian {
            Endian::Little => i64::from_le_bytes(bytes),
            Endian::Big => i64::from_be_bytes(bytes),
        })
    }
}

impl Iterator for ByteStream<'_, u64> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunks.next()?;
        let bytes = [
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
        ];
        Some(match self.endian {
            Endian::Little => u64::from_le_bytes(bytes),
            Endian::Big => u64::from_be_bytes(bytes),
        })
    }
}

impl Iterator for ByteStream<'_, i128> {
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunks.next()?;
        let bytes = [
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15],
        ];
        Some(match self.endian {
            Endian::Little => i128::from_le_bytes(bytes),
            Endian::Big => i128::from_be_bytes(bytes),
        })
    }
}

impl Iterator for ByteStream<'_, u128> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunks.next()?;
        let bytes = [
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15],
        ];
        Some(match self.endian {
            Endian::Little => u128::from_le_bytes(bytes),
            Endian::Big => u128::from_be_bytes(bytes),
        })
    }
}
