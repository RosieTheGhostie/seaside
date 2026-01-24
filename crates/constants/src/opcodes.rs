use num_derive::FromPrimitive;
use strum::Display;

#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum Opcode {
    #[strum(to_string = "<special fn>")]
    Special = 0x00,

    #[strum(to_string = "<register immediate fn>")]
    RegisterImmediate = 0x01,

    #[strum(to_string = "j")]
    Jump = 0x02,

    #[strum(to_string = "jal")]
    JumpAndLink = 0x03,

    #[strum(to_string = "beq")]
    BranchEqual = 0x04,

    #[strum(to_string = "bne")]
    BranchNotEqual = 0x05,

    #[strum(to_string = "blez")]
    BranchLessEqualZero = 0x06,

    #[strum(to_string = "bgtz")]
    BranchGreaterThanZero = 0x07,

    #[strum(to_string = "addi")]
    AddImmediate = 0x08,

    #[strum(to_string = "addiu")]
    AddImmediateUnsigned = 0x09,

    #[strum(to_string = "slti")]
    SetLessThanImmediate = 0x0a,

    #[strum(to_string = "sltiu")]
    SetLessThanImmediateUnsigned = 0x0b,

    #[strum(to_string = "andi")]
    AndImmediate = 0x0c,

    #[strum(to_string = "ori")]
    OrImmediate = 0x0d,

    #[strum(to_string = "xori")]
    XorImmediate = 0x0e,

    #[strum(to_string = "lui")]
    LoadUpperImmediate = 0x0f,

    #[strum(to_string = "<coprocessor 0 fn>")]
    Coprocessor0 = 0x10,

    #[strum(to_string = "<coprocessor 1 fn>")]
    Coprocessor1 = 0x11,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "<coprocessor 2 fn>")]
    Coprocessor2 = 0x12,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "<coprocessor 1x fn>")]
    Coprocessor1X = 0x13,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "beql")]
    BranchEqualLikely = 0x14,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "bnel")]
    BranchNotEqualLikely = 0x15,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "blezl")]
    BranchLessEqualZeroLikely = 0x16,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "bgtzl")]
    BranchGreaterThanZeroLikely = 0x17,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "daddi")]
    DoubleAddImmediate = 0x18,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "daddiu")]
    DoubleAddImmediateUnsigned = 0x19,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "ldl")]
    LoadDoubleLeft = 0x1a,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "ldr")]
    LoadDoubleRight = 0x1b,

    #[strum(to_string = "<special 2 fn>")]
    Special2 = 0x1c,

    #[strum(to_string = "lb")]
    LoadByte = 0x20,

    #[strum(to_string = "lh")]
    LoadHalf = 0x21,

    #[strum(to_string = "lwl")]
    LoadWordLeft = 0x22,

    #[strum(to_string = "lw")]
    LoadWord = 0x23,

    #[strum(to_string = "lbu")]
    LoadByteUnsigned = 0x24,

    #[strum(to_string = "lhu")]
    LoadHalfUnsigned = 0x25,

    #[strum(to_string = "lwr")]
    LoadWordRight = 0x26,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "lwu")]
    LoadWordUnsigned = 0x27,

    #[strum(to_string = "sb")]
    StoreByte = 0x28,

    #[strum(to_string = "sh")]
    StoreHalf = 0x29,

    #[strum(to_string = "swl")]
    StoreWordLeft = 0x2a,

    #[strum(to_string = "sw")]
    StoreWord = 0x2b,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "sdl")]
    StoreDoubleLeft = 0x2c,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "sdr")]
    StoreDoubleRight = 0x2d,

    #[strum(to_string = "swr")]
    StoreWordRight = 0x2e,

    #[strum(to_string = "ll")]
    LoadLinked = 0x30,

    #[strum(to_string = "lwc1")]
    LoadWordCoprocessor1 = 0x31,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "lwc2")]
    LoadWordCoprocessor2 = 0x32,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "lwc3")]
    LoadWordCoprocessor3 = 0x33, // <- also Prefetch ("pref")?

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "lld")]
    LoadLinkedDouble = 0x34,

    #[strum(to_string = "ldc1")]
    LoadDoubleCoprocessor1 = 0x35,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "ldc2")]
    LoadDoubleCoprocessor2 = 0x36,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "ld")]
    LoadDouble = 0x37,

    #[strum(to_string = "sc")]
    StoreConditional = 0x38,

    #[strum(to_string = "swc1")]
    StoreWordCoprocessor1 = 0x39,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "swc2")]
    StoreWordCoprocessor2 = 0x3a,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "swc3")]
    StoreWordCoprocessor3 = 0x3b,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "scd")]
    StoreConditionalDouble = 0x3c,

    #[strum(to_string = "sdc1")]
    StoreDoubleCoprocessor1 = 0x3d,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "sdc2")]
    StoreDoubleCoprocessor2 = 0x3e,

    #[cfg(feature = "unimplemented")]
    #[strum(to_string = "sd")]
    StoreDouble = 0x3f,
}
