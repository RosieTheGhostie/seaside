use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum RegisterImmediateFn {
    BranchLessThanZero = 0x00,                // BLTZ
    BranchGreaterEqualZero = 0x01,            // BGEZ
    TrapGreaterEqualImmediate = 0x08,         // TGEI
    TrapGreaterEqualImmediateUnsigned = 0x09, // TGEIU
    TrapLessThanImmediate = 0x0A,             // TLTI
    TrapLessThanImmediateUnsigned = 0x0B,     // TLTIU
    TrapEqualImmediate = 0x0C,                // TEQI
    TrapNotEqualImmediate = 0x0E,             // TNEI
    BranchLessThanZeroAndLink = 0x10,         // BLTZAL
    BranchGreaterEqualZeroAndLink = 0x11,     // BGEZAL
}
