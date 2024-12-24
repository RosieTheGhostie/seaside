use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor1Fn {
    Add = 0x00,               // ADD
    Subtract = 0x01,          // SUB
    Multiply = 0x02,          // MUL
    Divide = 0x03,            // DIV
    SquareRoot = 0x04,        // SQRT
    AbsoluteValue = 0x05,     // ABS
    Move = 0x06,              // MOV
    Negate = 0x07,            // NEG
    BranchConditional = 0x08, // BC
    RoundWord = 0x0C,         // ROUND_W
    TruncateWord = 0x0D,      // TRUNC_W
    CeilingWord = 0x0E,       // CEIL_W
    FloorWord = 0x0F,         // FLOOR_W
    MoveConditional = 0x11,   // MOVC
    MoveZero = 0x12,          // MOVZ
    MoveNotZero = 0x13,       // MOVN
    ConvertToSingle = 0x20,   // CVT_S
    ConvertToDouble = 0x21,   // CVT_D
    ConvertToWord = 0x24,     // CVT_W
    CompareEqual = 0x32,      // C_EQ
    CompareLessThan = 0x3C,   // C_LT
    CompareLessEqual = 0x3E,  // C_LE
}
