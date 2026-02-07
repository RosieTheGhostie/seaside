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
