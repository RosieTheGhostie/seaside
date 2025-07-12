use num_derive::FromPrimitive;
use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Opcode {
    #[error("<special fn>")]
    Special = 0x00,
    #[error("<register immediate fn>")]
    RegisterImmediate = 0x01,
    #[error("j")]
    Jump = 0x02,
    #[error("jal")]
    JumpAndLink = 0x03,
    #[error("beq")]
    BranchEqual = 0x04,
    #[error("bne")]
    BranchNotEqual = 0x05,
    #[error("blez")]
    BranchLessEqualZero = 0x06,
    #[error("bgtz")]
    BranchGreaterThanZero = 0x07,
    #[error("addi")]
    AddImmediate = 0x08,
    #[error("addiu")]
    AddImmediateUnsigned = 0x09,
    #[error("slti")]
    SetLessThanImmediate = 0x0a,
    #[error("sltiu")]
    SetLessThanImmediateUnsigned = 0x0b,
    #[error("andi")]
    AndImmediate = 0x0c,
    #[error("ori")]
    OrImmediate = 0x0d,
    #[error("xori")]
    XorImmediate = 0x0e,
    #[error("lui")]
    LoadUpperImmediate = 0x0f,
    #[error("<coprocessor 0 fn>")]
    Coprocessor0 = 0x10,
    #[error("<coprocessor 1 fn>")]
    Coprocessor1 = 0x11,
    // #[error("<coprocessor 2 fn>")]
    // Coprocessor2 = 0x12,
    // #[error("<coprocessor 1x fn>")]
    // Coprocessor1X = 0x13,
    // #[error("beql")]
    // BranchEqualLikely = 0x14,
    // #[error("bnel")]
    // BranchNotEqualLikely = 0x15,
    // #[error("blezl")]
    // BranchLessEqualZeroLikely = 0x16,
    // #[error("bgtzl")]
    // BranchGreaterThanZeroLikely = 0x17,
    // #[error("daddi")]
    // DoubleAddImmediate = 0x18,
    // #[error("daddiu")]
    // DoubleAddImmediateUnsigned = 0x19,
    // #[error("ldl")]
    // LoadDoubleLeft = 0x1a,
    // #[error("ldr")]
    // LoadDoubleRight = 0x1b,
    #[error("<special 2 fn>")]
    Special2 = 0x1c,
    #[error("lb")]
    LoadByte = 0x20,
    #[error("lh")]
    LoadHalf = 0x21,
    #[error("lwl")]
    LoadWordLeft = 0x22,
    #[error("lw")]
    LoadWord = 0x23,
    #[error("lbu")]
    LoadByteUnsigned = 0x24,
    #[error("lhu")]
    LoadHalfUnsigned = 0x25,
    #[error("lwr")]
    LoadWordRight = 0x26,
    // #[error("lwu")]
    // LoadWordUnsigned = 0x27,
    #[error("sb")]
    StoreByte = 0x28,
    #[error("sh")]
    StoreHalf = 0x29,
    #[error("swl")]
    StoreWordLeft = 0x2a,
    #[error("sw")]
    StoreWord = 0x2b,
    // #[error("sdl")]
    // StoreDoubleLeft = 0x2c,
    // #[error("sdr")]
    // StoreDoubleRight = 0x2d,
    #[error("swr")]
    StoreWordRight = 0x2e,
    #[error("ll")]
    LoadLinked = 0x30,
    #[error("lwc1")]
    LoadWordCoprocessor1 = 0x31,
    // #[error("lwc2")]
    // LoadWordCoprocessor2 = 0x32,
    // #[error("lwc3")]
    // LoadWordCoprocessor3 = 0x33, <- also Prefetch ("pref")?
    // #[error("lld")]
    // LoadLinkedDouble = 0x34,
    #[error("ldc1")]
    LoadDoubleCoprocessor1 = 0x35,
    // #[error("ldc2")]
    // LoadDoubleCoprocessor2 = 0x36,
    // #[error("ld")]
    // LoadDouble = 0x37,
    #[error("sc")]
    StoreConditional = 0x38,
    #[error("swc1")]
    StoreWordCoprocessor1 = 0x39,
    // #[error("swc2")]
    // StoreWordCoprocessor2 = 0x3a,
    // #[error("swc3")]
    // StoreWordCoprocessor3 = 0x3b,
    // #[error("scd")]
    // StoreConditionalDouble = 0x3c,
    #[error("sdc1")]
    StoreDoubleCoprocessor1 = 0x3d,
    // #[error("sdc2")]
    // StoreDoubleCoprocessor2 = 0x3e,
    // #[error("sd")]
    // StoreDouble = 0x3f,
}
