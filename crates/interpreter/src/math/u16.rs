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
