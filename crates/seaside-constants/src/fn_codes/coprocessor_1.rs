use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor1Fn {
    Add = 0x00,              // add.fmt
    Subtract = 0x01,         // sub.fmt
    Multiply = 0x02,         // mul.fmt
    Divide = 0x03,           // div.fmt
    SquareRoot = 0x04,       // sqrt.fmt
    AbsoluteValue = 0x05,    // abs.fmt
    Move = 0x06,             // mov.fmt
    Negate = 0x07,           // neg.fmt
    RoundWord = 0x0c,        // round.w.fmt
    TruncateWord = 0x0d,     // trunc.w.fmt
    CeilingWord = 0x0e,      // ceil.w.fmt
    FloorWord = 0x0f,        // floor.w.fmt
    MoveConditional = 0x11,  // movc.fmt
    MoveZero = 0x12,         // movz.fmt
    MoveNotZero = 0x13,      // movn.fmt
    ConvertToSingle = 0x20,  // cvt.s.fmt
    ConvertToDouble = 0x21,  // cvt.d.fmt
    ConvertToWord = 0x24,    // cvt.w.fmt
    CompareEqual = 0x32,     // c.eq.fmt
    CompareLessThan = 0x3c,  // c.lt.fmt
    CompareLessEqual = 0x3e, // c.le.fmt
}

impl Display for Coprocessor1Fn {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Coprocessor1Fn::*;
        f.write_str(match *self {
            Add => "add",
            Subtract => "sub",
            Multiply => "mul",
            Divide => "div",
            SquareRoot => "sqrt",
            AbsoluteValue => "abs",
            Move | MoveConditional => "mov", // `MoveConditional` will be properly prefixed later
            Negate => "neg",
            RoundWord => "round.w",
            TruncateWord => "trunc.w",
            CeilingWord => "ceil.w",
            FloorWord => "floor.w",
            MoveZero => "movz",
            MoveNotZero => "movn",
            ConvertToSingle => "cvt.s",
            ConvertToDouble => "cvt.d",
            ConvertToWord => "cvt.w",
            CompareEqual => "c.eq",
            CompareLessThan => "c.lt",
            CompareLessEqual => "c.le",
        })
    }
}
