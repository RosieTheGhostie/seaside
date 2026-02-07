use super::{Division, Product, division::divmod};
use crate::Exception;

/// Computes `a + b`, wrapping at the boundaries of a [`u8`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::add(2, 2), 4);
/// assert_eq!(math::u8::add(u8::MAX, 1), u8::MIN);
/// ```
pub const fn add(a: u8, b: u8) -> u8 {
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
/// assert_eq!(math::u8::add_with_overflow(2, 2), Ok(4));
/// assert_eq!(
///     math::u8::add_with_overflow(u8::MAX, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn add_with_overflow(a: u8, b: u8) -> Result<u8, Exception> {
    match a.checked_add(b) {
        Some(sum) => Ok(sum),
        None => Err(Exception::IntegerOverflowOrUnderflow),
    }
}

/// Computes `x + offset`, wrapping at the boundaries of a [`u8`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::offset(2, 2), 4);
/// assert_eq!(math::u8::offset(u8::MAX, 1), u8::MIN);
/// assert_eq!(math::u8::offset(u8::MIN, -1), u8::MAX);
/// ```
pub const fn offset(x: u8, offset: i8) -> u8 {
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
/// assert_eq!(math::u8::offset_with_overflow(2, 2), Ok(4));
/// assert_eq!(
///     math::u8::offset_with_overflow(u8::MAX, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// assert_eq!(
///     math::u8::offset_with_overflow(u8::MIN, -1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn offset_with_overflow(x: u8, offset: i8) -> Result<u8, Exception> {
    match x.checked_add_signed(offset) {
        Some(sum) => Ok(sum),
        None => Err(Exception::IntegerOverflowOrUnderflow),
    }
}

/// Computes `a - b`, wrapping at the boundaries of a [`u8`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::sub(4, 2), 2);
/// assert_eq!(math::u8::sub(u8::MIN, 1), u8::MAX);
/// ```
pub const fn sub(a: u8, b: u8) -> u8 {
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
/// assert_eq!(math::u8::sub_with_underflow(4, 2), Ok(2));
/// assert_eq!(
///     math::u8::sub_with_underflow(u8::MIN, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn sub_with_underflow(a: u8, b: u8) -> Result<u8, Exception> {
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
/// assert_eq!(math::u8::mul(3, 4), 12.into());
/// assert_eq!(math::u8::mul(16, 16), 256.into());
/// ```
pub const fn mul(a: u8, b: u8) -> Product<u8> {
    Product::from_u16(u16::wrapping_mul(a as _, b as _))
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
///     math::u8::divmod(12, 3),
///     Some(Division {
///         quotient: 4,
///         remainder: 0,
///     }),
/// );
/// assert_eq!(
///     math::u8::divmod(69, 4),
///     Some(Division {
///         quotient: 17,
///         remainder: 1,
///     }),
/// );
/// assert_eq!(math::u8::divmod(1, 0), None);
/// ```
pub const fn divmod(a: u8, b: u8) -> Option<Division<u8>> {
    if b != 0 { Some(divmod!(a, b)) } else { None }
}

/// Computes the bitwise AND of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::and(0x12, 0xf0), 0x10);
/// assert_eq!(math::u8::and(0x78, 0x87), 0x00);
/// ```
#[inline(always)]
pub const fn and(a: u8, b: u8) -> u8 {
    a & b
}

/// Computes the bitwise OR of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::or(0x12, 0xf0), 0xf2);
/// assert_eq!(math::u8::or(0x78, 0x87), 0xff);
/// ```
#[inline(always)]
pub const fn or(a: u8, b: u8) -> u8 {
    a | b
}

/// Computes the bitwise XOR of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::xor(0x12, 0xf0), 0xe2);
/// assert_eq!(math::u8::xor(0x78, 0x87), 0xff);
/// ```
#[inline(always)]
pub const fn xor(a: u8, b: u8) -> u8 {
    a ^ b
}

/// Computes the bitwise NOT of `a`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::not(0x12), 0xed);
/// assert_eq!(math::u8::not(0x00), 0xff);
/// assert_eq!(math::u8::not(0xff), 0x00);
/// ```
#[inline(always)]
pub const fn not(a: u8) -> u8 {
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
/// assert_eq!(math::u8::nand(0x12, 0xf0), 0xef);
/// assert_eq!(math::u8::nand(0x78, 0x87), 0xff);
/// ```
#[inline(always)]
pub const fn nand(a: u8, b: u8) -> u8 {
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
/// assert_eq!(math::u8::nor(0x12, 0xf0), 0x0d);
/// assert_eq!(math::u8::nor(0x78, 0x87), 0x00);
/// ```
#[inline(always)]
pub const fn nor(a: u8, b: u8) -> u8 {
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
/// assert_eq!(math::u8::xnor(0x12, 0xf0), 0x1d);
/// assert_eq!(math::u8::xnor(0x78, 0x87), 0x00);
/// ```
#[inline(always)]
pub const fn xnor(a: u8, b: u8) -> u8 {
    not(xor(a, b))
}

/// Shifts the bits of `x` left by `n` bits.
///
/// Any bits that end up beyond the bounds of a [`u8`] are lost.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::shift_left(0x12, 3), 0x90);
/// assert_eq!(math::u8::shift_left(0xff, 6), 0xc0);
/// ```
#[inline(always)]
pub const fn shift_left(x: u8, n: u8) -> u8 {
    x << n
}

/// Shifts the bits of `x` right by `n` bits, filling the leftmost `n` bits with zeroes.
///
/// Any bits that end up beyond the bounds of a [`u8`] are lost.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::shift_right(0x12, 3), 0x02);
/// assert_eq!(math::u8::shift_right(0xff, 6), 0x03);
/// ```
#[inline(always)]
pub const fn shift_right(x: u8, n: u8) -> u8 {
    x >> n
}

/// Shifts the bits of `x` right by `n` bits, filling the leftmost `n` bits with the most
/// significant bit of `x`.
///
/// Any bits that end up beyond the bounds of a [`u8`] are lost.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u8::shift_right_arithmetic(0x12, 3), 0x02);
/// assert_eq!(math::u8::shift_right_arithmetic(0xff, 6), 0xff);
/// ```
#[inline(always)]
pub const fn shift_right_arithmetic(x: u8, n: u8) -> u8 {
    (x.cast_signed() >> n) as _
}
