use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum RegisterImmediateFn {
    BranchLessThanZero = 0x00,                // bltz
    BranchGreaterEqualZero = 0x01,            // bgez
    TrapGreaterEqualImmediate = 0x08,         // tgei
    TrapGreaterEqualImmediateUnsigned = 0x09, // tgeiu
    TrapLessThanImmediate = 0x0a,             // tlti
    TrapLessThanImmediateUnsigned = 0x0b,     // tltiu
    TrapEqualImmediate = 0x0c,                // teqi
    TrapNotEqualImmediate = 0x0e,             // tnei
    BranchLessThanZeroAndLink = 0x10,         // bltzal
    BranchGreaterEqualZeroAndLink = 0x11,     // bgezal
}

impl Display for RegisterImmediateFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use RegisterImmediateFn::*;
        f.write_str(match *self {
            BranchLessThanZero => "bltz",
            BranchGreaterEqualZero => "bgez",
            TrapGreaterEqualImmediate => "tgei",
            TrapGreaterEqualImmediateUnsigned => "tgeiu",
            TrapLessThanImmediate => "tlti",
            TrapLessThanImmediateUnsigned => "tltiu",
            TrapEqualImmediate => "teqi",
            TrapNotEqualImmediate => "tnei",
            BranchLessThanZeroAndLink => "bltzal",
            BranchGreaterEqualZeroAndLink => "bgezal",
        })
    }
}
