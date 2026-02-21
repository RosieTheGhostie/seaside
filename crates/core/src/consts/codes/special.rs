use num_derive::FromPrimitive;
use strum::Display;

#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum SpecialFn {
    #[strum(to_string = "sll")]
    ShiftLeftLogical = 0x00,

    #[strum(to_string = "mov")] // will be properly suffixed later
    MoveConditional = 0x01,

    #[strum(to_string = "srl")]
    ShiftRightLogical = 0x02,

    #[strum(to_string = "sra")]
    ShiftRightArithmetic = 0x03,

    #[strum(to_string = "sllv")]
    ShiftLeftLogicalVariable = 0x04,

    #[strum(to_string = "srlv")]
    ShiftRightLogicalVariable = 0x06,

    #[strum(to_string = "srav")]
    ShiftRightArithmeticVariable = 0x07,

    #[strum(to_string = "jr")]
    JumpRegister = 0x08,

    #[strum(to_string = "jalr")]
    JumpAndLinkRegister = 0x09,

    #[strum(to_string = "movz")]
    MoveZero = 0x0a,

    #[strum(to_string = "movn")]
    MoveNotZero = 0x0b,

    #[strum(to_string = "syscall")]
    SystemCall = 0x0c,

    #[strum(to_string = "break")]
    Break = 0x0d,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "sync")]
    Synchronize = 0x0f,

    #[strum(to_string = "mfhi")]
    MoveFromHigh = 0x10,

    #[strum(to_string = "mthi")]
    MoveToHigh = 0x11,

    #[strum(to_string = "mflo")]
    MoveFromLow = 0x12,

    #[strum(to_string = "mtlo")]
    MoveToLow = 0x13,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsllv")]
    DoubleShiftLeftLogicalValue = 0x14,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsrlv")]
    DoubleShiftRightLogicalValue = 0x16,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsrav")]
    DoubleShiftRightArithmeticValue = 0x17,

    #[strum(to_string = "mult")]
    Multiply = 0x18,

    #[strum(to_string = "multu")]
    MultiplyUnsigned = 0x19,

    #[strum(to_string = "div")]
    Divide = 0x1a,

    #[strum(to_string = "divu")]
    DivideUnsigned = 0x1b,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dmult")]
    DoubleMultiply = 0x1c,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dmultu")]
    DoubleMultiplyUnsigned = 0x1d,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "ddiv")]
    DoubleDivide = 0x1e,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "ddivu")]
    DoubleDivideUnsigned = 0x1f,

    #[strum(to_string = "add")]
    Add = 0x20,

    #[strum(to_string = "addu")]
    AddUnsigned = 0x21,

    #[strum(to_string = "sub")]
    Subtract = 0x22,

    #[strum(to_string = "subu")]
    SubtractUnsigned = 0x23,

    #[strum(to_string = "and")]
    And = 0x24,

    #[strum(to_string = "or")]
    Or = 0x25,

    #[strum(to_string = "xor")]
    Xor = 0x26,

    #[strum(to_string = "nor")]
    Nor = 0x27,

    #[strum(to_string = "slt")]
    SetLessThan = 0x2a,

    #[strum(to_string = "sltu")]
    SetLessThanUnsigned = 0x2b,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dadd")]
    DoubleAdd = 0x2c,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "daddu")]
    DoubleAddUnsigned = 0x2d,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsub")]
    DoubleSubtract = 0x2e,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsubu")]
    DoubleSubtractUnsigned = 0x2f,

    #[strum(to_string = "tge")]
    TrapGreaterEqual = 0x30,

    #[strum(to_string = "tgeu")]
    TrapGreaterEqualUnsigned = 0x31,

    #[strum(to_string = "tlt")]
    TrapLessThan = 0x32,

    #[strum(to_string = "tltu")]
    TrapLessThanUnsigned = 0x33,

    #[strum(to_string = "teq")]
    TrapEqual = 0x34,

    #[strum(to_string = "tne")]
    TrapNotEqual = 0x36,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsll")]
    DoubleShiftLeftLogical = 0x38,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsrl")]
    DoubleShiftRightLogical = 0x3a,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsra")]
    DoubleShiftRightArithmetic = 0x3b,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsll32")]
    DoubleShiftLeftLogical32 = 0x3c,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsrl32")]
    DoubleShiftRightLogical32 = 0x3e,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = "dsra32")]
    DoubleShiftRightAritmetic32 = 0x3f,
}
