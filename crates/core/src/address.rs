use crate::size::Size;

/// A memory address.
pub type Address = u32;

/// Checks if `address` is aligned to `n_bytes`.
///
/// `n_bytes` should be a power of two.
///
/// # Panics
///
/// This function will panic if `n_bytes` is zero.
///
/// # Examples
///
/// ```
/// # use seaside_core::address::is_aligned;
/// // 0x12121212 == 00010010 00010010 00010010 00010010
/// assert!(is_aligned(0x12121212, 2));
/// assert!(!is_aligned(0x12121212, 4));
///
/// // 0x12345678 == 00010010 00110100 01010110 01111000
/// assert!(is_aligned(0x12345678, 4));
/// assert!(is_aligned(0x12345678, 8));
/// assert!(!is_aligned(0x12345678, 16));
/// ```
pub const fn is_aligned(address: Address, n_bytes: Size) -> bool {
    address.trailing_zeros() >= n_bytes.ilog2()
}
