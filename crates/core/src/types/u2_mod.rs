use core::{
    fmt::{self, Debug, Display, Formatter},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

/// The 2-bit unsigned integer type.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct u2(InnerU2);

impl u2 {
    /// The size of this integer type in bits.
    ///
    /// Note that, unlike for primitive integer types, the size of this type (as reported by
    /// [`size_of`]) is not an eighth of this constant. Put another way, the following assertion
    /// will succeed, even though the equivalent assertion for primitive integer types would fail:
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// assert_ne!(size_of::<u2>() as u32 * u8::BITS, u2::BITS);
    /// ```
    pub const BITS: u32 = 2;

    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self::new(InnerU2::MIN_AS_U8).expect("inner type has incorrect minimum");

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self::new(InnerU2::MAX_AS_U8).expect("inner type has incorrect maximum");

    /// Attempts to construct a new [`u2`] from the smallest primitive integer type wider than it.
    ///
    /// This will fail if the input is too large for a [`u2`] to represent.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// assert!(u2::new(0).is_some_and(|x| x.as_u8() == 0));
    /// assert!(u2::new(1).is_some_and(|x| x.as_u8() == 1));
    /// assert!(u2::new(2).is_some_and(|x| x.as_u8() == 2));
    /// assert!(u2::new(3).is_some_and(|x| x.as_u8() == 3));
    /// assert!(u2::new(4).is_none());
    /// ```
    ///
    /// # See Also
    ///
    /// - [`new_unchecked`](Self::new_unchecked)
    /// - [`new_wrapped`](Self::new_wrapped)
    #[inline(always)]
    pub const fn new(x: u8) -> Option<Self> {
        match InnerU2::new(x) {
            Some(inner) => Some(Self(inner)),
            None => None,
        }
    }

    /// Constructs a new [`u2`] from the smallest primitive integer type wider than it.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring that the input fits inside a [`u2`]. Failure to do so
    /// may result in undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// assert_eq!(unsafe { u2::new_unchecked(0) }.as_u8(), 0);
    /// assert_eq!(unsafe { u2::new_unchecked(1) }.as_u8(), 1);
    /// assert_eq!(unsafe { u2::new_unchecked(2) }.as_u8(), 2);
    /// assert_eq!(unsafe { u2::new_unchecked(3) }.as_u8(), 3);
    /// ```
    ///
    /// The following example would result in undefined behavior because the number 4 is 3 bits
    /// wide.
    ///
    /// ```ignore
    /// // Don't do this.
    /// let x = unsafe { u2::new_unchecked(4) };
    /// ```
    ///
    /// # See Also
    ///
    /// - [`new`](Self::new)
    /// - [`new_wrapped`](Self::new_wrapped)
    #[inline(always)]
    pub const unsafe fn new_unchecked(x: u8) -> Self {
        Self(unsafe { InnerU2::new_unchecked(x) })
    }

    /// Constructs a new [`u2`] from the smallest primitive integer type wider than it, wrapping at
    /// the boundaries of a [`u2`] in the case of overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// // These all behave as normal because their inputs are small enough.
    /// assert_eq!(u2::new_wrapped(0).as_u8(), 0);
    /// assert_eq!(u2::new_wrapped(1).as_u8(), 1);
    /// assert_eq!(u2::new_wrapped(2).as_u8(), 2);
    /// assert_eq!(u2::new_wrapped(3).as_u8(), 3);
    ///
    /// // These all result in wrapping because their inputs are too large for this type.
    /// assert_eq!(u2::new_wrapped(4).as_u8(), 0);
    /// assert_eq!(u2::new_wrapped(5).as_u8(), 1);
    /// assert_eq!(u2::new_wrapped(6).as_u8(), 2);
    /// assert_eq!(u2::new_wrapped(7).as_u8(), 3);
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

    /// Constructs a new [`u2`] from a [boolean](bool) value.
    ///
    /// This is semantically equivalent to `b as u2` (if [`u2`] were a primitive/pointer type,
    /// anyway).
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// assert_eq!(u2::from_bool(false).as_u8(), 0);
    /// assert_eq!(u2::from_bool(true).as_u8(), 1);
    /// ```
    #[inline(always)]
    pub const fn from_bool(b: bool) -> Self {
        // SAFETY: A `bool` is one bit wide, so it will always fit inside a `u2`.
        unsafe { Self::new_unchecked(b as _) }
    }

    /// Casts this integer to a [`u8`].
    #[inline(always)]
    pub const fn as_u8(self) -> u8 {
        self.0 as _
    }

    /// Casts this integer to a [`u16`].
    #[inline(always)]
    pub const fn as_u16(self) -> u16 {
        self.0 as _
    }

    /// Casts this integer to a [`u32`].
    #[inline(always)]
    pub const fn as_u32(self) -> u32 {
        self.0 as _
    }

    /// Casts this integer to a [`u64`].
    #[inline(always)]
    pub const fn as_u64(self) -> u64 {
        self.0 as _
    }

    /// Casts this integer to a [`u128`].
    #[inline(always)]
    pub const fn as_u128(self) -> u128 {
        self.0 as _
    }

    /// Checked integer addition. Computes `self + rhs`, returning [`None`] if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ZERO.checked_add(ZERO), Some(ZERO));
    /// assert_eq!(TWO.checked_add(ZERO), Some(TWO));
    /// assert_eq!(TWO.checked_add(ONE), Some(THREE));
    /// assert_eq!(TWO.checked_add(TWO), None);
    /// assert_eq!(THREE.checked_add(ONE), None);
    /// ```
    #[inline(always)]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        // SAFETY: `u2::MAX + u2::MAX == 6`, and `6 <= u8::MAX`.
        let sum_u8 = unsafe { self.as_u8().unchecked_add(rhs.as_u8()) };
        Self::new(sum_u8)
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning [`None`] if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ZERO.checked_sub(ZERO), Some(ZERO));
    /// assert_eq!(TWO.checked_sub(ZERO), Some(TWO));
    /// assert_eq!(TWO.checked_sub(ONE), Some(ONE));
    /// assert_eq!(TWO.checked_sub(TWO), Some(ZERO));
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
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ZERO.checked_mul(ZERO), Some(ZERO));
    /// assert_eq!(ONE.checked_mul(TWO), Some(TWO));
    /// assert_eq!(THREE.checked_mul(ONE), Some(THREE));
    /// assert_eq!(TWO.checked_mul(TWO), None);
    /// assert_eq!(THREE.checked_mul(TWO), None);
    /// ```
    #[inline(always)]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        // SAFETY: `u2::MAX * u2::MAX == 9`, and `9 <= u8::MAX`.
        let product_u8 = unsafe { self.as_u8().unchecked_mul(rhs.as_u8()) };
        Self::new(product_u8)
    }

    /// Checked integer division. Computes `self / rhs`, returning [`None`] if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(TWO.checked_div(ONE), Some(TWO));
    /// assert_eq!(TWO.checked_div(TWO), Some(ONE));
    /// assert_eq!(THREE.checked_div(TWO), Some(ONE));
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
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(TWO.checked_rem(ONE), Some(ZERO));
    /// assert_eq!(TWO.checked_rem(TWO), Some(ZERO));
    /// assert_eq!(THREE.checked_rem(TWO), Some(ONE));
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
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ZERO.leading_zeros(), 2);
    /// assert_eq!(ONE.leading_zeros(), 1);
    /// assert_eq!(TWO.leading_zeros(), 0);
    /// assert_eq!(THREE.leading_zeros(), 0);
    /// ```
    #[inline(always)]
    pub const fn leading_zeros(self) -> u32 {
        const LEFTOVER_BITS: u32 = u8::BITS - u2::BITS;

        self.as_u8().leading_zeros() - LEFTOVER_BITS
    }

    /// Returns the number of leading ones in the binary representation of self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ZERO.leading_ones(), 0);
    /// assert_eq!(ONE.leading_ones(), 0);
    /// assert_eq!(TWO.leading_ones(), 1);
    /// assert_eq!(THREE.leading_ones(), 2);
    /// ```
    #[inline(always)]
    pub const fn leading_ones(self) -> u32 {
        // You could certainly implement this mathematically, but given how few `u2`s there are, it
        // is way simpler to just use a LUT.
        const ANSWERS: [u32; 4] = [0, 0, 1, 2];

        ANSWERS[self.0 as usize]
    }

    /// Returns the number of trailing zeros in the binary representation of self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ZERO.trailing_zeros(), 2);
    /// assert_eq!(ONE.trailing_zeros(), 0);
    /// assert_eq!(TWO.trailing_zeros(), 1);
    /// assert_eq!(THREE.trailing_zeros(), 0);
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
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ZERO.trailing_ones(), 0);
    /// assert_eq!(ONE.trailing_ones(), 1);
    /// assert_eq!(TWO.trailing_ones(), 0);
    /// assert_eq!(THREE.trailing_ones(), 2);
    /// ```
    #[inline(always)]
    pub const fn trailing_ones(self) -> u32 {
        self.as_u8().trailing_ones()
    }

    /// Returns the logarithm of the number with respect to an arbitrary base, rounded down.
    ///
    /// This method might not be optimized owing to implementation details; [`ilog2`](Self::ilog2)
    /// can produce results more efficiently for base 2. This method also inherently doesn't support
    /// base 10 logarithms (although the results aren't too interesting), so consider
    /// [`ilog10`](Self::ilog10) if that is what you are looking for.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero, or if `base` is less than 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(THREE.ilog(TWO), 1);
    /// assert_eq!(ONE.ilog(THREE), 0);
    /// ```
    ///
    /// The following example will panic because `self` is zero.
    ///
    /// ```should_panic
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// let ilog2_of_zero = ZERO.ilog(TWO);
    /// ```
    ///
    /// The following examples will panic because `base` is less than 2.
    ///
    /// ```should_panic
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// let ilog0_of_one = ONE.ilog(ZERO);
    /// ```
    ///
    /// ```should_panic
    /// # use seaside_core::types::u2;
    /// # const ONE: u2 = u2::new(1).unwrap();
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
    /// [`checked_ilog2`](Self::checked_ilog2) can produce results more efficiently for base 2. This
    /// method also inherently doesn't support base 10 logarithms (although the results aren't too
    /// interesting), so consider [`checked_ilog10`](Self::checked_ilog10) if that is what you are
    /// looking for.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(THREE.checked_ilog(TWO), Some(1));
    /// assert_eq!(ONE.checked_ilog(THREE), Some(0));
    /// assert_eq!(ZERO.checked_ilog(TWO), None);     // self == 0
    /// assert_eq!(ONE.checked_ilog(ZERO), None);     // base < 2
    /// assert_eq!(ONE.checked_ilog(ONE), None);      // base < 2
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
    /// # use seaside_core::types::u2;
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ONE.ilog2(), 0);
    /// assert_eq!(TWO.ilog2(), 1);
    /// assert_eq!(THREE.ilog2(), 1);
    /// ```
    ///
    /// The following example will panic because `self` is zero.
    ///
    /// ```should_panic
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// let ilog2_of_zero = ZERO.ilog2();
    /// ```
    ///
    /// # See Also
    ///
    /// - [`checked_ilog2`](Self::checked_ilog2)
    #[inline(always)]
    pub const fn ilog2(self) -> u32 {
        self.checked_ilog2()
            .expect(super::tiny_uint_panic_messages::NON_POSITIVE_LOGARITHM_ARGUMENT)
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// Returns [`None`] if the number is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ZERO.checked_ilog2(), None);
    /// assert_eq!(ONE.checked_ilog2(), Some(0));
    /// assert_eq!(TWO.checked_ilog2(), Some(1));
    /// assert_eq!(THREE.checked_ilog2(), Some(1));
    /// ```
    ///
    /// # See Also
    ///
    /// - [`ilog2`](Self::ilog2)
    #[inline(always)]
    pub const fn checked_ilog2(self) -> Option<u32> {
        const ANSWERS: [Option<u32>; 4] = [None, Some(0), Some(1), Some(1)];

        ANSWERS[self.0 as usize]
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
    /// # use seaside_core::types::u2;
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ONE.ilog10(), 0);
    /// assert_eq!(TWO.ilog10(), 0);
    /// assert_eq!(THREE.ilog10(), 0);
    /// ```
    ///
    /// The following example will panic because `self` is zero.
    ///
    /// ```should_panic
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// let ilog10_of_zero = ZERO.ilog10();
    /// ```
    ///
    /// # See Also
    ///
    /// - [`checked_ilog10`](Self::checked_ilog10)
    #[inline(always)]
    pub const fn ilog10(self) -> u32 {
        self.checked_ilog10()
            .expect(super::tiny_uint_panic_messages::NON_POSITIVE_LOGARITHM_ARGUMENT)
    }

    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// Returns [`None`] if the number is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use seaside_core::types::u2;
    /// # const ZERO: u2 = u2::new(0).unwrap();
    /// # const ONE: u2 = u2::new(1).unwrap();
    /// # const TWO: u2 = u2::new(2).unwrap();
    /// # const THREE: u2 = u2::new(3).unwrap();
    /// assert_eq!(ZERO.checked_ilog10(), None);
    /// assert_eq!(ONE.checked_ilog10(), Some(0));
    /// assert_eq!(TWO.checked_ilog10(), Some(0));
    /// assert_eq!(THREE.checked_ilog10(), Some(0));
    /// ```
    ///
    /// # See Also
    ///
    /// - [`ilog10`](Self::ilog10)
    #[inline(always)]
    pub const fn checked_ilog10(self) -> Option<u32> {
        const ANSWERS: [Option<u32>; 4] = [None, Some(0), Some(0), Some(0)];

        ANSWERS[self.0 as usize]
    }
}

impl From<bool> for u2 {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl TryFrom<u8> for u2 {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(super::tiny_uint_panic_messages::CONVERT_WITH_OVERFLOW)
    }
}

impl From<u2> for u8 {
    fn from(value: u2) -> Self {
        value.as_u8()
    }
}

impl From<u2> for u16 {
    fn from(value: u2) -> Self {
        value.as_u16()
    }
}

impl From<u2> for u32 {
    fn from(value: u2) -> Self {
        value.as_u32()
    }
}

impl From<u2> for u64 {
    fn from(value: u2) -> Self {
        value.as_u64()
    }
}

impl From<u2> for u128 {
    fn from(value: u2) -> Self {
        value.as_u128()
    }
}

impl Add for u2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect(super::tiny_uint_panic_messages::ADD_WITH_OVERFLOW)
    }
}

impl Add<&Self> for u2 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        self.add(*rhs)
    }
}

impl AddAssign for u2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl AddAssign<&Self> for u2 {
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs);
    }
}

impl BitAnd for u2 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitAnd<&Self> for u2 {
    type Output = Self;

    fn bitand(self, rhs: &Self) -> Self::Output {
        self.bitand(*rhs)
    }
}

impl BitAndAssign for u2 {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitAndAssign<&Self> for u2 {
    fn bitand_assign(&mut self, rhs: &Self) {
        self.bitand_assign(*rhs);
    }
}

impl BitOr for u2 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitOr<&Self> for u2 {
    type Output = Self;

    fn bitor(self, rhs: &Self) -> Self::Output {
        self.bitor(*rhs)
    }
}

impl BitOrAssign for u2 {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitOrAssign<&Self> for u2 {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.bitor_assign(*rhs);
    }
}

impl BitXor for u2 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl BitXor<&Self> for u2 {
    type Output = Self;

    fn bitxor(self, rhs: &Self) -> Self::Output {
        self.bitxor(*rhs)
    }
}

impl BitXorAssign for u2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl BitXorAssign<&Self> for u2 {
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.bitxor_assign(*rhs);
    }
}

impl Debug for u2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <u8 as Debug>::fmt(&self.as_u8(), f)
    }
}

impl Display for u2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <u8 as Display>::fmt(&self.as_u8(), f)
    }
}

impl Div for u2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs)
            .expect(super::tiny_uint_panic_messages::DIVIDE_BY_ZERO)
    }
}

impl Div<&Self> for u2 {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        self.div(*rhs)
    }
}

impl DivAssign for u2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl DivAssign<&Self> for u2 {
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs);
    }
}

impl Mul for u2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs)
            .expect(super::tiny_uint_panic_messages::MULTIPLY_WITH_OVERFLOW)
    }
}

impl Mul<&Self> for u2 {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        self.mul(*rhs)
    }
}

impl MulAssign for u2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl MulAssign<&Self> for u2 {
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs);
    }
}

impl Not for u2 {
    type Output = Self;

    fn not(self) -> Self::Output {
        self ^ Self::MAX
    }
}

impl Rem for u2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.checked_rem(rhs)
            .expect(super::tiny_uint_panic_messages::REMAINDER_WITH_ZERO_DIVISOR)
    }
}

impl Rem<&Self> for u2 {
    type Output = Self;

    fn rem(self, rhs: &Self) -> Self::Output {
        self.rem(*rhs)
    }
}

impl RemAssign for u2 {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

impl RemAssign<&Self> for u2 {
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs);
    }
}

impl Shl for u2 {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self::new_wrapped(self.as_u8() << rhs.as_u8())
    }
}

impl Shl<&Self> for u2 {
    type Output = Self;

    fn shl(self, rhs: &Self) -> Self::Output {
        self.shl(*rhs)
    }
}

impl ShlAssign for u2 {
    fn shl_assign(&mut self, rhs: Self) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&Self> for u2 {
    fn shl_assign(&mut self, rhs: &Self) {
        self.shl_assign(*rhs);
    }
}

impl Shr for u2 {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        let shifted_u8 = self.as_u8() >> rhs.as_u8();

        // SAFETY: Unsigned right shifts can only make values smaller.
        unsafe { Self::new_unchecked(shifted_u8) }
    }
}

impl Shr<&Self> for u2 {
    type Output = Self;

    fn shr(self, rhs: &Self) -> Self::Output {
        self.shr(*rhs)
    }
}

impl ShrAssign for u2 {
    fn shr_assign(&mut self, rhs: Self) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&Self> for u2 {
    fn shr_assign(&mut self, rhs: &Self) {
        self.shr_assign(*rhs);
    }
}

impl Sub for u2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect(super::tiny_uint_panic_messages::SUBTRACT_WITH_OVERFLOW)
    }
}

impl Sub<&Self> for u2 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        self.sub(*rhs)
    }
}

impl SubAssign for u2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl SubAssign<&Self> for u2 {
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs);
    }
}

/// The internal representation of a [`u2`].
///
/// This is implemented as an `enum` primarily to assist in memory layout optimizations, as the
/// compiler will recognize that most bytes cannot represent valid [`u2`]s and is therefore free to
/// use them for other things.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum InnerU2 {
    /// Zero.
    #[default]
    _0 = 0,

    /// One.
    _1 = 1,

    /// Two.
    _2 = 2,

    /// Three.
    _3 = 3,
}

impl InnerU2 {
    /// The minimum value of an [`InnerU2`], represented as a [`u8`] for convenience.
    pub const MIN_AS_U8: u8 = 0;

    /// The maximum value of an [`InnerU2`], represented as a [`u8`] for convenience.
    pub const MAX_AS_U8: u8 = (1 << u2::BITS) - 1;

    /// Attempts to construct a new [`InnerU2`] from the smallest primitive integer type wider than
    /// it.
    ///
    /// This will fail if the input is too large for an [`InnerU2`] to represent.
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

    /// Constructs a new [`InnerU2`] from the smallest primitive integer type wider than it.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring that the input fits inside an [`InnerU2`]. Failure to
    /// do so may result in undefined behavior.
    ///
    /// # See Also
    ///
    /// - [`new`](Self::new)
    #[inline(always)]
    pub const unsafe fn new_unchecked(x: u8) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(x) }
    }

    /// Checks if `x` can be losslessly represented as an [`InnerU2`].
    #[inline(always)]
    const fn can_represent(x: u8) -> bool {
        // A more general implementation would need to check against the lower bound as well, but
        // since this is an unsigned type, the compiler recognizes that such a check would be
        // redundant.
        x <= Self::MAX_AS_U8
    }
}
