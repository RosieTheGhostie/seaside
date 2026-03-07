pub use field_layouts::*;

mod field_layouts;
mod lut;
mod macros;

use num_derive::FromPrimitive;
use strum::{Display, EnumDiscriminants};

/// A raw MIPS machine code instruction.
pub type Instruction = u32;

#[derive(Clone, Copy, Debug, Display, EnumDiscriminants, Eq, PartialEq)]
#[strum_discriminants(name(Opcode), derive(Display, FromPrimitive))]
#[repr(u8)]
pub enum UnpackedInstruction {
    #[strum_discriminants(strum(to_string = "<special fn>"))]
    Special(special::Fields) = 0x00,

    #[strum_discriminants(strum(to_string = "<register immediate fn>"))]
    RegisterImmediate(register_immediate::Fields) = 0x01,

    #[strum_discriminants(strum(to_string = "j"))]
    Jump(jump::Fields) = 0x02,

    #[strum_discriminants(strum(to_string = "jal"))]
    JumpAndLink(jump::Fields) = 0x03,

    #[strum_discriminants(strum(to_string = "beq"))]
    BranchEqual(immediate::Fields) = 0x04,

    #[strum_discriminants(strum(to_string = "bne"))]
    BranchNotEqual(immediate::Fields) = 0x05,

    #[strum_discriminants(strum(to_string = "blez"))]
    BranchLessEqualZero(immediate::Fields) = 0x06,

    #[strum_discriminants(strum(to_string = "bgtz"))]
    BranchGreaterThanZero(immediate::Fields) = 0x07,

    #[strum_discriminants(strum(to_string = "addi"))]
    AddImmediate(immediate::Fields) = 0x08,

    #[strum_discriminants(strum(to_string = "addiu"))]
    AddImmediateUnsigned(immediate::Fields) = 0x09,

    #[strum_discriminants(strum(to_string = "slti"))]
    SetLessThanImmediate(immediate::Fields) = 0x0a,

    #[strum_discriminants(strum(to_string = "sltiu"))]
    SetLessThanImmediateUnsigned(immediate::Fields) = 0x0b,

    #[strum_discriminants(strum(to_string = "andi"))]
    AndImmediate(immediate::Fields) = 0x0c,

    #[strum_discriminants(strum(to_string = "ori"))]
    OrImmediate(immediate::Fields) = 0x0d,

    #[strum_discriminants(strum(to_string = "xori"))]
    XorImmediate(immediate::Fields) = 0x0e,

    #[strum_discriminants(strum(to_string = "lui"))]
    LoadUpperImmediate(immediate::Fields) = 0x0f,

    #[strum_discriminants(strum(to_string = "<coprocessor 0 fn>"))]
    Coprocessor0(coprocessor_0::Fields) = 0x10,

    #[strum_discriminants(strum(to_string = "<coprocessor 1 fn>"))]
    Coprocessor1(coprocessor_1::Fields) = 0x11,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "<coprocessor 2 fn>"))]
    Coprocessor2(coprocessor_2::Fields) = 0x12,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "<coprocessor 1x fn>"))]
    Coprocessor1X(coprocessor_1x::Fields) = 0x13,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "beql"))]
    BranchEqualLikely(immediate::Fields) = 0x14,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "bnel"))]
    BranchNotEqualLikely(immediate::Fields) = 0x15,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "blezl"))]
    BranchLessEqualZeroLikely(immediate::Fields) = 0x16,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "bgtzl"))]
    BranchGreaterThanZeroLikely(immediate::Fields) = 0x17,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "daddi"))]
    DoubleAddImmediate(immediate::Fields) = 0x18,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "daddiu"))]
    DoubleAddImmediateUnsigned(immediate::Fields) = 0x19,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "ldl"))]
    LoadDoubleLeft(immediate::Fields) = 0x1a,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "ldr"))]
    LoadDoubleRight(immediate::Fields) = 0x1b,

    #[strum_discriminants(strum(to_string = "<special 2 fn>"))]
    Special2(special_2::Fields) = 0x1c,

    #[strum_discriminants(strum(to_string = "lb"))]
    LoadByte(immediate::Fields) = 0x20,

    #[strum_discriminants(strum(to_string = "lh"))]
    LoadHalf(immediate::Fields) = 0x21,

    #[strum_discriminants(strum(to_string = "lwl"))]
    LoadWordLeft(immediate::Fields) = 0x22,

    #[strum_discriminants(strum(to_string = "lw"))]
    LoadWord(immediate::Fields) = 0x23,

    #[strum_discriminants(strum(to_string = "lbu"))]
    LoadByteUnsigned(immediate::Fields) = 0x24,

    #[strum_discriminants(strum(to_string = "lhu"))]
    LoadHalfUnsigned(immediate::Fields) = 0x25,

    #[strum_discriminants(strum(to_string = "lwr"))]
    LoadWordRight(immediate::Fields) = 0x26,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "lwu"))]
    LoadWordUnsigned(immediate::Fields) = 0x27,

    #[strum_discriminants(strum(to_string = "sb"))]
    StoreByte(immediate::Fields) = 0x28,

    #[strum_discriminants(strum(to_string = "sh"))]
    StoreHalf(immediate::Fields) = 0x29,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "swl"))]
    StoreWordLeft(immediate::Fields) = 0x2a,

    #[strum_discriminants(strum(to_string = "sw"))]
    StoreWord(immediate::Fields) = 0x2b,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "sdl"))]
    StoreDoubleLeft(immediate::Fields) = 0x2c,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "sdr"))]
    StoreDoubleRight(immediate::Fields) = 0x2d,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "swr"))]
    StoreWordRight(immediate::Fields) = 0x2e,

    #[strum_discriminants(strum(to_string = "ll"))]
    LoadLinked(immediate::Fields) = 0x30,

    #[strum_discriminants(strum(to_string = "lwc1"))]
    LoadWordCoprocessor1(immediate::Fields) = 0x31,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "lwc2"))]
    LoadWordCoprocessor2(immediate::Fields) = 0x32,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "lwc3"))]
    LoadWordCoprocessor3(immediate::Fields) = 0x33, // <- also Prefetch ("pref")?

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "lld"))]
    LoadLinkedDouble(immediate::Fields) = 0x34,

    #[strum_discriminants(strum(to_string = "ldc1"))]
    LoadDoubleCoprocessor1(immediate::Fields) = 0x35,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "ldc2"))]
    LoadDoubleCoprocessor2(immediate::Fields) = 0x36,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "ld"))]
    LoadDouble(immediate::Fields) = 0x37,

    #[strum_discriminants(strum(to_string = "sc"))]
    StoreConditional(immediate::Fields) = 0x38,

    #[strum_discriminants(strum(to_string = "swc1"))]
    StoreWordCoprocessor1(immediate::Fields) = 0x39,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "swc2"))]
    StoreWordCoprocessor2(immediate::Fields) = 0x3a,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "swc3"))]
    StoreWordCoprocessor3(immediate::Fields) = 0x3b,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "scd"))]
    StoreConditionalDouble(immediate::Fields) = 0x3c,

    #[strum_discriminants(strum(to_string = "sdc1"))]
    StoreDoubleCoprocessor1(immediate::Fields) = 0x3d,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "sdc2"))]
    StoreDoubleCoprocessor2(immediate::Fields) = 0x3e,

    /// **!! UNIMPLEMENTED !!**
    #[strum_discriminants(strum(to_string = "sd"))]
    StoreDouble(immediate::Fields) = 0x3f,
}

impl UnpackedInstruction {
    pub const fn opcode(&self) -> Opcode {
        // https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.access-memory
        unsafe { *(self as *const Self as *const _) }
    }

    pub fn parse_from_operator(operator: &str) -> Option<Self> {
        lut::INSTRUCTION_TEMPLATES.get(operator).copied()
    }
}

pub trait Packable {
    fn pack(self) -> Instruction;
}

impl Packable for UnpackedInstruction {
    fn pack(self) -> Instruction {
        use UnpackedInstruction::*;

        let mut machine_code = (self.opcode() as Instruction) << 26;
        machine_code |= match self {
            Special(fields) => fields.pack(),
            Special2(fields) => fields.pack(),
            RegisterImmediate(fields) => fields.pack(),
            Jump(fields) | JumpAndLink(fields) => fields.pack(),
            BranchEqual(fields)
            | BranchNotEqual(fields)
            | BranchLessEqualZero(fields)
            | BranchGreaterThanZero(fields)
            | AddImmediate(fields)
            | AddImmediateUnsigned(fields)
            | SetLessThanImmediate(fields)
            | SetLessThanImmediateUnsigned(fields)
            | AndImmediate(fields)
            | OrImmediate(fields)
            | XorImmediate(fields)
            | LoadUpperImmediate(fields)
            | BranchEqualLikely(fields)
            | BranchNotEqualLikely(fields)
            | BranchLessEqualZeroLikely(fields)
            | BranchGreaterThanZeroLikely(fields)
            | DoubleAddImmediate(fields)
            | DoubleAddImmediateUnsigned(fields)
            | LoadDoubleLeft(fields)
            | LoadDoubleRight(fields)
            | LoadByte(fields)
            | LoadHalf(fields)
            | LoadWordLeft(fields)
            | LoadWord(fields)
            | LoadByteUnsigned(fields)
            | LoadHalfUnsigned(fields)
            | LoadWordRight(fields)
            | LoadWordUnsigned(fields)
            | StoreByte(fields)
            | StoreHalf(fields)
            | StoreWordLeft(fields)
            | StoreWord(fields)
            | StoreDoubleLeft(fields)
            | StoreDoubleRight(fields)
            | StoreWordRight(fields)
            | LoadLinked(fields)
            | LoadWordCoprocessor1(fields)
            | LoadWordCoprocessor2(fields)
            | LoadWordCoprocessor3(fields)
            | LoadLinkedDouble(fields)
            | LoadDoubleCoprocessor1(fields)
            | LoadDoubleCoprocessor2(fields)
            | LoadDouble(fields)
            | StoreConditional(fields)
            | StoreWordCoprocessor1(fields)
            | StoreWordCoprocessor2(fields)
            | StoreWordCoprocessor3(fields)
            | StoreConditionalDouble(fields)
            | StoreDoubleCoprocessor1(fields)
            | StoreDoubleCoprocessor2(fields)
            | StoreDouble(fields) => fields.pack(),
            Coprocessor0(fields) => fields.pack(),
            Coprocessor1(fields) => fields.pack(),
            Coprocessor2(fields) => fields.pack(),
            Coprocessor1X(fields) => fields.pack(),
        };

        machine_code
    }
}
