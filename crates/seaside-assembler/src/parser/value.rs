use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

/// A scalar value.
///
/// These are used exclusively for [value array](super::Expr::ValueArray)s.
#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum Value {
    /// An integer literal.
    #[error("{0}")]
    Int(i64),
    /// A floating-point literal.
    #[error("{0}")]
    Float(f64),
}
