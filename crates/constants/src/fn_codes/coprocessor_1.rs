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

    #[cfg(feature = "unimplemented")]
    #[error("round.l")]
    RoundLong = 0x08,

    #[cfg(feature = "unimplemented")]
    #[error("trunc.l")]
    TruncateLong = 0x09,

    #[cfg(feature = "unimplemented")]
    #[error("ceil.l")]
    CeilingLong = 0x0a,

    #[cfg(feature = "unimplemented")]
    #[error("floor.l")]
    FloorLong = 0x0b,

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

    #[cfg(feature = "unimplemented")]
    #[error("recip")]
    Reciprocal = 0x15,

    #[cfg(feature = "unimplemented")]
    #[error("rsqrt")]
    ReciprocalSquareRoot = 0x16,

    #[error("cvt.s")]
    ConvertToSingle = 0x20,

    #[error("cvt.d")]
    ConvertToDouble = 0x21,

    #[error("cvt.w")]
    ConvertToWord = 0x24,

    #[cfg(feature = "unimplemented")]
    #[error("cvt.l")]
    ConvertToLong = 0x25,

    #[cfg(feature = "unimplemented")]
    #[error("c.f")]
    CompareFalse = 0x30,

    #[cfg(feature = "unimplemented")]
    #[error("c.un")]
    CompareUnordered = 0x31,

    #[error("c.eq")]
    CompareEqual = 0x32,

    #[cfg(feature = "unimplemented")]
    #[error("c.ueq")]
    CompareUnorderedEqual = 0x33,

    #[cfg(feature = "unimplemented")]
    #[error("c.olt")]
    CompareOrderedLessThan = 0x34,

    #[cfg(feature = "unimplemented")]
    #[error("c.ult")]
    CompareUnorderedLessThan = 0x35,

    #[cfg(feature = "unimplemented")]
    #[error("c.ole")]
    CompareOrderedLessEqual = 0x36,

    #[cfg(feature = "unimplemented")]
    #[error("c.ule")]
    CompareUnorderedLessEqual = 0x37,

    #[cfg(feature = "unimplemented")]
    #[error("c.sf")]
    CompareSignalFalse = 0x38,

    #[cfg(feature = "unimplemented")]
    #[error("c.ngle")]
    CompareNotGreaterLessEqual = 0x39,

    #[cfg(feature = "unimplemented")]
    #[error("c.seq")]
    CompareSignalEqual = 0x3a,

    #[cfg(feature = "unimplemented")]
    #[error("c.ngl")]
    CompareNotGreaterLess = 0x3b,

    #[error("c.lt")]
    CompareLessThan = 0x3c,

    #[cfg(feature = "unimplemented")]
    #[error("c.nge")]
    CompareNotGreaterEqual = 0x3d,

    #[error("c.le")]
    CompareLessEqual = 0x3e,

    #[cfg(feature = "unimplemented")]
    #[error("c.ngt")]
    CompareNotGreaterThan = 0x3f,
}

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Coprocessor1RegisterImmediateFn {
    #[error("mfc1")]
    MoveFromCoprocessor1 = 0x00,

    #[cfg(feature = "unimplemented")]
    #[error("dmfc1")]
    DoubleMoveFromCoprocessor1 = 0x01,

    #[cfg(feature = "unimplemented")]
    #[error("cfc1")]
    ControlFromCoprocessor1 = 0x02,

    #[error("mtc1")]
    MoveToCoprocessor1 = 0x04,

    #[cfg(feature = "unimplemented")]
    #[error("dmtc1")]
    DoubleMoveToCoprocessor1 = 0x05,

    #[cfg(feature = "unimplemented")]
    #[error("ctc1")]
    ControlToCoprocessor1 = 0x06,

    #[error("bc1")] // will be properly suffixed later
    BranchCoprocessor1Flag = 0x08,
}
