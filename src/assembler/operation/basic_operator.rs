use crate::constants::{
    fn_codes::{Coprocessor0Fn, Coprocessor1Fn, RegisterImmediateFn, Special2Fn, SpecialFn},
    number_fmt::NumberFormat,
    opcodes::Opcode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasicOperator {
    Special(SpecialFn, Option<bool>),
    RegisterImmediate(RegisterImmediateFn),
    Jump,
    JumpAndLink,
    BranchEqual,
    BranchNotEqual,
    BranchLessEqualZero,
    BranchGreaterThanZero,
    AddImmediate,
    AddImmediateUnsigned,
    SetLessThanImmediate,
    SetLessThanImmediateUnsigned,
    AndImmediate,
    OrImmediate,
    XorImmediate,
    LoadUpperImmediate,
    Coprocessor0(Coprocessor0Fn),
    Coprocessor1(Coprocessor1Fn, NumberFormat, Option<bool>),
    Special2(Special2Fn),
    LoadByte,
    LoadHalf,
    LoadWordLeft,
    LoadWord,
    LoadByteUnsigned,
    LoadHalfUnsigned,
    LoadWordRight,
    StoreByte,
    StoreHalf,
    StoreWordLeft,
    StoreWord,
    StoreConditional,
    StoreWordRight,
    LoadLinked,
    LoadWordCoprocessor1,
    LoadDoubleCoprocessor1,
    StoreWordCoprocessor1,
    StoreDoubleCoprocessor1,
}

impl BasicOperator {
    pub fn op_or_fn_code(&self) -> u8 {
        use BasicOperator::*;
        match self {
            Special(r#fn, _) => *r#fn as u8,
            RegisterImmediate(r#fn) => *r#fn as u8,
            Jump => Opcode::Jump as u8,
            JumpAndLink => Opcode::JumpAndLink as u8,
            BranchEqual => Opcode::BranchEqual as u8,
            BranchNotEqual => Opcode::BranchNotEqual as u8,
            BranchLessEqualZero => Opcode::BranchLessEqualZero as u8,
            BranchGreaterThanZero => Opcode::BranchGreaterThanZero as u8,
            AddImmediate => Opcode::AddImmediate as u8,
            AddImmediateUnsigned => Opcode::AddImmediateUnsigned as u8,
            SetLessThanImmediate => Opcode::SetLessThanImmediate as u8,
            SetLessThanImmediateUnsigned => Opcode::SetLessThanImmediateUnsigned as u8,
            AndImmediate => Opcode::AndImmediate as u8,
            OrImmediate => Opcode::OrImmediate as u8,
            XorImmediate => Opcode::XorImmediate as u8,
            LoadUpperImmediate => Opcode::LoadUpperImmediate as u8,
            Coprocessor0(r#fn) => *r#fn as u8,
            Coprocessor1(r#fn, _, _) => *r#fn as u8,
            Special2(r#fn) => *r#fn as u8,
            LoadByte => Opcode::LoadByte as u8,
            LoadHalf => Opcode::LoadHalf as u8,
            LoadWordLeft => Opcode::LoadWordLeft as u8,
            LoadWord => Opcode::LoadWord as u8,
            LoadByteUnsigned => Opcode::LoadByteUnsigned as u8,
            LoadHalfUnsigned => Opcode::LoadHalfUnsigned as u8,
            LoadWordRight => Opcode::LoadWordRight as u8,
            StoreByte => Opcode::StoreByte as u8,
            StoreHalf => Opcode::StoreHalf as u8,
            StoreWordLeft => Opcode::StoreWordLeft as u8,
            StoreWord => Opcode::StoreWord as u8,
            StoreConditional => Opcode::StoreConditional as u8,
            StoreWordRight => Opcode::StoreWordRight as u8,
            LoadLinked => Opcode::LoadLinked as u8,
            LoadWordCoprocessor1 => Opcode::LoadWordCoprocessor1 as u8,
            LoadDoubleCoprocessor1 => Opcode::LoadDoubleCoprocessor1 as u8,
            StoreWordCoprocessor1 => Opcode::StoreWordCoprocessor1 as u8,
            StoreDoubleCoprocessor1 => Opcode::StoreDoubleCoprocessor1 as u8,
        }
    }
}
