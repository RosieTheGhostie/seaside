use super::{Division, Product, division::divmod};
use crate::Exception;

/// Computes `a + b`, wrapping at the boundaries of a [`u32`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::add(2, 2), 4);
/// assert_eq!(math::u32::add(u32::MAX, 1), u32::MIN);
/// ```
pub const fn add(a: u32, b: u32) -> u32 {
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
/// assert_eq!(math::u32::add_with_overflow(2, 2), Ok(4));
/// assert_eq!(
///     math::u32::add_with_overflow(u32::MAX, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn add_with_overflow(a: u32, b: u32) -> Result<u32, Exception> {
    match a.checked_add(b) {
        Some(sum) => Ok(sum),
        None => Err(Exception::IntegerOverflowOrUnderflow),
    }
}

/// Computes `x + offset`, wrapping at the boundaries of a [`u32`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::offset(2, 2), 4);
/// assert_eq!(math::u32::offset(u32::MAX, 1), u32::MIN);
/// assert_eq!(math::u32::offset(u32::MIN, -1), u32::MAX);
/// ```
pub const fn offset(x: u32, offset: i32) -> u32 {
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
/// assert_eq!(math::u32::offset_with_overflow(2, 2), Ok(4));
/// assert_eq!(
///     math::u32::offset_with_overflow(u32::MAX, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// assert_eq!(
///     math::u32::offset_with_overflow(u32::MIN, -1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn offset_with_overflow(x: u32, offset: i32) -> Result<u32, Exception> {
    match x.checked_add_signed(offset) {
        Some(sum) => Ok(sum),
        None => Err(Exception::IntegerOverflowOrUnderflow),
    }
}

/// Computes `a - b`, wrapping at the boundaries of a [`u32`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::sub(4, 2), 2);
/// assert_eq!(math::u32::sub(u32::MIN, 1), u32::MAX);
/// ```
pub const fn sub(a: u32, b: u32) -> u32 {
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
/// assert_eq!(math::u32::sub_with_underflow(4, 2), Ok(2));
/// assert_eq!(
///     math::u32::sub_with_underflow(u32::MIN, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn sub_with_underflow(a: u32, b: u32) -> Result<u32, Exception> {
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
/// assert_eq!(math::u32::mul(3, 4), 12.into());
/// assert_eq!(math::u32::mul(65_536, 65_536), 4_294_967_296.into());
/// ```
pub const fn mul(a: u32, b: u32) -> Product<u32> {
    Product::from_u64(u64::wrapping_mul(a as _, b as _))
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
///     math::u32::divmod(12, 3),
///     Some(Division {
///         quotient: 4,
///         remainder: 0,
///     }),
/// );
/// assert_eq!(
///     math::u32::divmod(69, 4),
///     Some(Division {
///         quotient: 17,
///         remainder: 1,
///     }),
/// );
/// assert_eq!(math::u32::divmod(1, 0), None);
/// ```
pub const fn divmod(a: u32, b: u32) -> Option<Division<u32>> {
    if b != 0 { Some(divmod!(a, b)) } else { None }
}

/// Computes the bitwise AND of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::and(0x1234_5678, 0xf0f0_f0f0), 0x1030_5070);
/// assert_eq!(math::u32::and(0x1234_5678, 0xedcb_a987), 0x0000_0000);
/// ```
#[inline(always)]
pub const fn and(a: u32, b: u32) -> u32 {
    a & b
}

/// Computes the bitwise OR of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::or(0x1234_5678, 0xf0f0_f0f0), 0xf2f4_f6f8);
/// assert_eq!(math::u32::or(0x1234_5678, 0xedcb_a987), 0xffff_ffff);
/// ```
#[inline(always)]
pub const fn or(a: u32, b: u32) -> u32 {
    a | b
}

/// Computes the bitwise XOR of `a` and `b`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::xor(0x1234_5678, 0xf0f0_f0f0), 0xe2c4_a688);
/// assert_eq!(math::u32::xor(0x1234_5678, 0xedcb_a987), 0xffff_ffff);
/// ```
#[inline(always)]
pub const fn xor(a: u32, b: u32) -> u32 {
    a ^ b
}

/// Computes the bitwise NOT of `a`.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::not(0x1234_5678), 0xedcb_a987);
/// assert_eq!(math::u32::not(0x0000_0000), 0xffff_ffff);
/// assert_eq!(math::u32::not(0xffff_ffff), 0x0000_0000);
/// ```
#[inline(always)]
pub const fn not(a: u32) -> u32 {
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
/// assert_eq!(math::u32::nand(0x1234_5678, 0xf0f0_f0f0), 0xefcf_af8f);
/// assert_eq!(math::u32::nand(0x1234_5678, 0xedcb_a987), 0xffff_ffff);
/// ```
#[inline(always)]
pub const fn nand(a: u32, b: u32) -> u32 {
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
/// assert_eq!(math::u32::nor(0x1234_5678, 0xf0f0_f0f0), 0x0d0b_0907);
/// assert_eq!(math::u32::nor(0x1234_5678, 0xedcb_a987), 0x0000_0000);
/// ```
#[inline(always)]
pub const fn nor(a: u32, b: u32) -> u32 {
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
/// assert_eq!(math::u32::xnor(0x1234_5678, 0xf0f0_f0f0), 0x1d3b_5977);
/// assert_eq!(math::u32::xnor(0x1234_5678, 0xedcb_a987), 0x0000_0000);
/// ```
#[inline(always)]
pub const fn xnor(a: u32, b: u32) -> u32 {
    not(xor(a, b))
}

/// Shifts the bits of `x` left by `n` bits.
///
/// Any bits that end up beyond the bounds of a [`u32`] are lost.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::shift_left(0x1234_5678, 3), 0x91a2_b3c0);
/// assert_eq!(math::u32::shift_left(0xffff_ffff, 12), 0xffff_f000);
/// ```
#[inline(always)]
pub const fn shift_left(x: u32, n: u32) -> u32 {
    x << n
}

/// Shifts the bits of `x` right by `n` bits, filling the leftmost `n` bits with zeroes.
///
/// Any bits that end up beyond the bounds of a [`u32`] are lost.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::shift_right(0x1234_5678, 3), 0x0246_8acf);
/// assert_eq!(math::u32::shift_right(0xffff_ffff, 12), 0x000f_ffff);
/// ```
#[inline(always)]
pub const fn shift_right(x: u32, n: u32) -> u32 {
    x >> n
}

/// Shifts the bits of `x` right by `n` bits, filling the leftmost `n` bits with the most
/// significant bit of `x`.
///
/// Any bits that end up beyond the bounds of a [`u32`] are lost.
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::u32::shift_right_arithmetic(0x1234_5678, 3), 0x0246_8acf);
/// assert_eq!(math::u32::shift_right_arithmetic(0xffff_ffff, 12), 0xffff_ffff);
/// ```
#[inline(always)]
pub const fn shift_right_arithmetic(x: u32, n: u32) -> u32 {
    (x.cast_signed() >> n) as _
}
