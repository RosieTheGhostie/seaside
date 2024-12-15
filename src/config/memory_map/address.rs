use serde::{Deserialize, Serialize};

pub type Address = u32;

#[derive(Deserialize, Serialize)]
pub struct AddressRange {
    pub base: Address,
    pub limit: Address,
}

impl AddressRange {
    pub fn overlapping(&self, other: &Self) -> bool {
        self.limit >= other.base
    }
}

pub const fn is_aligned(address: Address, n_bytes: u32) -> bool {
    address.trailing_zeros() >= n_bytes
}
