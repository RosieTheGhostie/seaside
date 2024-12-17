use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum SpecialFn {
    ShiftLeftLogical = 0x00,             // SLL
    MoveConditional = 0x01,              // MOVC
    ShiftRightLogical = 0x02,            // SRL
    ShiftRightArithmetic = 0x03,         // SRA
    ShiftLeftLogicalVariable = 0x04,     // SLLV
    ShiftRightLogicalVariable = 0x06,    // SRLV
    ShiftRightArithmeticVariable = 0x07, // SRAV
    JumpRegister = 0x08,                 // JR
    JumpAndLinkRegister = 0x09,          // JALR
    MoveZero = 0x0A,                     // MOVZ
    MoveNotZero = 0x0B,                  // MOVN
    SystemCall = 0x0C,                   // SYSCALL
    Break = 0x0D,                        // BREAK
    MoveFromHigh = 0x10,                 // MFHI
    MoveToHigh = 0x11,                   // MTHI
    MoveFromLow = 0x12,                  // MFLO
    MoveToLow = 0x13,                    // MTLO
    Multiply = 0x18,                     // MULT
    MultiplyUnsigned = 0x19,             // MULTU
    Divide = 0x1A,                       // DIV
    DivideUnsigned = 0x1B,               // DIVU
    Add = 0x20,                          // ADD
    AddUnsigned = 0x21,                  // ADDU
    Subtract = 0x22,                     // SUB
    SubtractUnsigned = 0x23,             // SUBU
    And = 0x24,                          // AND
    Or = 0x25,                           // OR
    Xor = 0x26,                          // XOR
    Nor = 0x27,                          // NOR
    SetLessThan = 0x2A,                  // SLT
    SetLessThanUnsigned = 0x2B,          // SLTU
    TrapGreaterEqual = 0x30,             // TGE
    TrapGreaterEqualUnsigned = 0x31,     // TGEU
    TrapLessThan = 0x32,                 // TLT
    TrapLessThanUnsigned = 0x33,         // TLTU
    TrapEqual = 0x34,                    // TEQ
    TrapNotEqual = 0x36,                 // TNE
}
