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
    #[error("sb")]
    StoreByte = 0x28,
    #[error("sh")]
    StoreHalf = 0x29,
    #[error("swl")]
    StoreWordLeft = 0x2a,
    #[error("sw")]
    StoreWord = 0x2b,
    #[error("sc")]
    StoreConditional = 0x2d,
    #[error("swr")]
    StoreWordRight = 0x2e,
    #[error("ll")]
    LoadLinked = 0x30,
    #[error("lwc1")]
    LoadWordCoprocessor1 = 0x31,
    #[error("ldc1")]
    LoadDoubleCoprocessor1 = 0x35,
    #[error("swc1")]
    StoreWordCoprocessor1 = 0x39,
    #[error("sdc1")]
    StoreDoubleCoprocessor1 = 0x3d,
}
