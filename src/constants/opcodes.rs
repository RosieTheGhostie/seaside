use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Opcode {
    Special = 0x00,                      // <special fn>
    RegisterImmediate = 0x01,            // <register immediate fn>
    Jump = 0x02,                         // j
    JumpAndLink = 0x03,                  // jal
    BranchEqual = 0x04,                  // beq
    BranchNotEqual = 0x05,               // bne
    BranchLessEqualZero = 0x06,          // blez
    BranchGreaterThanZero = 0x07,        // bgtz
    AddImmediate = 0x08,                 // addi
    AddImmediateUnsigned = 0x09,         // addiu
    SetLessThanImmediate = 0x0a,         // slti
    SetLessThanImmediateUnsigned = 0x0b, // sltiu
    AndImmediate = 0x0c,                 // andi
    OrImmediate = 0x0d,                  // ori
    XorImmediate = 0x0e,                 // xori
    LoadUpperImmediate = 0x0f,           // lui
    Coprocessor0 = 0x10,                 // <coprocessor 0 fn>
    Coprocessor1 = 0x11,                 // <coprocessor 1 fn>
    Special2 = 0x1C,                     // <special 2 fn>
    LoadByte = 0x20,                     // lb
    LoadHalf = 0x21,                     // lh
    LoadWordLeft = 0x22,                 // lwl
    LoadWord = 0x23,                     // lw
    LoadByteUnsigned = 0x24,             // lbu
    LoadHalfUnsigned = 0x25,             // lhu
    LoadWordRight = 0x26,                // lwr
    StoreByte = 0x28,                    // sb
    StoreHalf = 0x29,                    // sh
    StoreWordLeft = 0x2a,                // swl
    StoreWord = 0x2b,                    // sw
    StoreConditional = 0x2d,             // sc
    StoreWordRight = 0x2e,               // swr
    LoadLinked = 0x30,                   // ll
    LoadWordCoprocessor1 = 0x31,         // lwc1
    LoadDoubleCoprocessor1 = 0x35,       // ldc1
    StoreWordCoprocessor1 = 0x39,        // swc1
    StoreDoubleCoprocessor1 = 0x3d,      // sdc1
}
