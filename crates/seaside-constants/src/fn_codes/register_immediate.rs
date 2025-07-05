use num_derive::FromPrimitive;
use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum RegisterImmediateFn {
    #[error("bltz")]
    BranchLessThanZero = 0x00,
    #[error("bgez")]
    BranchGreaterEqualZero = 0x01,
    #[error("tgei")]
    TrapGreaterEqualImmediate = 0x08,
    #[error("tgeiu")]
    TrapGreaterEqualImmediateUnsigned = 0x09,
    #[error("tlti")]
    TrapLessThanImmediate = 0x0a,
    #[error("tltiu")]
    TrapLessThanImmediateUnsigned = 0x0b,
    #[error("teqi")]
    TrapEqualImmediate = 0x0c,
    #[error("tnei")]
    TrapNotEqualImmediate = 0x0e,
    #[error("bltzal")]
    BranchLessThanZeroAndLink = 0x10,
    #[error("bgezal")]
    BranchGreaterEqualZeroAndLink = 0x11,
}
