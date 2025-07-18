use num_derive::FromPrimitive;
use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum SpecialFn {
    #[error("sll")]
    ShiftLeftLogical = 0x00,
    #[error("mov")] // will be properly suffixed later
    MoveConditional = 0x01,
    #[error("srl")]
    ShiftRightLogical = 0x02,
    #[error("sra")]
    ShiftRightArithmetic = 0x03,
    #[error("sllv")]
    ShiftLeftLogicalVariable = 0x04,
    #[error("srlv")]
    ShiftRightLogicalVariable = 0x06,
    #[error("srav")]
    ShiftRightArithmeticVariable = 0x07,
    #[error("jr")]
    JumpRegister = 0x08,
    #[error("jalr")]
    JumpAndLinkRegister = 0x09,
    #[error("movz")]
    MoveZero = 0x0a,
    #[error("movn")]
    MoveNotZero = 0x0b,
    #[error("syscall")]
    SystemCall = 0x0c,
    #[error("break")]
    Break = 0x0d,
    // #[error("sync")]
    // Synchronize = 0x0f,
    #[error("mfhi")]
    MoveFromHigh = 0x10,
    #[error("mthi")]
    MoveToHigh = 0x11,
    #[error("mflo")]
    MoveFromLow = 0x12,
    #[error("mtlo")]
    MoveToLow = 0x13,
    // #[error("dsllv")]
    // DoubleShiftLeftLogicalValue = 0x14,
    // #[error("dsrlv")]
    // DoubleShiftRightLogicalValue = 0x16,
    // #[error("dsrav")]
    // DoubleShiftRightArithmeticValue = 0x17,
    #[error("mult")]
    Multiply = 0x18,
    #[error("multu")]
    MultiplyUnsigned = 0x19,
    #[error("div")]
    Divide = 0x1a,
    #[error("divu")]
    DivideUnsigned = 0x1b,
    // #[error("dmult")]
    // DoubleMultiply = 0x1c,
    // #[error("dmultu")]
    // DoubleMultiplyUnsigned = 0x1d,
    // #[error("ddiv")]
    // DoubleDivide = 0x1e,
    // #[error("ddivu")]
    // DoubleDivideUnsigned = 0x1f,
    #[error("add")]
    Add = 0x20,
    #[error("addu")]
    AddUnsigned = 0x21,
    #[error("sub")]
    Subtract = 0x22,
    #[error("subu")]
    SubtractUnsigned = 0x23,
    #[error("and")]
    And = 0x24,
    #[error("or")]
    Or = 0x25,
    #[error("xor")]
    Xor = 0x26,
    #[error("nor")]
    Nor = 0x27,
    #[error("slt")]
    SetLessThan = 0x2a,
    #[error("sltu")]
    SetLessThanUnsigned = 0x2b,
    // #[error("dadd")]
    // DoubleAdd = 0x2c,
    // #[error("daddu")]
    // DoubleAddUnsigned = 0x2d,
    // #[error("dsub")]
    // DoubleSubtract = 0x2e,
    // #[error("dsubu")]
    // DoubleSubtractUnsigned = 0x2f,
    #[error("tge")]
    TrapGreaterEqual = 0x30,
    #[error("tgeu")]
    TrapGreaterEqualUnsigned = 0x31,
    #[error("tlt")]
    TrapLessThan = 0x32,
    #[error("tltu")]
    TrapLessThanUnsigned = 0x33,
    #[error("teq")]
    TrapEqual = 0x34,
    #[error("tne")]
    TrapNotEqual = 0x36,
    // #[error("dsll")]
    // DoubleShiftLeftLogical = 0x38,
    // #[error("dsrl")]
    // DoubleShiftRightLogical = 0x3a,
    // #[error("dsra")]
    // DoubleShiftRightArithmetic = 0x3b,
    // #[error("dsll32")]
    // DoubleShiftLeftLogical32 = 0x3c,
    // #[error("dsrl32")]
    // DoubleShiftRightLogical32 = 0x3e,
    // #[error("dsra32")]
    // DoubleShiftRightAritmetic32 = 0x3f,
}
