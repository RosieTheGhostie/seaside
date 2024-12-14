pub enum Exception {
    InvalidLoad = 4,
    InvalidStore = 5,
    SyscallFailure = 8,
    Breakpoint = 9,
    ReservedInstruction = 10,
    IntegerOverflowOrUndeflow = 12,
    Trap = 13,
    DivideByZero = 15,
    FloatOverflow = 16,
    FloatUnderflow = 17,
}
