use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor0Fn {
    MoveFromCoprocessor0 = 0x00, // mfc0
    MoveToCoprocessor0 = 0x04,   // mtc0
    ErrorReturn = 0x10,          // eret
}

impl Display for Coprocessor0Fn {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Coprocessor0Fn::*;
        f.write_str(match *self {
            MoveFromCoprocessor0 => "mfc0",
            MoveToCoprocessor0 => "mtc0",
            ErrorReturn => "eret",
        })
    }
}
