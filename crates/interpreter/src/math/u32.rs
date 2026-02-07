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
