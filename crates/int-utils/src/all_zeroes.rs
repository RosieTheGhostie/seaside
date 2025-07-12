//! When you want a default that doesn't necessarily make sense.
//!
//! Provides the trait [`AllZeroes`], which provides a factory method for making a "zero" object.

/// Provides a factory method for making a "zero" object.
pub trait AllZeroes {
    /// Returns a "zero" object.
    ///
    /// "Zero" objects are defined as objects with all their fields set to some sort of zero value.
    fn all_zeroes() -> Self;
}
