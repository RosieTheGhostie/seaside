use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

/// An operand in an [instruction](super::Expr::Instruction).
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Operand<'src> {
    /// An integer literal.
    #[error("{0}")]
    Int(i64),
    /// The name or index of a hardware register.
    ///
    /// The string slice does not include the preceding `$`.
    #[error("${0}")]
    Register(&'src str),
    /// The name or index of a hardware register wrapped in parentheses.
    ///
    /// The string slice does not include the parentheses nor the preceding `$`.
    #[error("(${0})")]
    WrappedRegister(&'src str),
    /// The name of a label.
    #[error("{0}")]
    Label(&'src str),
}
