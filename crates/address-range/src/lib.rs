//! Specialized range types relating to memory addresses for use in [seaside].
//!
//! Provides two different kinds of memory address ranges:
//! - [`AddressRange`] (analogous to [`RangeInclusive<Address>`](core::ops::RangeInclusive))
//! - [`SizedAddressRange`](sized::SizedAddressRange) (see docs for more details)
//!
//! [seaside]: https://github.com/RosieTheGhostie/seaside

#[cfg(feature = "sized")]
pub mod sized;
pub mod traits;

pub use seaside_type_aliases::Address;

use traits::{Contains, Overlapping};

/// An inclusive range of memory addresses.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AddressRange {
    /// The inclusive minimum address in the range.
    base: Address,

    /// The inclusive maximum address in the range.
    limit: Address,
}

impl AddressRange {
    /// An [`AddressRange`] covering all possible memory addresses.
    pub const FULL: Self = unsafe { Self::new_unchecked(Address::MIN, Address::MAX) };

    /// Constructs a new address range.
    ///
    /// # Panics
    ///
    /// This method will panic if `limit` is strictly less than `base`. For a non-panicking
    /// alternative, see [`new_checked`](Self::new_checked).
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// let data_segment = AddressRange::new(0x1001_0000, 0x1003_ffff);
    /// assert!(!data_segment.contains_address(0x1000_ffff));
    /// assert!(data_segment.contains_address(0x1001_0000));
    /// assert!(data_segment.contains_address(0x1002_6942));
    /// assert!(data_segment.contains_address(0x1003_ffff));
    /// assert!(!data_segment.contains_address(0x1004_0000));
    /// ```
    ///
    /// The following example will panic because the limit address (`0x4000_0000`) is less than the
    /// base address (`0x8000_0000`).
    ///
    /// ```should_panic
    /// # use seaside_address_range::AddressRange;
    /// let backwards_range = AddressRange::new(0x8000_0000, 0x4000_0000); // PANIC
    /// ```
    ///
    /// # See Also
    ///
    /// - [`new_checked`](Self::new_checked)
    /// - [`new_unchecked`](Self::new_unchecked)
    pub const fn new(base: Address, limit: Address) -> Self {
        Self::new_checked(base, limit)
            .expect("limit address should be greater than or equal to the base address")
    }

    /// Attempts to construct a new address range.
    ///
    /// This method will return [`None`] if `limit` is strictly less than `base`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// assert!(AddressRange::new_checked(0x1001_0000, 0x1003_ffff).is_some());
    /// assert!(AddressRange::new_checked(0x0040_0000, 0x0040_0000).is_some());
    /// assert!(AddressRange::new_checked(0x0000_0000, 0xffff_ffff).is_some());
    /// assert!(AddressRange::new_checked(0xffff_ffff, 0x0000_0000).is_none());
    /// assert!(AddressRange::new_checked(0x0040_0000, 0x003f_ffff).is_none());
    /// ```
    ///
    /// # See Also
    ///
    /// - [`new`](Self::new)
    /// - [`new_unchecked`](Self::new_unchecked)
    pub const fn new_checked(base: Address, limit: Address) -> Option<Self> {
        if limit >= base {
            Some(unsafe { Self::new_unchecked(base, limit) })
        } else {
            None
        }
    }

    /// Constructs a new address range.
    ///
    /// # Safety
    ///
    /// Using the constructed range will result in undefined behavior if
    /// [`new_checked`](Self::new_checked) would have returned [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// // Do not do this.
    /// let dangerous_range = unsafe { AddressRange::new_unchecked(0x8000_0000, 0x4000_0000) };
    /// ```
    ///
    /// # See Also
    ///
    /// - [`new`](Self::new)
    /// - [`new_checked`](Self::new_checked)
    pub const unsafe fn new_unchecked(base: Address, limit: Address) -> Self {
        Self { base, limit }
    }

    /// Returns the inclusive minimum address in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// let ktext_segment = AddressRange::new(0x8000_0000, 0x8fff_ffff);
    /// assert_eq!(ktext_segment.base(), 0x8000_0000);
    /// ```
    pub const fn base(&self) -> Address {
        self.base
    }

    /// Returns the inclusive maximum address in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// let ktext_segment = AddressRange::new(0x8000_0000, 0x8fff_ffff);
    /// assert_eq!(ktext_segment.limit(), 0x8fff_ffff);
    /// ```
    pub const fn limit(&self) -> Address {
        self.limit
    }

    /// Returns `true` if `address` is within this range.
    ///
    /// In non-`const` contexts, one could also use the [`Contains`] `trait` for identical effect.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// let extern_segment = AddressRange::new(0x1000_0000, 0x1000_ffff);
    ///
    /// assert!(!extern_segment.contains_address(0x0fff_ffff));
    /// assert!(extern_segment.contains_address(0x1000_0000));
    /// assert!(extern_segment.contains_address(0x1000_6942));
    /// assert!(extern_segment.contains_address(0x1000_ffff));
    /// assert!(!extern_segment.contains_address(0x1001_0000));
    /// ```
    pub const fn contains_address(&self, address: Address) -> bool {
        self.base <= address && address <= self.limit
    }

    /// Returns `true` if this range can be [split](Self::split) at the given address.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// let stack_and_heap = AddressRange::new(0x1004_0000, 0x7fff_ffff);
    ///
    /// assert!(!stack_and_heap.can_split(0x1004_0000));
    /// assert!(stack_and_heap.can_split(0x1004_0001));
    /// assert!(stack_and_heap.can_split(0x4000_0000));
    /// assert!(stack_and_heap.can_split(0x7fff_fffe));
    /// assert!(stack_and_heap.can_split(0x7fff_ffff));
    /// assert!(!stack_and_heap.can_split(0x8000_0000));
    /// ```
    pub const fn can_split(&self, at: Address) -> bool {
        self.base < at && at <= self.limit
    }

    /// Splits this range into two ranges.
    ///
    /// The first range will contain all addresses between the base (inclusive) and `at`
    /// (exclusive), and the second will contain all addresses between `at` (inclusive) and the
    /// limit (inclusive).
    ///
    /// # Panics
    ///
    /// This method will panic if `at` is not within this range or if either of the new ranges would
    /// be empty. For a non-panicking alternative, see [`split_checked`](Self::split_checked).
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// let (user_space, kernel_space) = AddressRange::FULL.split(0x8000_0000);
    ///
    /// assert_eq!(user_space.base(), 0x0000_0000);
    /// assert_eq!(user_space.limit(), 0x7fff_ffff);
    /// assert_eq!(kernel_space.base(), 0x8000_0000);
    /// assert_eq!(kernel_space.limit(), 0xffff_ffff);
    /// ```
    ///
    /// The following example will panic because the split address lies outside the range.
    ///
    /// ```should_panic
    /// # use seaside_address_range::AddressRange;
    /// let text_segment = AddressRange::new(0x0040_0000, 0x0fff_ffff);
    /// let (left, right) = text_segment.split(0x2184_9640); // PANIC
    /// ```
    ///
    /// # See Also
    ///
    /// - [`split_checked`](Self::split_checked)
    /// - [`split_unchecked`](Self::split_unchecked)
    pub const fn split(self, at: Address) -> (Self, Self) {
        self.split_checked(at)
            .expect("split should result in two valid, non-empty address ranges")
    }

    /// Attempts to split this range into two ranges.
    ///
    /// This method will return [`None`] if `at` is not within this range or if either of the new
    /// ranges would be empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// let text_segment = AddressRange::new(0x0040_0000, 0x0fff_ffff);
    ///
    /// assert!(text_segment.split_checked(0x0040_0000).is_none());
    /// assert!(text_segment.split_checked(0x0040_0001).is_some());
    /// assert!(text_segment.split_checked(0x0049_2891).is_some());
    /// assert!(text_segment.split_checked(0x0fff_fffe).is_some());
    /// assert!(text_segment.split_checked(0x0fff_ffff).is_some());
    /// assert!(text_segment.split_checked(0x1000_0000).is_none());
    /// ```
    ///
    /// # See Also
    ///
    /// - [`split`](Self::split)
    /// - [`split_unchecked`](Self::split_unchecked)
    pub const fn split_checked(self, at: Address) -> Option<(Self, Self)> {
        if self.can_split(at) {
            Some(unsafe { self.split_unchecked(at) })
        } else {
            None
        }
    }

    /// Splits this range into two ranges.
    ///
    /// # Safety
    ///
    /// Using either of the new ranges will result in undefined behavior if
    /// [`split_checked`](Self::split_checked) would have returned [`None`].
    ///
    /// # Examples
    ///
    /// In the following example, `lower` is "supposed" to be empty; however, [`AddressRange`] does
    /// not support such a concept.
    ///
    /// ```
    /// # use seaside_address_range::AddressRange;
    /// // Do not do this.
    /// let (lower, upper) = unsafe { AddressRange::FULL.split_unchecked(0x0000_0000) };
    /// ```
    ///
    /// # See Also
    ///
    /// - [`split`](Self::split)
    /// - [`split_checked`](Self::split_checked)
    pub const unsafe fn split_unchecked(self, at: Address) -> (Self, Self) {
        let before_at = unsafe { at.unchecked_sub(1) };
        (
            unsafe { Self::new_unchecked(self.base, before_at) },
            unsafe { Self::new_unchecked(at, self.limit) },
        )
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
        // SAFETY: `0 >= 0`
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

/// Constructs an [`AddressRange`] using the standard range syntax.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate seaside_address_range;
/// # fn main() {
/// let user_space = address_range![..0x8000_0000];
///
/// assert_eq!(user_space.base(), 0x0000_0000);
/// assert_eq!(user_space.limit(), 0x7fff_ffff);
/// # }
/// ```
#[macro_export]
macro_rules! address_range {
    [..$(=)?] => {
        $crate::AddressRange::FULL
    };
    [$base:literal..$(=)?] => {
        $crate::AddressRange::new($base, $crate::Address::MAX)
    };
    [..$limit:literal] => {
        $crate::AddressRange::new($crate::Address::MIN, $limit - 1)
    };
    [..=$limit:literal] => {
        $crate::AddressRange::new($crate::Address::MIN, $limit)
    };
    [$base:literal..$limit:literal] => {
        $crate::AddressRange::new($base, $limit - 1)
    };
    [$base:literal..=$limit:literal] => {
        $crate::AddressRange::new($base, $limit)
    };
}
