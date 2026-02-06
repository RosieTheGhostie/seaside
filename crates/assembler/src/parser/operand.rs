use strum::Display;

/// An operand in an [instruction](super::Expr::Instruction).
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum Operand<'src> {
    /// An integer literal.
    #[strum(to_string = "{0}")]
    Int(i64),

    /// The name or index of a hardware register.
    ///
    /// The string slice does not include the preceding `$`.
    #[strum(to_string = "${0}")]
    Register(&'src str),

    /// The name or index of a hardware register wrapped in parentheses.
    ///
    /// The string slice does not include the parentheses nor the preceding `$`.
    #[strum(to_string = "(${0})")]
    WrappedRegister(&'src str),

    /// The name of a label.
    #[strum(to_string = "{0}")]
    Label(&'src str),
}
