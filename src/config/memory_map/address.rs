use super::{Contains, Overlapping};
use serde::{Deserialize, Serialize};

pub type Address = u32;

#[derive(Deserialize, Serialize)]
pub struct AddressRange {
    pub base: Address,
    pub limit: Address,
}

impl Overlapping<AddressRange> for AddressRange {
    fn overlapping(&self, other: &Self) -> bool {
        self.limit >= other.base
    }
}

impl Contains<Address> for AddressRange {
    fn contains(&self, address: &Address) -> bool {
        self.base <= *address && *address <= self.limit
    }
}

impl Contains<AddressRange> for AddressRange {
    fn contains(&self, other: &AddressRange) -> bool {
        self.base <= other.base && other.limit <= self.limit
    }
}

pub const fn is_aligned(address: Address, n_bytes: u32) -> bool {
    address.trailing_zeros() >= n_bytes
}
