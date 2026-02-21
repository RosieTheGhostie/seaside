use num_derive::FromPrimitive;
use strum::Display;

#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor1Fn {
    #[strum(to_string = "add")]
    Add = 0x00,

    #[strum(to_string = "sub")]
    Subtract = 0x01,

    #[strum(to_string = "mul")]
    Multiply = 0x02,

    #[strum(to_string = "div")]
    Divide = 0x03,

    #[strum(to_string = "sqrt")]
    SquareRoot = 0x04,

    #[strum(to_string = "abs")]
    AbsoluteValue = 0x05,

    #[strum(to_string = "mov")]
    Move = 0x06,

    #[strum(to_string = "neg")]
    Negate = 0x07,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "round.l")]
    RoundLong = 0x08,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "trunc.l")]
    TruncateLong = 0x09,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "ceil.l")]
    CeilingLong = 0x0a,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "floor.l")]
    FloorLong = 0x0b,

    #[strum(to_string = "round.w")]
    RoundWord = 0x0c,

    #[strum(to_string = "trunc.w")]
    TruncateWord = 0x0d,

    #[strum(to_string = "ceil.w")]
    CeilingWord = 0x0e,

    #[strum(to_string = "floor.w")]
    FloorWord = 0x0f,

    #[strum(to_string = "mov")] // will be properly suffixed later
    MoveConditional = 0x11,

    #[strum(to_string = "movz")]
    MoveZero = 0x12,

    #[strum(to_string = "movn")]
    MoveNotZero = 0x13,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "recip")]
    Reciprocal = 0x15,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "rsqrt")]
    ReciprocalSquareRoot = 0x16,

    #[strum(to_string = "cvt.s")]
    ConvertToSingle = 0x20,

    #[strum(to_string = "cvt.d")]
    ConvertToDouble = 0x21,

    #[strum(to_string = "cvt.w")]
    ConvertToWord = 0x24,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "cvt.l")]
    ConvertToLong = 0x25,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.f")]
    CompareFalse = 0x30,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.un")]
    CompareUnordered = 0x31,

    #[strum(to_string = "c.eq")]
    CompareEqual = 0x32,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.ueq")]
    CompareUnorderedEqual = 0x33,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.olt")]
    CompareOrderedLessThan = 0x34,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.ult")]
    CompareUnorderedLessThan = 0x35,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.ole")]
    CompareOrderedLessEqual = 0x36,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.ule")]
    CompareUnorderedLessEqual = 0x37,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.sf")]
    CompareSignalFalse = 0x38,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.ngle")]
    CompareNotGreaterLessEqual = 0x39,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.seq")]
    CompareSignalEqual = 0x3a,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.ngl")]
    CompareNotGreaterLess = 0x3b,

    #[strum(to_string = "c.lt")]
    CompareLessThan = 0x3c,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.nge")]
    CompareNotGreaterEqual = 0x3d,

    #[strum(to_string = "c.le")]
    CompareLessEqual = 0x3e,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "c.ngt")]
    CompareNotGreaterThan = 0x3f,
}

#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor1RegisterImmediateFn {
    #[strum(to_string = "mfc1")]
    MoveFromCoprocessor1 = 0x00,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dmfc1")]
    DoubleMoveFromCoprocessor1 = 0x01,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "cfc1")]
    ControlFromCoprocessor1 = 0x02,

    #[strum(to_string = "mtc1")]
    MoveToCoprocessor1 = 0x04,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dmtc1")]
    DoubleMoveToCoprocessor1 = 0x05,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "ctc1")]
    ControlToCoprocessor1 = 0x06,

    #[strum(to_string = "bc1")] // will be properly suffixed later
    BranchCoprocessor1Flag = 0x08,
}
