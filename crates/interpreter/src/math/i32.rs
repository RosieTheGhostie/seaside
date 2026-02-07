use super::{Division, Product, division::divmod};
use crate::Exception;

/// Computes `a + b`, wrapping at the boundaries of an [`i32`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::i32::add(2, 2), 4);
/// assert_eq!(math::i32::add(i32::MAX, 1), i32::MIN);
/// ```
pub const fn add(a: i32, b: i32) -> i32 {
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
/// assert_eq!(math::i32::add_with_overflow(2, 2), Ok(4));
/// assert_eq!(
///     math::i32::add_with_overflow(i32::MAX, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn add_with_overflow(a: i32, b: i32) -> Result<i32, Exception> {
    match a.checked_add(b) {
        Some(sum) => Ok(sum),
        None => Err(Exception::IntegerOverflowOrUnderflow),
    }
}

/// Computes `a - b`, wrapping at the boundaries of an [`i32`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::i32::sub(4, 2), 2);
/// assert_eq!(math::i32::sub(i32::MIN, 1), i32::MAX);
/// ```
pub const fn sub(a: i32, b: i32) -> i32 {
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
/// assert_eq!(math::i32::sub_with_underflow(4, 2), Ok(2));
/// assert_eq!(
///     math::i32::sub_with_underflow(i32::MIN, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn sub_with_underflow(a: i32, b: i32) -> Result<i32, Exception> {
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
/// assert_eq!(math::i32::mul(3, 4), 12.into());
/// assert_eq!(math::i32::mul(65_536, -65_536), (-4_294_967_296).into());
/// ```
pub const fn mul(a: i32, b: i32) -> Product<i32> {
    Product::from_i64(i64::wrapping_mul(a as _, b as _))
}

/// Computes `a * b`, truncating the product to fit inside an [`i32`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::i32::mul_with_truncation(3, 4), 12);
/// assert_eq!(math::i32::mul_with_truncation(65_536, -65_536), 0);
/// ```
pub const fn mul_with_truncation(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
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
///     math::i32::divmod(12, 3),
///     Some(Division {
///         quotient: 4,
///         remainder: 0,
///     }),
/// );
/// assert_eq!(
///     math::i32::divmod(69, -4),
///     // -17 * -4 + 1 == 69
///     Some(Division {
///         quotient: -17,
///         remainder: 1,
///     }),
/// );
/// assert_eq!(math::i32::divmod(1, 0), None);
/// ```
pub const fn divmod(a: i32, b: i32) -> Option<Division<i32>> {
    if b != 0 { Some(divmod!(a, b)) } else { None }
}
