use num_derive::FromPrimitive;

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
