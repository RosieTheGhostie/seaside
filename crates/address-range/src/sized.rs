use seaside_type_aliases::{Address, Size};

use crate::traits::{Contains, Overlapping};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SizedAddressRange<const INVERTED: bool = false> {
    /// The address considered to be the base of this range.
    ///
    /// If `INVERTED` is `true`, this will be the inclusive maximum. Otherwise, it will be the
    /// inclusive minimum.
    pub base: Address,

    /// The number of bytes within the range.
    pub size: Size,
}

impl<const INVERTED: bool> SizedAddressRange<INVERTED> {
    pub const fn new(base: Address, size: Address) -> Self {
        Self { base, size }
    }
}

impl SizedAddressRange<false> {
    pub const fn inclusive_limit(&self) -> Option<Address> {
        match self.size.checked_sub(1) {
            Some(size_minus_1) => self.base.checked_add(size_minus_1),
            None => None,
        }
    }

    pub const fn exclusive_limit(&self) -> Option<Address> {
        self.base.checked_add(self.size)
    }

    pub const fn contains_address(&self, address: Address) -> bool {
        if let Some(distance_from_base) = self.base.checked_sub(address) {
            distance_from_base < self.size
        } else {
            false
        }
    }
}

impl Overlapping<SizedAddressRange<false>> for SizedAddressRange<false> {
    fn overlapping(&self, other: &Self) -> bool {
        // This could certainly be optimized, but I'm more interested in getting other things done.
        self.contains(&other.base) || other.contains(&self.base)
    }
}

impl Contains<Address> for SizedAddressRange<false> {
    fn contains(&self, address: &Address) -> bool {
        self.contains_address(*address)
    }
}

impl Contains<SizedAddressRange<false>> for SizedAddressRange<false> {
    fn contains(&self, other: &SizedAddressRange<false>) -> bool {
        if self.base > other.base {
            false
        } else if let Some(limit_a) = self.exclusive_limit() {
            other
                .exclusive_limit()
                .is_some_and(|limit_b| limit_b <= limit_a)
        } else {
            true
        }
    }
}

#[cfg(feature = "seaside-int-utils")]
impl<const INVERTED: bool> seaside_int_utils::AllZeroes for SizedAddressRange<INVERTED> {
    fn all_zeroes() -> Self {
        Self::new(0, 0)
    }
}
