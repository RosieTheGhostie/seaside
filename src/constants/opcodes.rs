use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Opcode {
    Special = 0x00,                      // SPECIAL
    RegisterImmediate = 0x01,            // REGIMM
    Jump = 0x02,                         // J
    JumpAndLink = 0x03,                  // JAL
    BranchEqual = 0x04,                  // BEQ
    BranchNotEqual = 0x05,               // BNE
    BranchLessEqualZero = 0x06,          // BLEZ
    BranchGreaterThanZero = 0x07,        // BGTZ
    AddImmediate = 0x08,                 // ADDI
    AddImmediateUnsigned = 0x09,         // ADDIU
    SetLessThanImmediate = 0x0A,         // SLTI
    SetLessThanImmediateUnsigned = 0x0B, // SLTIU
    AndImmediate = 0x0C,                 // ANDI
    OrImmediate = 0x0D,                  // ORI
    XorImmediate = 0x0E,                 // XORI
    LoadUpperImmediate = 0x0F,           // LUI
    Coprocessor0 = 0x10,                 // CPC0
    Coprocessor1 = 0x11,                 // CPC1
    Special2 = 0x1C,                     // SPECIAL2
    LoadByte = 0x20,                     // LB
    LoadHalf = 0x21,                     // LH
    LoadWordLeft = 0x22,                 // LWL
    LoadWord = 0x23,                     // LW
    LoadByteUnsigned = 0x24,             // LBU
    LoadHalfUnsigned = 0x25,             // LHU
    LoadWordRight = 0x26,                // LWR
    StoreByte = 0x28,                    // SB
    StoreHalf = 0x29,                    // SH
    StoreWordLeft = 0x2A,                // SWL
    StoreWord = 0x2B,                    // SW
    StoreConditional = 0x2D,             // SC
    StoreWordRight = 0x2E,               // SWR
    LoadLinked = 0x30,                   // LL
    LoadWordCoprocessor1 = 0x31,         // LWC1
    StoreWordCoprocessor1 = 0x39,        // SWC1
}
