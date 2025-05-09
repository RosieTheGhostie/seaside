use super::traits::{Contains, Overlapping};
use seaside_int_utils::AllZeroes;
use seaside_type_aliases::Address;
use serde::{Deserialize, Serialize};

/// An inclusive range of [`Address`]es.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AddressRange {
    pub base: Address,
    pub limit: Address,
}

impl Serialize for AddressRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        [self.base, self.limit].serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AddressRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let [base, limit] = <[u32; 2]>::deserialize(deserializer)?;
        Ok(Self { base, limit })
    }
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

macro_rules! address_range {
    [$base:literal..$limit:literal] => {
        $crate::memory_map::AddressRange { base: $base, limit: $limit + 1 }
    };
    [$base:literal..=$limit:literal] => {
        $crate::memory_map::AddressRange { base: $base, limit: $limit }
    };
}
pub(crate) use address_range;
