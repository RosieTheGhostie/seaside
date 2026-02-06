use strum::Display;

/// A scalar value.
///
/// These are used exclusively for [value array](super::Expr::ValueArray)s.
#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Value {
    /// An integer literal.
    #[strum(to_string = "{0}")]
    Int(i64),

    /// A floating-point literal.
    #[strum(to_string = "{0}")]
    Float(f64),
}
