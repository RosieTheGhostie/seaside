#[cfg(feature = "sized")]
pub mod sized;
pub mod traits;

use seaside_type_aliases::Address;

use traits::{Contains, Overlapping};

/// An inclusive range of [`Address`]es.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AddressRange {
    /// The inclusive minimum address in the range.
    base: Address,

    /// The inclusive maximum address in the range.
    limit: Address,
}

impl AddressRange {
    pub const FULL: Self = unsafe { Self::new_unchecked(Address::MIN, Address::MAX) };

    pub const fn new(base: Address, limit: Address) -> Self {
        Self::new_checked(base, limit)
            .expect("limit address must be greater than or equal to the base address")
    }

    pub const fn new_checked(base: Address, limit: Address) -> Option<Self> {
        if limit >= base {
            Some(unsafe { Self::new_unchecked(base, limit) })
        } else {
            None
        }
    }

    pub const unsafe fn new_unchecked(base: Address, limit: Address) -> Self {
        Self { base, limit }
    }

    pub const fn base(&self) -> Address {
        self.base
    }

    pub const fn limit(&self) -> Address {
        self.limit
    }

    pub const fn contains_address(&self, address: Address) -> bool {
        self.base <= address && address <= self.limit
    }

    pub const fn split(self, at: Address) -> (Self, Self) {
        self.split_checked(at)
            .expect("address range must contain the split address")
    }

    pub const fn split_checked(self, at: Address) -> Option<(Self, Self)> {
        if self.contains_address(at) {
            Some(unsafe { self.split_unchecked(at) })
        } else {
            None
        }
    }

    pub const unsafe fn split_unchecked(self, at: Address) -> (Self, Self) {
        (unsafe { Self::new_unchecked(self.base, at - 1) }, unsafe {
            Self::new_unchecked(at, self.limit)
        })
    }
}

impl Overlapping<AddressRange> for AddressRange {
    fn overlapping(&self, other: &Self) -> bool {
        self.limit >= other.base
    }
}

impl Contains<Address> for AddressRange {
    fn contains(&self, address: &Address) -> bool {
        self.contains_address(*address)
    }
}

impl Contains<AddressRange> for AddressRange {
    fn contains(&self, other: &AddressRange) -> bool {
        self.base <= other.base && other.limit <= self.limit
    }
}

#[cfg(feature = "seaside-int-utils")]
impl seaside_int_utils::AllZeroes for AddressRange {
    fn all_zeroes() -> Self {
        unsafe { Self::new_unchecked(0, 0) }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AddressRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        [self.base, self.limit].serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AddressRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let [base, limit] = <[Address; 2]>::deserialize(deserializer)?;
        Ok(Self { base, limit })
    }
}

#[macro_export]
macro_rules! address_range {
    [$base:literal..$limit:literal] => {
        $crate::AddressRange::new($base, $limit - 1)
    };
    [$base:literal..=$limit:literal] => {
        $crate::AddressRange::new($base, $limit)
    };
}
