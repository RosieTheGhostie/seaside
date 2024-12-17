pub type Address = u32;

pub const fn is_aligned(address: Address, n_bytes: u32) -> bool {
    address.trailing_zeros() >= n_bytes
}
