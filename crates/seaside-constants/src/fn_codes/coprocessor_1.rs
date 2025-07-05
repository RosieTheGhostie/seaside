use num_derive::FromPrimitive;
use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Coprocessor1Fn {
    #[error("add")]
    Add = 0x00,
    #[error("sub")]
    Subtract = 0x01,
    #[error("mul")]
    Multiply = 0x02,
    #[error("div")]
    Divide = 0x03,
    #[error("sqrt")]
    SquareRoot = 0x04,
    #[error("abs")]
    AbsoluteValue = 0x05,
    #[error("mov")]
    Move = 0x06,
    #[error("neg")]
    Negate = 0x07,
    #[error("round.w")]
    RoundWord = 0x0c,
    #[error("trunc.w")]
    TruncateWord = 0x0d,
    #[error("ceil.w")]
    CeilingWord = 0x0e,
    #[error("floor.w")]
    FloorWord = 0x0f,
    #[error("mov")] // will be properly suffixed later
    MoveConditional = 0x11,
    #[error("movz")]
    MoveZero = 0x12,
    #[error("movn")]
    MoveNotZero = 0x13,
    #[error("cvt.s")]
    ConvertToSingle = 0x20,
    #[error("cvt.d")]
    ConvertToDouble = 0x21,
    #[error("cvt.w")]
    ConvertToWord = 0x24,
    #[error("c.eq")]
    CompareEqual = 0x32,
    #[error("c.lt")]
    CompareLessThan = 0x3c,
    #[error("c.le")]
    CompareLessEqual = 0x3e,
}
