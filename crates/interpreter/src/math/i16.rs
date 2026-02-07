use super::{Division, Product, division::divmod};
use crate::Exception;

/// Computes `a + b`, wrapping at the boundaries of an [`i16`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::i16::add(2, 2), 4);
/// assert_eq!(math::i16::add(i16::MAX, 1), i16::MIN);
/// ```
pub const fn add(a: i16, b: i16) -> i16 {
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
/// assert_eq!(math::i16::add_with_overflow(2, 2), Ok(4));
/// assert_eq!(
///     math::i16::add_with_overflow(i16::MAX, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn add_with_overflow(a: i16, b: i16) -> Result<i16, Exception> {
    match a.checked_add(b) {
        Some(sum) => Ok(sum),
        None => Err(Exception::IntegerOverflowOrUnderflow),
    }
}

/// Computes `a - b`, wrapping at the boundaries of an [`i16`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::i16::sub(4, 2), 2);
/// assert_eq!(math::i16::sub(i16::MIN, 1), i16::MAX);
/// ```
pub const fn sub(a: i16, b: i16) -> i16 {
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
/// assert_eq!(math::i16::sub_with_underflow(4, 2), Ok(2));
/// assert_eq!(
///     math::i16::sub_with_underflow(i16::MIN, 1),
///     Err(Exception::IntegerOverflowOrUnderflow),
/// );
/// ```
pub const fn sub_with_underflow(a: i16, b: i16) -> Result<i16, Exception> {
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
/// assert_eq!(math::i16::mul(3, 4), 12.into());
/// assert_eq!(math::i16::mul(256, -256), (-65536).into());
/// ```
pub const fn mul(a: i16, b: i16) -> Product<i16> {
    Product::from_i32(i32::wrapping_mul(a as _, b as _))
}

/// Computes `a * b`, truncating the product to fit inside an [`i16`].
///
/// # Examples
///
/// ```
/// # use seaside_interpreter::math;
/// assert_eq!(math::i16::mul_with_truncation(3, 4), 12);
/// assert_eq!(math::i16::mul_with_truncation(256, -256), 0);
/// ```
pub const fn mul_with_truncation(a: i16, b: i16) -> i16 {
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
///     math::i16::divmod(12, 3),
///     Some(Division {
///         quotient: 4,
///         remainder: 0,
///     }),
/// );
/// assert_eq!(
///     math::i16::divmod(69, -4),
///     // -17 * -4 + 1 == 69
///     Some(Division {
///         quotient: -17,
///         remainder: 1,
///     }),
/// );
/// assert_eq!(math::i16::divmod(1, 0), None);
/// ```
pub const fn divmod(a: i16, b: i16) -> Option<Division<i16>> {
    if b != 0 { Some(divmod!(a, b)) } else { None }
}
