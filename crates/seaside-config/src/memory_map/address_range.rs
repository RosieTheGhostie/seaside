use super::{Contains, Overlapping};
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
