#[doc(inline)]
pub use u2_mod::u2;
#[doc(inline)]
pub use u3_mod::u3;
#[doc(inline)]
pub use u5_mod::u5;

mod u2_mod;
mod u3_mod;
mod u5_mod;

mod panic_messages {
    pub const CONVERT_WITH_OVERFLOW: &str = "attempt to convert with overflow";
    pub const ADD_WITH_OVERFLOW: &str = "attempt to add with overflow";
    pub const SUBTRACT_WITH_OVERFLOW: &str = "attempt to subtract with overflow";
    pub const MULTIPLY_WITH_OVERFLOW: &str = "attempt to multiply with overflow";
    pub const DIVIDE_BY_ZERO: &str = "attempt to divide by zero";
    pub const REMAINDER_WITH_ZERO_DIVISOR: &str =
        "attempt to calculate the remainder with a divisor of zero";
    pub const NON_POSITIVE_LOGARITHM_ARGUMENT: &str =
        "argument of integer logarithm must be positive";
}

mod parse_int_errors {
    use core::num::ParseIntError;

    macro_rules! const_unwrap_err {
        ($expr:expr) => {
            match $expr {
                Ok(_) => panic!("tried to unwrap an Ok variant"),
                Err(err) => err,
            }
        };
    }

    // pub const EMPTY: ParseIntError = const_unwrap_err!(u8::from_str_radix("", 10));
    pub const POS_OVERFLOW: ParseIntError = const_unwrap_err!(u8::from_str_radix("256", 10));
    // pub const NEG_OVERFLOW: ParseIntError = const_unwrap_err!(i8::from_str_radix("-129", 10));
    // pub const INVALID_DIGIT: ParseIntError = const_unwrap_err!(u8::from_str_radix("a", 10));
}

use num_traits::AsPrimitive;

impl AsPrimitive<u3> for u2 {
    fn as_(self) -> u3 {
        self.as_u3()
    }
}

impl AsPrimitive<u5> for u2 {
    fn as_(self) -> u5 {
        self.as_u5()
    }
}

impl AsPrimitive<u5> for u3 {
    fn as_(self) -> u5 {
        self.as_u5()
    }
}

macro_rules! impl_ops {
    {} => {};
    {
        #![type = $t:ty]

        $(
            #[assign(trait = $assign_trait:ident, fn = $assign_fn:ident)]
            impl $op_trait:ident {
                fn $op_fn:ident($self:ident, $rhs:ident) -> .. $impl_block:block
            }
        )*
    } => {
        $(
            impl ::core::ops::$op_trait for $t {
                type Output = Self;

                fn $op_fn($self, $rhs: Self) -> Self::Output $impl_block
            }

            impl ::core::ops::$op_trait<&Self> for $t {
                type Output = Self;

                fn $op_fn($self, $rhs: &Self) -> Self::Output {
                    $self.$op_fn(*$rhs)
                }
            }

            impl ::core::ops::$assign_trait for $t {
                fn $assign_fn(&mut $self, $rhs: Self) {
                    *$self = $self.$op_fn($rhs);
                }
            }

            impl ::core::ops::$assign_trait<&Self> for $t {
                fn $assign_fn(&mut $self, $rhs: &Self) {
                    *$self = $self.$op_fn($rhs);
                }
            }
        )*
    };
}
use impl_ops;

macro_rules! r#impl {
    (AsPrimitive for $t:ty) => {
        impl $t {
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

            /// Casts this integer to a [`usize`].
            #[inline(always)]
            pub const fn as_usize(self) -> usize {
                self.0 as _
            }

            /// Casts this integer to an [`i8`].
            #[inline(always)]
            pub const fn as_i8(self) -> i8 {
                self.0 as _
            }

            /// Casts this integer to an [`i16`].
            #[inline(always)]
            pub const fn as_i16(self) -> i16 {
                self.0 as _
            }

            /// Casts this integer to an [`i32`].
            #[inline(always)]
            pub const fn as_i32(self) -> i32 {
                self.0 as _
            }

            /// Casts this integer to an [`i64`].
            #[inline(always)]
            pub const fn as_i64(self) -> i64 {
                self.0 as _
            }

            /// Casts this integer to an [`i128`].
            #[inline(always)]
            pub const fn as_i128(self) -> i128 {
                self.0 as _
            }

            /// Casts this integer to an [`isize`].
            #[inline(always)]
            pub const fn as_isize(self) -> isize {
                self.0 as _
            }

            /// Casts this integer to an [`f32`].
            #[inline(always)]
            pub const fn as_f32(self) -> f32 {
                self.as_u8() as _
            }

            /// Casts this integer to an [`f64`].
            #[inline(always)]
            pub const fn as_f64(self) -> f64 {
                self.as_u8() as _
            }
        }

        impl ::num_traits::AsPrimitive<u8> for $t {
            fn as_(self) -> u8 {
                self.as_u8()
            }
        }

        impl ::num_traits::AsPrimitive<u16> for $t {
            fn as_(self) -> u16 {
                self.as_u16()
            }
        }

        impl ::num_traits::AsPrimitive<u32> for $t {
            fn as_(self) -> u32 {
                self.as_u32()
            }
        }

        impl ::num_traits::AsPrimitive<u64> for $t {
            fn as_(self) -> u64 {
                self.as_u64()
            }
        }

        impl ::num_traits::AsPrimitive<u128> for $t {
            fn as_(self) -> u128 {
                self.as_u128()
            }
        }

        impl ::num_traits::AsPrimitive<usize> for $t {
            fn as_(self) -> usize {
                self.as_usize()
            }
        }

        impl ::num_traits::AsPrimitive<i8> for $t {
            fn as_(self) -> i8 {
                self.as_i8()
            }
        }

        impl ::num_traits::AsPrimitive<i16> for $t {
            fn as_(self) -> i16 {
                self.as_i16()
            }
        }

        impl ::num_traits::AsPrimitive<i32> for $t {
            fn as_(self) -> i32 {
                self.as_i32()
            }
        }

        impl ::num_traits::AsPrimitive<i64> for $t {
            fn as_(self) -> i64 {
                self.as_i64()
            }
        }

        impl ::num_traits::AsPrimitive<i128> for $t {
            fn as_(self) -> i128 {
                self.as_i128()
            }
        }

        impl ::num_traits::AsPrimitive<isize> for $t {
            fn as_(self) -> isize {
                self.as_isize()
            }
        }

        impl ::num_traits::AsPrimitive<f32> for $t {
            fn as_(self) -> f32 {
                self.as_f32()
            }
        }

        impl ::num_traits::AsPrimitive<f64> for $t {
            fn as_(self) -> f64 {
                self.as_f64()
            }
        }
    };
    (Bounded for $t:ty) => {
        impl ::num_traits::Bounded for $t {
            fn min_value() -> Self {
                Self::MIN
            }

            fn max_value() -> Self {
                Self::MAX
            }
        }
    };
    (Debug for $t:ty) => {
        impl ::core::fmt::Debug for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <u8 as ::core::fmt::Debug>::fmt(&self.as_u8(), f)
            }
        }
    };
    (Display for $t:ty) => {
        impl ::core::fmt::Display for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <u8 as ::core::fmt::Display>::fmt(&self.as_u8(), f)
            }
        }
    };
    (FromPrimitive for $t:ty) => {
        impl ::core::convert::From<bool> for $t {
            #[inline(always)]
            fn from(value: bool) -> Self {
                Self::from_bool(value)
            }
        }

        impl ::num_traits::AsPrimitive<$t> for bool {
            fn as_(self) -> $t {
                self.into()
            }
        }

        impl ::core::convert::TryFrom<u8> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                Self::new(value).ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<u16> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: u16) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_u16(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<u32> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: u32) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_u32(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<u64> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: u64) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_u64(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<u128> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: u128) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_u128(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<usize> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: usize) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_usize(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<i8> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: i8) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_i8(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<i16> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: i16) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_i16(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<i32> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_i32(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<i64> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_i64(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<i128> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: i128) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_i128(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::core::convert::TryFrom<isize> for $t {
            type Error = &'static str;

            #[inline(always)]
            fn try_from(value: isize) -> Result<Self, Self::Error> {
                <Self as ::num_traits::FromPrimitive>::from_isize(value)
                    .ok_or($crate::tiny_uints::panic_messages::CONVERT_WITH_OVERFLOW)
            }
        }

        impl ::num_traits::FromPrimitive for $t {
            #[inline(always)]
            fn from_u8(n: u8) -> Option<Self> {
                Self::new(n)
            }

            fn from_u16(n: u16) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_u32(n: u32) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_u64(n: u64) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_u128(n: u128) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_usize(n: usize) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_i8(n: i8) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_i16(n: i16) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_i32(n: i32) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_i64(n: i64) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_i128(n: i128) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }

            fn from_isize(n: isize) -> Option<Self> {
                if let Ok(n) = n.try_into() {
                    Self::from_u8(n)
                } else {
                    None
                }
            }
        }
    };
    (FromStr for $t:ty) => {
        impl ::core::str::FromStr for $t {
            type Err = ::core::num::ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::new(u8::from_str(s)?)
                    .ok_or($crate::tiny_uints::parse_int_errors::POS_OVERFLOW)
            }
        }
    };
    (Not for $t:ty) => {
        impl ::core::ops::Not for $t {
            type Output = Self;

            fn not(self) -> Self::Output {
                self ^ Self::MAX
            }
        }
    };
    (Num for $t:ty) => {
        impl $t {
            // Zero.
            pub const ZERO: Self = Self::new(0).unwrap();

            // One.
            pub const ONE: Self = Self::new(1).unwrap();
        }

        impl ::num_traits::ConstOne for $t {
            const ONE: $t = <$t>::ONE;
        }

        impl ::num_traits::ConstZero for $t {
            const ZERO: $t = <$t>::ZERO;
        }

        impl ::num_traits::Num for $t {
            type FromStrRadixErr = ::core::num::ParseIntError;

            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                Self::new(u8::from_str_radix(str, radix)?)
                    .ok_or($crate::tiny_uints::parse_int_errors::POS_OVERFLOW)
            }
        }

        impl ::num_traits::One for $t {
            fn one() -> Self {
                <Self as ::num_traits::ConstOne>::ONE
            }
        }

        impl ::num_traits::Zero for $t {
            fn zero() -> Self {
                <Self as ::num_traits::ConstZero>::ZERO
            }

            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }
    };
    (NumCast for $t:ty) => {
        impl ::num_traits::NumCast for $t {
            fn from<T: ::num_traits::ToPrimitive>(n: T) -> Option<Self> {
                <T as ::num_traits::ToPrimitive>::to_u8(&n)
                    .and_then(<Self as ::num_traits::FromPrimitive>::from_u8)
            }
        }
    };
    (ToPrimitive for $t:ty) => {
        impl ::core::convert::From<$t> for u8 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_u8()
            }
        }

        impl ::core::convert::From<$t> for u16 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_u16()
            }
        }

        impl ::core::convert::From<$t> for u32 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_u32()
            }
        }

        impl ::core::convert::From<$t> for u64 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_u64()
            }
        }

        impl ::core::convert::From<$t> for u128 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_u128()
            }
        }

        impl ::core::convert::From<$t> for usize {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_usize()
            }
        }

        impl ::core::convert::From<$t> for i8 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_i8()
            }
        }

        impl ::core::convert::From<$t> for i16 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_i16()
            }
        }

        impl ::core::convert::From<$t> for i32 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_i32()
            }
        }

        impl ::core::convert::From<$t> for i64 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_i64()
            }
        }

        impl ::core::convert::From<$t> for i128 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_i128()
            }
        }

        impl ::core::convert::From<$t> for isize {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_isize()
            }
        }

        impl ::core::convert::From<$t> for f32 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_f32()
            }
        }

        impl ::core::convert::From<$t> for f64 {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.as_f64()
            }
        }

        impl ::num_traits::ToPrimitive for $t {
            #[inline(always)]
            fn to_u8(&self) -> Option<u8> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_u16(&self) -> Option<u16> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_u32(&self) -> Option<u32> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_u64(&self) -> Option<u64> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_u128(&self) -> Option<u128> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_usize(&self) -> Option<usize> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_i8(&self) -> Option<i8> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_i16(&self) -> Option<i16> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_i32(&self) -> Option<i32> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_i64(&self) -> Option<i64> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_i128(&self) -> Option<i128> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_isize(&self) -> Option<isize> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_f32(&self) -> Option<f32> {
                Some((*self).into())
            }

            #[inline(always)]
            fn to_f64(&self) -> Option<f64> {
                Some((*self).into())
            }
        }
    };
}
use r#impl;
