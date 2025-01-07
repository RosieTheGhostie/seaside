use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum SpecialFn {
    ShiftLeftLogical = 0x00,             // sll
    MoveConditional = 0x01,              // movc
    ShiftRightLogical = 0x02,            // srl
    ShiftRightArithmetic = 0x03,         // sra
    ShiftLeftLogicalVariable = 0x04,     // sllv
    ShiftRightLogicalVariable = 0x06,    // srlv
    ShiftRightArithmeticVariable = 0x07, // srav
    JumpRegister = 0x08,                 // jr
    JumpAndLinkRegister = 0x09,          // jalr
    MoveZero = 0x0a,                     // movz
    MoveNotZero = 0x0b,                  // movn
    SystemCall = 0x0c,                   // syscall
    Break = 0x0d,                        // break
    MoveFromHigh = 0x10,                 // mfhi
    MoveToHigh = 0x11,                   // mthi
    MoveFromLow = 0x12,                  // mflo
    MoveToLow = 0x13,                    // mtlo
    Multiply = 0x18,                     // mult
    MultiplyUnsigned = 0x19,             // multu
    Divide = 0x1a,                       // div
    DivideUnsigned = 0x1b,               // divu
    Add = 0x20,                          // add
    AddUnsigned = 0x21,                  // addu
    Subtract = 0x22,                     // sub
    SubtractUnsigned = 0x23,             // subu
    And = 0x24,                          // and
    Or = 0x25,                           // or
    Xor = 0x26,                          // xor
    Nor = 0x27,                          // nor
    SetLessThan = 0x2a,                  // slt
    SetLessThanUnsigned = 0x2b,          // sltu
    TrapGreaterEqual = 0x30,             // tge
    TrapGreaterEqualUnsigned = 0x31,     // tgeu
    TrapLessThan = 0x32,                 // tlt
    TrapLessThanUnsigned = 0x33,         // tltu
    TrapEqual = 0x34,                    // teq
    TrapNotEqual = 0x36,                 // tne
}
