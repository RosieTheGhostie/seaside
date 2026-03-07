use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};
use num_traits::Unsigned;

use super::{r#impl, impl_ops, u2, u3};

/// The 5-bit unsigned integer type.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct u5(InnerU5);

impl u5 {
    /// The size of this integer type in bits.
    ///
    /// Note that, unlike for primitive integer types, the size of this type (as reported by
    /// [`size_of`]) is not an eighth of this constant. Put another way, the following assertion
    /// will succeed, even though the equivalent assertion for primitive integer types would fail:
    ///
    /// ```
    /// # use seaside_core::u5;
    /// assert_ne!(size_of::<u5>() as u32 * u8::BITS, u5::BITS);
    /// ```
    pub const BITS: u32 = 5;

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self::new(InnerU5::MIN_AS_U8).expect("inner type has incorrect minimum");

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self::new(InnerU5::MAX_AS_U8).expect("inner type has incorrect maximum");

    /// Attempts to construct a new [`u5`] from the smallest primitive integer type wider than it.
    ///
    /// This will fail if the input is too large for a [`u5`] to represent.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// for x in 0..32 {
    ///     assert!(u5::new(x).is_some_and(|y| y.as_u8() == x));
    /// }
    ///
    /// assert!(u5::new(32).is_none());
    /// ```
    ///
    /// # See Also
    ///
    /// - [`new_unchecked`](Self::new_unchecked)
    /// - [`new_wrapped`](Self::new_wrapped)
    #[inline(always)]
    pub const fn new(x: u8) -> Option<Self> {
        match InnerU5::new(x) {
            Some(inner) => Some(Self(inner)),
            None => None,
        }
    }

    /// Constructs a new [`u5`] from the smallest primitive integer type wider than it.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring that the input fits inside a [`u5`]. Failure to do so
    /// may result in undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// for x in 0..32 {
    ///     assert_eq!(unsafe { u5::new_unchecked(x) }.as_u8(), x);
    /// }
    /// ```
    ///
    /// The following example would result in undefined behavior because the number 8 is 4 bits
    /// wide.
    ///
    /// ```ignore
    /// // Don't do this.
    /// let x = unsafe { u5::new_unchecked(32) };
    /// ```
    ///
    /// # See Also
    ///
    /// - [`new`](Self::new)
    /// - [`new_wrapped`](Self::new_wrapped)
    #[inline(always)]
    pub const unsafe fn new_unchecked(x: u8) -> Self {
        Self(unsafe { InnerU5::new_unchecked(x) })
    }

    /// Constructs a new [`u5`] from the smallest primitive integer type wider than it, wrapping at
    /// the boundaries of a [`u5`] in the case of overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// // These all behave as normal because their inputs are small enough.
    /// assert_eq!(u5::new_wrapped(0).as_u8(), 0);
    /// assert_eq!(u5::new_wrapped(1).as_u8(), 1);
    /// assert_eq!(u5::new_wrapped(30).as_u8(), 30);
    /// assert_eq!(u5::new_wrapped(31).as_u8(), 31);
    ///
    /// // These all result in wrapping because their inputs are too large for this type.
    /// assert_eq!(u5::new_wrapped(32).as_u8(), 0);
    /// assert_eq!(u5::new_wrapped(33).as_u8(), 1);
    /// assert_eq!(u5::new_wrapped(62).as_u8(), 30);
    /// assert_eq!(u5::new_wrapped(63).as_u8(), 31);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`new`](Self::new)
    /// - [`new_unchecked`](Self::new_unchecked)
    #[inline(always)]
    pub const fn new_wrapped(x: u8) -> Self {
        Self::MAX.and_u8(x)
    }

    /// Constructs a new [`u5`] from a [boolean](bool) value.
    ///
    /// This is semantically equivalent to `b as u5` (if [`u5`] were a primitive/pointer type,
    /// anyway).
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// assert_eq!(u5::from_bool(false).as_u8(), 0);
    /// assert_eq!(u5::from_bool(true).as_u8(), 1);
    /// ```
    #[inline(always)]
    pub const fn from_bool(b: bool) -> Self {
        // SAFETY: A `bool` is one bit wide, so it will always fit inside a `u5`.
        unsafe { Self::new_unchecked(b as _) }
    }

    /// Gets the most significant bit of this integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const TWENTY: u5 = u5::new(20).unwrap();
    /// assert_eq!(ZERO.most_significant_bit(), 0 as _);
    /// assert_eq!(TWENTY.most_significant_bit(), 1 as _);
    /// assert_eq!(u5::MAX.most_significant_bit(), 1 as _);
    /// ```
    pub const fn most_significant_bit(self) -> bool {
        const SHIFT_AMOUNT: u32 = u5::BITS - 1;

        self.as_u8() >> SHIFT_AMOUNT != 0
    }

    /// Gets the two most significant bits of this integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::{u2, u5};
    /// # const TWENTY: u5 = u5::new(20).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// assert_eq!(TWENTY.upper_two_bits(), TWO);
    /// assert_eq!(u5::MAX.upper_two_bits(), u2::MAX);
    /// ```
    pub const fn upper_two_bits(self) -> u2 {
        const SHIFT_AMOUNT: u32 = u5::BITS - u2::BITS;

        // SAFETY: The resulting value will be at most two bits wide.
        unsafe { u2::new_unchecked(self.as_u8() >> SHIFT_AMOUNT) }
    }

    /// Gets the three most significant bits of this integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::{u3, u5};
    /// # const TWENTY: u5 = u5::new(20).unwrap();
    /// # const FIVE: u3 = u3::new(5).unwrap();
    /// assert_eq!(TWENTY.upper_three_bits(), FIVE);
    /// assert_eq!(u5::MAX.upper_two_bits(), u3::MAX);
    /// ```
    pub const fn upper_three_bits(self) -> u3 {
        const SHIFT_AMOUNT: u32 = u5::BITS - u3::BITS;

        // SAFETY: The resulting value will be at most three bits wide.
        unsafe { u3::new_unchecked(self.as_u8() >> SHIFT_AMOUNT) }
    }

    /// Gets the least significant bit of this integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const FIVE: u5 = u5::new(5).unwrap();
    /// # const SIX: u5 = u5::new(6).unwrap();
    /// assert_eq!(FIVE.least_significant_bit(), 1 as _);
    /// assert_eq!(SIX.least_significant_bit(), 0 as _);
    /// assert_eq!(u5::MAX.least_significant_bit(), 1 as _);
    /// ```
    #[doc(alias = "is_odd")]
    pub const fn least_significant_bit(self) -> bool {
        self.as_u8() != 0
    }

    /// Gets the two least significant bits of this integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::{u2, u5};
    /// # const SIX: u5 = u5::new(6).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// assert_eq!(SIX.lower_two_bits(), TWO);
    /// assert_eq!(u5::MAX.lower_two_bits(), u2::MAX);
    /// ```
    pub const fn lower_two_bits(self) -> u2 {
        u2::new_wrapped(self.as_u8())
    }

    /// Gets the three least significant bits of this integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::{u3, u5};
    /// # const TWELVE: u5 = u5::new(12).unwrap();
    /// # const FOUR: u3 = u3::new(4).unwrap();
    /// assert_eq!(TWELVE.lower_three_bits(), FOUR);
    /// assert_eq!(u5::MAX.lower_three_bits(), u3::MAX);
    /// ```
    pub const fn lower_three_bits(self) -> u3 {
        u3::new_wrapped(self.as_u8())
    }

    /// Checked integer addition. Computes `self + rhs`, returning [`None`] if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const FOUR: u5 = u5::new(4).unwrap();
    /// # const FIVE: u5 = u5::new(5).unwrap();
    /// # const FOURTEEN: u5 = u5::new(14).unwrap();
    /// # const SEVENTEEN: u5 = u5::new(17).unwrap();
    /// # const THIRTY_ONE: u5 = u5::new(31).unwrap();
    /// assert_eq!(ZERO.checked_add(ZERO), Some(ZERO));
    /// assert_eq!(TWO.checked_add(ZERO), Some(TWO));
    /// assert_eq!(FOURTEEN.checked_add(SEVENTEEN), Some(THIRTY_ONE));
    /// assert_eq!(SEVENTEEN.checked_add(SEVENTEEN), None);
    /// assert_eq!(u5::MAX.checked_add(ONE), None);
    /// ```
    #[inline(always)]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        // SAFETY: `u5::MAX + u5::MAX == 62`, and `62 <= u8::MAX`.
        let sum_u8 = unsafe { self.as_u8().unchecked_add(rhs.as_u8()) };
        Self::new(sum_u8)
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning [`None`] if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const THREE: u5 = u5::new(3).unwrap();
    /// # const FOUR: u5 = u5::new(4).unwrap();
    /// # const FIVE: u5 = u5::new(5).unwrap();
    /// assert_eq!(ZERO.checked_sub(ZERO), Some(ZERO));
    /// assert_eq!(TWO.checked_sub(ZERO), Some(TWO));
    /// assert_eq!(FIVE.checked_sub(ONE), Some(FOUR));
    /// assert_eq!(THREE.checked_sub(THREE), Some(ZERO));
    /// assert_eq!(ONE.checked_sub(THREE), None);
    /// ```
    #[inline(always)]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.as_u8().checked_sub(rhs.as_u8()) {
            // SAFETY: Unsigned subtraction will always yield a value less than the first operand
            //         unless it underflows, and we know it didn't underflow if we got a `Some`
            //         variant.
            Some(difference_u8) => Some(unsafe { Self::new_unchecked(difference_u8) }),
            None => None,
        }
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning [`None`] if overflow
    /// occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const THREE: u5 = u5::new(3).unwrap();
    /// # const FOUR: u5 = u5::new(4).unwrap();
    /// # const FIVE: u5 = u5::new(5).unwrap();
    /// # const SEVEN: u5 = u5::new(7).unwrap();
    /// # const TWELVE: u5 = u5::new(12).unwrap();
    /// # const TWENTY_FOUR: u5 = u5::new(24).unwrap();
    /// assert_eq!(ZERO.checked_mul(ZERO), Some(ZERO));
    /// assert_eq!(ONE.checked_mul(TWO), Some(TWO));
    /// assert_eq!(THREE.checked_mul(ONE), Some(THREE));
    /// assert_eq!(TWELVE.checked_mul(TWO), Some(TWENTY_FOUR));
    /// assert_eq!(TWELVE.checked_mul(THREE), None);
    /// assert_eq!(FIVE.checked_mul(SEVEN), None);
    /// ```
    #[inline(always)]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        // SAFETY: `u5::MAX * u5::MAX == 961`, and `961 <= u16::MAX`.
        let product_u16 = unsafe { self.as_u16().unchecked_mul(rhs.as_u16()) };
        if product_u16 <= Self::MAX.as_u16() {
            // SAFETY: The invariant is satisfied.
            Some(unsafe { Self::new_unchecked(product_u16 as _) })
        } else {
            None
        }
    }

    /// Checked integer division. Computes `self / rhs`, returning [`None`] if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const FIVE: u5 = u5::new(5).unwrap();
    /// # const SEVEN: u5 = u5::new(7).unwrap();
    /// assert_eq!(TWO.checked_div(ONE), Some(TWO));
    /// assert_eq!(TWO.checked_div(TWO), Some(ONE));
    /// assert_eq!(SEVEN.checked_div(FIVE), Some(ONE));
    /// assert_eq!(ZERO.checked_div(ZERO), None);
    /// assert_eq!(ONE.checked_div(ZERO), None);
    /// ```
    #[inline(always)]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.as_u8().checked_div(rhs.as_u8()) {
            // SAFETY: Unsigned integer division can only make values smaller.
            Some(quotient_u8) => Some(unsafe { Self::new_unchecked(quotient_u8) }),
            None => None,
        }
    }

    /// Checked integer remainder. Computes `self % rhs`, returning [`None`] if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const FIVE: u5 = u5::new(5).unwrap();
    /// # const SEVEN: u5 = u5::new(7).unwrap();
    /// assert_eq!(TWO.checked_rem(ONE), Some(ZERO));
    /// assert_eq!(TWO.checked_rem(TWO), Some(ZERO));
    /// assert_eq!(SEVEN.checked_rem(FIVE), Some(TWO));
    /// assert_eq!(ZERO.checked_rem(ZERO), None);
    /// assert_eq!(ONE.checked_rem(ZERO), None);
    /// ```
    #[inline(always)]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.as_u8().checked_rem(rhs.as_u8()) {
            // SAFETY: Unsigned division always yields a remainder strictly less than the divisor.
            Some(remainder_u8) => Some(unsafe { Self::new_unchecked(remainder_u8) }),
            None => None,
        }
    }

    /// Performs the bitwise AND (`&`) operation.
    ///
    /// This is effectively a `const` version of [`BitAnd::bitand`].
    #[inline(always)]
    pub const fn and(self, rhs: Self) -> Self {
        Self::and_u8(self, rhs.as_u8())
    }

    /// Performs the bitwise AND (`&`) operation.
    ///
    /// This is effectively a `const` version of [`BitAnd::bitand`].
    #[inline(always)]
    pub const fn and_bool(self, rhs: bool) -> Self {
        Self::and_u8(self, rhs as _)
    }

    /// Performs the bitwise AND (`&`) operation.
    ///
    /// This is effectively a `const` version of [`BitAnd::bitand`].
    #[inline(always)]
    pub const fn and_u8(self, rhs: u8) -> Self {
        // SAFETY: Bitwise AND operations between two integers yield an integer that is at most as
        //         wide as the smaller of the two integers.
        unsafe { Self::new_unchecked(self.as_u8() & rhs) }
    }

    /// Performs the bitwise AND (`&`) operation.
    ///
    /// This is effectively a `const` version of [`BitAnd::bitand`].
    #[inline(always)]
    pub const fn and_u16(self, rhs: u16) -> Self {
        Self::and_u8(self, (rhs & u8::MAX as u16) as _)
    }

    /// Performs the bitwise AND (`&`) operation.
    ///
    /// This is effectively a `const` version of [`BitAnd::bitand`].
    #[inline(always)]
    pub const fn and_u32(self, rhs: u32) -> Self {
        Self::and_u8(self, (rhs & u8::MAX as u32) as _)
    }

    /// Performs the bitwise AND (`&`) operation.
    ///
    /// This is effectively a `const` version of [`BitAnd::bitand`].
    #[inline(always)]
    pub const fn and_u64(self, rhs: u64) -> Self {
        Self::and_u8(self, (rhs & u8::MAX as u64) as _)
    }

    /// Performs the bitwise AND (`&`) operation.
    ///
    /// This is effectively a `const` version of [`BitAnd::bitand`].
    #[inline(always)]
    pub const fn and_u128(self, rhs: u128) -> Self {
        Self::and_u8(self, (rhs & u8::MAX as u128) as _)
    }

    /// Performs the bitwise OR (`|`) operation.
    ///
    /// This is effectively a `const` version of [`BitOr::bitor`].
    #[inline(always)]
    pub const fn or(self, rhs: Self) -> Self {
        // SAFETY: Bitwise OR operations between two integers of the same size yield an integer of
        //         that same size.
        unsafe { Self::new_unchecked(self.as_u8() | rhs.as_u8()) }
    }

    /// Performs the bitwise OR (`|`) operation.
    ///
    /// This is effectively a `const` version of [`BitOr::bitor`].
    #[inline(always)]
    pub const fn or_bool(self, rhs: bool) -> Self {
        self.or(Self::from_bool(rhs))
    }

    /// Performs the bitwise XOR (`^`) operation.
    ///
    /// This is effectively a `const` version of [`BitXor::bitxor`].
    #[inline(always)]
    pub const fn xor(self, rhs: Self) -> Self {
        // SAFETY: Bitwise XOR operations between two integers of the same size yield an integer of
        //         that same size.
        unsafe { Self::new_unchecked(self.as_u8() ^ rhs.as_u8()) }
    }

    /// Performs the bitwise XOR (`^`) operation.
    ///
    /// This is effectively a `const` version of [`BitXor::bitxor`].
    #[inline(always)]
    pub const fn xor_bool(self, rhs: bool) -> Self {
        self.xor(Self::from_bool(rhs))
    }

    /// Returns the number of leading zeros in the binary representation of self.
    ///
    /// Depending on what you're doing with the value, you might also be interested in the
    /// [`ilog2`](Self::ilog2) function which returns a consistent number, even if the type widens.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const THREE: u5 = u5::new(3).unwrap();
    /// # const FOUR: u5 = u5::new(4).unwrap();
    /// assert_eq!(ZERO.leading_zeros(), u5::BITS);
    /// assert_eq!(ONE.leading_zeros(), 4);
    /// assert_eq!(TWO.leading_zeros(), 3);
    /// assert_eq!(THREE.leading_zeros(), 3);
    /// assert_eq!(FOUR.leading_zeros(), 2);
    /// assert_eq!(u5::MAX.leading_zeros(), 0);
    /// ```
    #[inline(always)]
    pub const fn leading_zeros(self) -> u32 {
        const LEFTOVER_BITS: u32 = u8::BITS - u5::BITS;

        self.as_u8().leading_zeros() - LEFTOVER_BITS
    }

    /// Returns the number of leading ones in the binary representation of self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const FIFTEEN: u5 = u5::new(15).unwrap();
    /// # const SIXTEEN: u5 = u5::new(16).unwrap();
    /// # const TWENTY_FOUR: u5 = u5::new(24).unwrap();
    /// # const TWENTY_EIGHT: u5 = u5::new(28).unwrap();
    /// # const THIRTY: u5 = u5::new(30).unwrap();
    /// assert_eq!(ZERO.leading_ones(), 0);
    /// assert_eq!(FIFTEEN.leading_ones(), 0);
    /// assert_eq!(SIXTEEN.leading_ones(), 1);
    /// assert_eq!(TWENTY_FOUR.leading_ones(), 2);
    /// assert_eq!(TWENTY_EIGHT.leading_ones(), 3);
    /// assert_eq!(THIRTY.leading_ones(), 4);
    /// assert_eq!(u5::MAX.leading_ones(), u5::BITS);
    /// ```
    #[inline(always)]
    pub const fn leading_ones(self) -> u32 {
        const LEFTOVER_BITS: u32 = u8::BITS - u5::BITS;

        (self.as_u8() << LEFTOVER_BITS).leading_ones()
    }

    /// Returns the number of trailing zeros in the binary representation of self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const FOUR: u5 = u5::new(4).unwrap();
    /// # const SIX: u5 = u5::new(6).unwrap();
    /// # const TWENTY_FOUR: u5 = u5::new(24).unwrap();
    /// assert_eq!(ZERO.trailing_zeros(), u5::BITS);
    /// assert_eq!(ONE.trailing_zeros(), 0);
    /// assert_eq!(TWO.trailing_zeros(), 1);
    /// assert_eq!(FOUR.trailing_zeros(), 2);
    /// assert_eq!(SIX.trailing_zeros(), 1);
    /// assert_eq!(TWENTY_FOUR.trailing_zeros(), 3);
    /// assert_eq!(u5::MAX.trailing_zeros(), 0);
    /// ```
    #[inline(always)]
    pub const fn trailing_zeros(self) -> u32 {
        if let n @ ..Self::BITS = self.as_u8().trailing_zeros() {
            n
        } else {
            Self::BITS
        }
    }

    /// Returns the number of trailing ones in the binary representation of self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const THREE: u5 = u5::new(3).unwrap();
    /// # const FIVE: u5 = u5::new(5).unwrap();
    /// # const SEVEN: u5 = u5::new(7).unwrap();
    /// # const FIFTEEN: u5 = u5::new(15).unwrap();
    /// assert_eq!(ZERO.trailing_ones(), 0);
    /// assert_eq!(ONE.trailing_ones(), 1);
    /// assert_eq!(TWO.trailing_ones(), 0);
    /// assert_eq!(THREE.trailing_ones(), 2);
    /// assert_eq!(FIVE.trailing_ones(), 1);
    /// assert_eq!(SEVEN.trailing_ones(), 3);
    /// assert_eq!(FIFTEEN.trailing_ones(), 4);
    /// assert_eq!(u5::MAX.trailing_ones(), u5::BITS);
    /// ```
    #[inline(always)]
    pub const fn trailing_ones(self) -> u32 {
        self.as_u8().trailing_ones()
    }

    /// Returns the logarithm of the number with respect to an arbitrary base, rounded down.
    ///
    /// This method might not be optimized owing to implementation details; [`ilog2`](Self::ilog2)
    /// can produce results more efficiently for base 2, and [`ilog10`](Self::ilog10) can produce
    /// results more efficiently for base 10.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero, or if `base` is less than 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const THREE: u5 = u5::new(3).unwrap();
    /// assert_eq!(THREE.ilog(TWO), 1);
    /// assert_eq!(ONE.ilog(THREE), 0);
    /// ```
    ///
    /// The following example will panic because `self` is zero.
    ///
    /// ```should_panic
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// let ilog2_of_zero = ZERO.ilog(TWO);
    /// ```
    ///
    /// The following examples will panic because `base` is less than 2.
    ///
    /// ```should_panic
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// let ilog0_of_one = ONE.ilog(ZERO);
    /// ```
    ///
    /// ```should_panic
    /// # use seaside_core::u5;
    /// # const ONE: u5 = u5::ONE;
    /// let ilog1_of_one = ONE.ilog(ONE);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`checked_ilog`](Self::checked_ilog)
    #[inline(always)]
    pub const fn ilog(self, base: Self) -> u32 {
        self.as_u8().ilog(base.as_u8())
    }

    /// Returns the logarithm of the number with respect to an arbitrary base, rounded down.
    ///
    /// Returns [`None`] if the number is zero, or if the base is not at least 2.
    ///
    /// This method might not be optimized owing to implementation details;
    /// [`checked_ilog2`](Self::checked_ilog2) can produce results more efficiently for base 2, and
    /// [`checked_ilog10`](Self::checked_ilog10) can produce results more efficiently for base 10.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const THREE: u5 = u5::new(3).unwrap();
    /// # const SIX: u5 = u5::new(6).unwrap();
    /// assert_eq!(THREE.checked_ilog(TWO), Some(1));
    /// assert_eq!(ONE.checked_ilog(THREE), Some(0));
    /// assert_eq!(ZERO.checked_ilog(TWO), None);     // self == 0
    /// assert_eq!(ONE.checked_ilog(ZERO), None);     // base < 2
    /// assert_eq!(SIX.checked_ilog(ONE), None);      // base < 2
    /// ```
    ///
    /// # See Also
    ///
    /// - [`ilog`](Self::ilog)
    #[inline(always)]
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        self.as_u8().checked_ilog(base.as_u8())
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const THREE: u5 = u5::new(3).unwrap();
    /// # const FOUR: u5 = u5::new(4).unwrap();
    /// # const EIGHT: u5 = u5::new(8).unwrap();
    /// # const SIXTEEN: u5 = u5::new(16).unwrap();
    /// assert_eq!(ONE.ilog2(), 0);
    /// assert_eq!(TWO.ilog2(), 1);
    /// assert_eq!(THREE.ilog2(), 1);
    /// assert_eq!(FOUR.ilog2(), 2);
    /// assert_eq!(EIGHT.ilog2(), 3);
    /// assert_eq!(SIXTEEN.ilog2(), 4);
    /// ```
    ///
    /// The following example will panic because `self` is zero.
    ///
    /// ```should_panic
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// let ilog2_of_zero = ZERO.ilog2();
    /// ```
    ///
    /// # See Also
    ///
    /// - [`checked_ilog2`](Self::checked_ilog2)
    #[inline(always)]
    pub const fn ilog2(self) -> u32 {
        self.as_u8().ilog2()
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// Returns [`None`] if the number is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const THREE: u5 = u5::new(3).unwrap();
    /// # const FOUR: u5 = u5::new(4).unwrap();
    /// # const EIGHT: u5 = u5::new(8).unwrap();
    /// # const SIXTEEN: u5 = u5::new(16).unwrap();
    /// assert_eq!(ZERO.checked_ilog2(), None);
    /// assert_eq!(ONE.checked_ilog2(), Some(0));
    /// assert_eq!(TWO.checked_ilog2(), Some(1));
    /// assert_eq!(THREE.checked_ilog2(), Some(1));
    /// assert_eq!(FOUR.checked_ilog2(), Some(2));
    /// assert_eq!(EIGHT.checked_ilog2(), Some(3));
    /// assert_eq!(SIXTEEN.checked_ilog2(), Some(4));
    /// ```
    ///
    /// # See Also
    ///
    /// - [`ilog2`](Self::ilog2)
    #[inline(always)]
    pub const fn checked_ilog2(self) -> Option<u32> {
        self.as_u8().checked_ilog2()
    }

    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const NINE: u5 = u5::new(7).unwrap();
    /// # const TEN: u5 = u5::new(10).unwrap();
    /// # const TWENTY: u5 = u5::new(20).unwrap();
    /// assert_eq!(ONE.ilog10(), 0);
    /// assert_eq!(TWO.ilog10(), 0);
    /// assert_eq!(NINE.ilog10(), 0);
    /// assert_eq!(TEN.ilog10(), 1);
    /// assert_eq!(TWENTY.ilog10(), 1);
    /// ```
    ///
    /// The following example will panic because `self` is zero.
    ///
    /// ```should_panic
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// let ilog10_of_zero = ZERO.ilog10();
    /// ```
    ///
    /// # See Also
    ///
    /// - [`checked_ilog10`](Self::checked_ilog10)
    #[inline(always)]
    pub const fn ilog10(self) -> u32 {
        self.as_u8().ilog10()
    }

    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// Returns [`None`] if the number is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::u5;
    /// # const ZERO: u5 = u5::ZERO;
    /// # const ONE: u5 = u5::ONE;
    /// # const TWO: u5 = u5::new(2).unwrap();
    /// # const NINE: u5 = u5::new(7).unwrap();
    /// # const TEN: u5 = u5::new(10).unwrap();
    /// # const TWENTY: u5 = u5::new(20).unwrap();
    /// assert_eq!(ZERO.checked_ilog10(), None);
    /// assert_eq!(ONE.checked_ilog10(), Some(0));
    /// assert_eq!(TWO.checked_ilog10(), Some(0));
    /// assert_eq!(NINE.checked_ilog10(), Some(0));
    /// assert_eq!(TEN.checked_ilog10(), Some(1));
    /// assert_eq!(TWENTY.checked_ilog10(), Some(1));
    /// ```
    ///
    /// # See Also
    ///
    /// - [`ilog10`](Self::ilog10)
    #[inline(always)]
    pub const fn checked_ilog10(self) -> Option<u32> {
        self.as_u8().checked_ilog10()
    }
}

r#impl!(AsPrimitive for u5);
r#impl!(Bounded for u5);
r#impl!(Debug for u5);
r#impl!(Display for u5);
r#impl!(FromPrimitive for u5);
r#impl!(FromStr for u5);
r#impl!(Not for u5);
r#impl!(Num for u5);
r#impl!(NumCast for u5);
r#impl!(ToPrimitive for u5);
impl Unsigned for u5 {}

impl_ops! {
    #![type = u5]

    #[assign(trait = AddAssign, fn = add_assign)]
    impl Add {
        fn add(self, rhs) -> .. {
            self.checked_add(rhs)
                .expect(super::panic_messages::ADD_WITH_OVERFLOW)
        }
    }

    #[assign(trait = BitAndAssign, fn = bitand_assign)]
    impl BitAnd {
        fn bitand(self, rhs) -> .. {
            self.and(rhs)
        }
    }

    #[assign(trait = BitOrAssign, fn = bitor_assign)]
    impl BitOr {
        fn bitor(self, rhs) -> .. {
            self.or(rhs)
        }
    }

    #[assign(trait = BitXorAssign, fn = bitxor_assign)]
    impl BitXor {
        fn bitxor(self, rhs) -> .. {
            self.xor(rhs)
        }
    }

    #[assign(trait = DivAssign, fn = div_assign)]
    impl Div {
        fn div(self, rhs) -> .. {
            self.checked_div(rhs)
                .expect(super::panic_messages::DIVIDE_BY_ZERO)
        }
    }

    #[assign(trait = MulAssign, fn = mul_assign)]
    impl Mul {
        fn mul(self, rhs) -> .. {
            self.checked_mul(rhs)
                .expect(super::panic_messages::MULTIPLY_WITH_OVERFLOW)
        }
    }

    #[assign(trait = RemAssign, fn = rem_assign)]
    impl Rem {
        fn rem(self, rhs) -> .. {
            self.checked_rem(rhs)
                .expect(super::panic_messages::REMAINDER_WITH_ZERO_DIVISOR)
        }
    }

    #[assign(trait = ShlAssign, fn = shl_assign)]
    impl Shl {
        fn shl(self, rhs) -> .. {
            Self::new_wrapped(self.as_u8() << rhs.as_u8())
        }
    }

    #[assign(trait = ShrAssign, fn = shr_assign)]
    impl Shr {
        fn shr(self, rhs) -> .. {
            let shifted_u8 = self.as_u8() >> rhs.as_u8();

            // SAFETY: Unsigned right shifts can only make values smaller.
            unsafe { Self::new_unchecked(shifted_u8) }
        }
    }

    #[assign(trait = SubAssign, fn = sub_assign)]
    impl Sub {
        fn sub(self, rhs) -> .. {
            self.checked_sub(rhs)
                .expect(super::panic_messages::SUBTRACT_WITH_OVERFLOW)
        }
    }
}

/// The internal representation of a [`u5`].
///
/// This is implemented as an `enum` primarily to assist in memory layout optimizations, as the
/// compiler will recognize that most bytes cannot represent valid [`u5`]s and is therefore free to
/// use them for other things.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
#[repr(u8)]
enum InnerU5 {
    /// Zero.
    #[default]
    _0 = 0,

    /// One.
    _1 = 1,

    /// Two.
    _2 = 2,

    /// Three.
    _3 = 3,

    /// Four.
    _4 = 4,

    /// Five.
    _5 = 5,

    /// Six.
    _6 = 6,

    /// Seven.
    _7 = 7,

    /// Eight.
    _8 = 8,

    /// Nine.
    _9 = 9,

    /// Ten.
    _10 = 10,

    /// Eleven.
    _11 = 11,

    /// Twelve.
    _12 = 12,

    /// Thirteen.
    _13 = 13,

    /// Fourteen.
    _14 = 14,

    /// Fifteen.
    _15 = 15,

    /// Sixteen.
    _16 = 16,

    /// Seventeen.
    _17 = 17,

    /// Eighteen.
    _18 = 18,

    /// Nineteen.
    _19 = 19,

    /// Twenty.
    _20 = 20,

    /// Twenty-one.
    _21 = 21,

    /// Twenty-two.
    _22 = 22,

    /// Twenty-three.
    _23 = 23,

    /// Twenty-four.
    _24 = 24,

    /// Twenty-five.
    _25 = 25,

    /// Twenty-six.
    _26 = 26,

    /// Twenty-seven.
    _27 = 27,

    /// Twenty-eight.
    _28 = 28,

    /// Twenty-nine.
    _29 = 29,

    /// Thirty.
    _30 = 30,

    /// Thirty-one.
    _31 = 31,
}

impl InnerU5 {
    /// The minimum value of an [`InnerU5`], represented as a [`u8`] for convenience.
    pub const MIN_AS_U8: u8 = 0;

    /// The maximum value of an [`InnerU5`], represented as a [`u8`] for convenience.
    pub const MAX_AS_U8: u8 = (1 << u5::BITS) - 1;

    /// Attempts to construct a new [`InnerU5`] from the smallest primitive integer type wider than
    /// it.
    ///
    /// This will fail if the input is too large for an [`InnerU5`] to represent.
    ///
    /// # See Also
    ///
    /// - [`new_unchecked`](Self::new_unchecked)
    #[inline(always)]
    pub const fn new(x: u8) -> Option<Self> {
        if Self::can_represent(x) {
            // SAFETY: The invariant is satisfied.
            Some(unsafe { Self::new_unchecked(x) })
        } else {
            None
        }
    }

    /// Constructs a new [`InnerU5`] from the smallest primitive integer type wider than it.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring that the input fits inside an [`InnerU5`]. Failure to
    /// do so may result in undefined behavior.
    ///
    /// # See Also
    ///
    /// - [`new`](Self::new)
    #[inline(always)]
    pub const unsafe fn new_unchecked(x: u8) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(x) }
    }

    /// Checks if `x` can be losslessly represented as an [`InnerU5`].
    #[inline(always)]
    const fn can_represent(x: u8) -> bool {
        // A more general implementation would need to check against the lower bound as well, but
        // since this is an unsigned type, the compiler recognizes that such a check would be
        // redundant.
        x <= Self::MAX_AS_U8
    }
}
