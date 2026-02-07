pub use data::DataRegion;
pub use region::{ReadableRegion, Region, SliceableRegion, SliceableRegionMut, WriteableRegion};
pub use text::TextRegion;

mod data;
mod inner;
mod region;
mod text;

pub(self) use inner::Inner;

/// Heap-allocates a byte array with the given length, filling the whole thing with zeroes.
pub(self) fn allocate_zeroed_byte_array(len: usize) -> Box<[u8]> {
    unsafe { Box::new_zeroed_slice(len).assume_init() }
}
