use serde::{Deserialize, Serialize};

pub type Address = u32;

#[derive(Serialize, Deserialize)]
pub struct AddressRange {
    pub base: Address,
    pub limit: Option<Address>,
}

pub const fn is_aligned(address: Address, n_bytes: u32) -> bool {
    address.trailing_zeros() >= n_bytes
}
