use super::{Contains, Overlapping};
use seaside_int_utils::AllZeroes;
use seaside_type_aliases::Address;
use serde::{Deserialize, Serialize};

/// An inclusive range of [`Address`]es.
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

impl AllZeroes for AddressRange {
    fn all_zeroes() -> Self {
        Self {
            base: 0x00000000,
            limit: 0x00000000,
        }
    }
}
