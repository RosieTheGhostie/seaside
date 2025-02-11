use seaside_constants::{
    fn_codes::{Coprocessor0Fn, Coprocessor1Fn, RegisterImmediateFn, Special2Fn, SpecialFn},
    opcodes::Opcode,
};

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Opcode(Opcode),
    SpecialFn(SpecialFn),
    RegisterImmediateFn(RegisterImmediateFn),
    Coprocessor0Fn(Coprocessor0Fn),
    Coprocessor1Fn(Coprocessor1Fn),
    BranchCoprocessor1,
    Special2Fn(Special2Fn),
}

impl Default for Operation {
    fn default() -> Self {
        Self::Opcode(Opcode::Special)
    }
}
