pub use data::DataRegion;
pub use region::Region;
pub use text::TextRegion;

mod data;
mod region;
mod text;

/// Heap-allocates a byte array with the given length, filling the whole thing with zeroes.
fn allocate_zeroed_byte_array(len: usize) -> Box<[u8]> {
    unsafe { Box::new_zeroed_slice(len).assume_init() }
}

/// Heap-allocates a word array with the given length, filling the whole thing with zeroes.
///
/// # Notes
///
/// - `len` is the length in **words**, not bytes.
fn allocate_zeroed_word_array(len: usize) -> Box<[u32]> {
    unsafe { Box::new_zeroed_slice(len).assume_init() }
}
