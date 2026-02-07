use super::{Division, Product, division::divmod};
use crate::Exception;

/// Computes `a + b`, wrapping at the boundaries of a [`u16`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::add(2, 2), 4);
/// assert_eq!(math::u16::add(u16::MAX, 1), u16::MIN);
/// ```
pub const fn add(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}

/// Computes `a + b`.
///
/// # Errors
///
/// TODO
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::{Exception, math};
/// assert_eq!(math::u16::add_with_overflow(2, 2), Ok(4));
/// assert_eq!(
///     math::u16::add_with_overflow(u16::MAX, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn add_with_overflow(a: u16, b: u16) -> Result<u16, Exception> {
    match a.checked_add(b) {
        Some(sum) => Ok(sum),
        None => Err(Exception::IntegerOverflowOrUnderflow),
    }
}

/// Computes `x + offset`, wrapping at the boundaries of a [`u16`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::offset(2, 2), 4);
/// assert_eq!(math::u16::offset(u16::MAX, 1), u16::MIN);
/// assert_eq!(math::u16::offset(u16::MIN, -1), u16::MAX);
/// ```
pub const fn offset(x: u16, offset: i16) -> u16 {
    x.wrapping_add_signed(offset)
}

/// Computes `x + offset`.
///
/// # Errors
///
/// TODO
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::{Exception, math};
/// assert_eq!(math::u16::offset_with_overflow(2, 2), Ok(4));
/// assert_eq!(
///     math::u16::offset_with_overflow(u16::MAX, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// assert_eq!(
///     math::u16::offset_with_overflow(u16::MIN, -1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn offset_with_overflow(x: u16, offset: i16) -> Result<u16, Exception> {
    match x.checked_add_signed(offset) {
        Some(sum) => Ok(sum),
        None => Err(Exception::IntegerOverflowOrUnderflow),
    }
}

/// Computes `a - b`, wrapping at the boundaries of a [`u16`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::sub(4, 2), 2);
/// assert_eq!(math::u16::sub(u16::MIN, 1), u16::MAX);
/// ```
pub const fn sub(a: u16, b: u16) -> u16 {
    a.wrapping_sub(b)
}

/// Computes `a - b`.
///
/// # Errors
///
/// TODO
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::{Exception, math};
/// assert_eq!(math::u16::sub_with_underflow(4, 2), Ok(2));
/// assert_eq!(
///     math::u16::sub_with_underflow(u16::MIN, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn sub_with_underflow(a: u16, b: u16) -> Result<u16, Exception> {
    match a.checked_sub(b) {
        Some(difference) => Ok(difference),
        None => Err(Exception::IntegerOverflowOrUnderflow),
    }
}

/// Computes `a * b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::mul(3, 4), 12.into());
/// assert_eq!(math::u16::mul(256, 256), 65_536.into());
/// ```
pub const fn mul(a: u16, b: u16) -> Product<u16> {
    Product::from_u32(u32::wrapping_mul(a as _, b as _))
}

/// Computes both `a / b` and `a % b`.
///
/// This will return [`None`] if `b == 0`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math::{self, Division};
/// assert_eq!(
///     math::u16::divmod(12, 3),
///     Some(Division {
///         quotient: 4,
///         remainder: 0,
///     }),
/// );
/// assert_eq!(
///     math::u16::divmod(69, 4),
///     Some(Division {
///         quotient: 17,
///         remainder: 1,
///     }),
/// );
/// assert_eq!(math::u16::divmod(1, 0), None);
/// ```
pub const fn divmod(a: u16, b: u16) -> Option<Division<u16>> {
    if b != 0 { Some(divmod!(a, b)) } else { None }
}

/// Computes the bitwise AND of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::and(0x1234, 0xf0f0), 0x1030);
/// assert_eq!(math::u16::and(0x5678, 0xa987), 0x0000);
/// ```
#[inline(always)]
pub const fn and(a: u16, b: u16) -> u16 {
    a & b
}

/// Computes the bitwise OR of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::or(0x1234, 0xf0f0), 0xf2f4);
/// assert_eq!(math::u16::or(0x5678, 0xa987), 0xffff);
/// ```
#[inline(always)]
pub const fn or(a: u16, b: u16) -> u16 {
    a | b
}

/// Computes the bitwise XOR of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::xor(0x1234, 0xf0f0), 0xe2c4);
/// assert_eq!(math::u16::xor(0x5678, 0xa987), 0xffff);
/// ```
#[inline(always)]
pub const fn xor(a: u16, b: u16) -> u16 {
    a ^ b
}

/// Computes the bitwise NOT of `a`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::not(0x1234), 0xedcb);
/// assert_eq!(math::u16::not(0x0000), 0xffff);
/// assert_eq!(math::u16::not(0xffff), 0x0000);
/// ```
#[inline(always)]
pub const fn not(a: u16) -> u16 {
    !a
}

/// Computes the bitwise NAND of `a` and `b`.
///
/// This is equivalent to [negating](not) the [bitwise AND](and) of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::nand(0x1234, 0xf0f0), 0xefcf);
/// assert_eq!(math::u16::nand(0x5678, 0xa987), 0xffff);
/// ```
#[inline(always)]
pub const fn nand(a: u16, b: u16) -> u16 {
    not(and(a, b))
}

/// Computes the bitwise NOR of `a` and `b`.
///
/// This is equivalent to [negating](not) the [bitwise OR](or) of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::nor(0x1234, 0xf0f0), 0x0d0b);
/// assert_eq!(math::u16::nor(0x5678, 0xa987), 0x0000);
/// ```
#[inline(always)]
pub const fn nor(a: u16, b: u16) -> u16 {
    not(or(a, b))
}

/// Computes the bitwise XNOR of `a` and `b`.
///
/// This is equivalent to [negating](not) the [bitwise XOR](xor) of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::xnor(0x1234, 0xf0f0), 0x1d3b);
/// assert_eq!(math::u16::xnor(0x5678, 0xa987), 0x0000);
/// ```
#[inline(always)]
pub const fn xnor(a: u16, b: u16) -> u16 {
    not(xor(a, b))
}

/// Shifts the bits of `x` left by `n` bits.
///
/// Any bits that end up beyond the bounds of a [`u16`] are lost.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::shift_left(0x1234, 3), 0x91a0);
/// assert_eq!(math::u16::shift_left(0xffff, 12), 0xf000);
/// ```
#[inline(always)]
pub const fn shift_left(x: u16, n: u16) -> u16 {
    x << n
}

/// Shifts the bits of `x` right by `n` bits, filling the leftmost `n` bits with zeroes.
///
/// Any bits that end up beyond the bounds of a [`u16`] are lost.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::shift_right(0x1234, 3), 0x0246);
/// assert_eq!(math::u16::shift_right(0xffff, 12), 0x000f);
/// ```
#[inline(always)]
pub const fn shift_right(x: u16, n: u16) -> u16 {
    x >> n
}

/// Shifts the bits of `x` right by `n` bits, filling the leftmost `n` bits with the most
/// significant bit of `x`.
///
/// Any bits that end up beyond the bounds of a [`u16`] are lost.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u16::shift_right_arithmetic(0x1234, 3), 0x0246);
/// assert_eq!(math::u16::shift_right_arithmetic(0xffff, 12), 0xffff);
/// ```
#[inline(always)]
pub const fn shift_right_arithmetic(x: u16, n: u16) -> u16 {
    (x.cast_signed() >> n) as _
}
