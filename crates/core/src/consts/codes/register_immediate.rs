use num_derive::FromPrimitive;
use strum::Display;

#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum RegisterImmediateFn {
    #[strum(to_string = "bltz")]
    BranchLessThanZero = 0x00,

    #[strum(to_string = "bgez")]
    BranchGreaterEqualZero = 0x01,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "bltzl")]
    BranchLessThanZeroLikely = 0x02,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "bgezl")]
    BranchGreaterEqualZeroLikely = 0x03,

    #[strum(to_string = "tgei")]
    TrapGreaterEqualImmediate = 0x08,

    #[strum(to_string = "tgeiu")]
    TrapGreaterEqualImmediateUnsigned = 0x09,

    #[strum(to_string = "tlti")]
    TrapLessThanImmediate = 0x0a,

    #[strum(to_string = "tltiu")]
    TrapLessThanImmediateUnsigned = 0x0b,

    #[strum(to_string = "teqi")]
    TrapEqualImmediate = 0x0c,

    #[strum(to_string = "tnei")]
    TrapNotEqualImmediate = 0x0e,

    #[strum(to_string = "bltzal")]
    BranchLessThanZeroAndLink = 0x10,

    #[strum(to_string = "bgezal")]
    BranchGreaterEqualZeroAndLink = 0x11,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "bltzall")]
    BranchLessThanZeroAndLinkLikely = 0x12,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "bgezall")]
    BranchGreaterEqualZeroAndLinkLikely = 0x13,
}
