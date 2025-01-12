#![allow(dead_code)]
use crate::config::Endian;
use std::{marker::PhantomData, slice::ChunksExact};

pub struct ByteStream<'a, T> {
    marker: PhantomData<fn() -> T>,
    chunks: ChunksExact<'a, u8>,
    endian: Endian,
}

impl<'a> ByteStream<'a, u16> {
    pub fn new(bytes: &'a [u8], endian: Endian) -> Self {
        Self {
            marker: PhantomData,
            chunks: bytes.chunks_exact(2),
            endian,
        }
    }
}

impl<'a> ByteStream<'a, u32> {
    pub fn new(bytes: &'a [u8], endian: Endian) -> Self {
        Self {
            marker: PhantomData,
            chunks: bytes.chunks_exact(4),
            endian,
        }
    }
}

impl<'a> ByteStream<'a, u64> {
    pub fn new(bytes: &'a [u8], endian: Endian) -> Self {
        Self {
            marker: PhantomData,
            chunks: bytes.chunks_exact(8),
            endian,
        }
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
