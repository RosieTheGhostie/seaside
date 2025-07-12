use seaside_constants::{
    Opcode,
    fn_codes::{
        Coprocessor0Fn, Coprocessor1Fn, Coprocessor1RegisterImmediateFn, RegisterImmediateFn,
        Special2Fn, SpecialFn,
    },
};

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Opcode(Opcode),
    SpecialFn(SpecialFn),
    RegisterImmediateFn(RegisterImmediateFn),
    Coprocessor0Fn(Coprocessor0Fn),
    Coprocessor1Fn(Coprocessor1Fn),
    Coprocessor1RegisterImmediateFn(Coprocessor1RegisterImmediateFn),
    Special2Fn(Special2Fn),
}

impl Default for Operation {
    fn default() -> Self {
        Self::Opcode(Opcode::Special)
    }
}
