/// Provides a method to check whether an element is "contained" in this object.
///
/// This is usually unnecessary with standard collections, as all of those have their own `contains`
/// method not tied to a trait, but this could be helpful for your own types.
pub trait Contains<T> {
    /// Returns true if this object "contains" `value`.
    fn contains(&self, value: &T) -> bool;
}

/// Provides a method to check whether two objects are considered to "overlap".
///
/// Mostly intended for use with range-type objects, but could be used with other things as well.
pub trait Overlapping<T> {
    /// Returns true if this object "overlaps" with `value`.
    fn overlapping(&self, value: &T) -> bool;
}
