use super::{Component, DestructuredInstruction, Operation};
use crate::fields;
use num_traits::FromPrimitive;
use seaside_constants::{
    NumberFormat,
    fn_codes::{Coprocessor1Fn, Coprocessor1RegisterImmediateFn},
};
use seaside_type_aliases::Instruction;

pub fn destructure(instruction: Instruction) -> Option<DestructuredInstruction> {
    use Coprocessor1Fn::*;
    let mut components = [Component::default(); 5];
    let ft = fields::ft(instruction);
    let fmt = fields::fmt(instruction);
    match Coprocessor1RegisterImmediateFn::from_u8(fmt) {
        Some(
            r#fn @ (Coprocessor1RegisterImmediateFn::MoveFromCoprocessor1
            | Coprocessor1RegisterImmediateFn::MoveToCoprocessor1),
        ) => {
            components[0] = Component::CpuRegister(ft.to_cpu());
            components[1] = Component::FpuRegister(fields::fs(instruction));
            return Some(DestructuredInstruction::new(
                Operation::Coprocessor1RegisterImmediateFn(r#fn),
                components,
            ));
        }
        Some(r#fn @ Coprocessor1RegisterImmediateFn::BranchCoprocessor1Flag) => {
            components[0] = Component::Condition(fields::condition_from_fpu_register(ft));
            components[1] = Component::Cc(fields::cc_from_fpu_register(ft));
            components[2] = Component::Offset(fields::imm(instruction));
            return Some(DestructuredInstruction::new(
                Operation::Coprocessor1RegisterImmediateFn(r#fn),
                components,
            ));
        }
        None => {}
    }
    let fmt = NumberFormat::from_u8(fmt)?;
    let fs = fields::fs(instruction);
    let fd = fields::fd(instruction);
    let r#fn = Coprocessor1Fn::from_u8(fields::r#fn(instruction))?;
    match r#fn {
        Add | Subtract | Multiply | Divide => {
            components[0] = Component::Fmt(fmt);
            components[1] = Component::FpuRegister(fd);
            components[2] = Component::FpuRegister(fs);
            components[3] = Component::FpuRegister(ft);
        }
        SquareRoot | AbsoluteValue | Move | Negate | RoundWord | TruncateWord | CeilingWord
        | FloorWord | ConvertToSingle | ConvertToDouble | ConvertToWord => {
            components[0] = Component::Fmt(fmt);
            components[1] = Component::FpuRegister(fd);
            components[2] = Component::FpuRegister(fs);
        }
        MoveConditional => {
            components[0] = Component::Condition(fields::condition_from_fpu_register(ft));
            components[1] = Component::Fmt(fmt);
            components[2] = Component::FpuRegister(fd);
            components[3] = Component::FpuRegister(fs);
            components[4] = Component::Cc(fields::cc_from_fpu_register(ft));
        }
        MoveZero | MoveNotZero => {
            components[0] = Component::Fmt(fmt);
            components[1] = Component::FpuRegister(fd);
            components[2] = Component::FpuRegister(fs);
            components[3] = Component::CpuRegister(ft.to_cpu());
        }
        CompareEqual | CompareLessThan | CompareLessEqual => {
            components[0] = Component::Fmt(fmt);
            components[1] = Component::Cc(fields::cc_from_fpu_register(fd));
            components[2] = Component::FpuRegister(fs);
            components[3] = Component::FpuRegister(ft);
        }
    }
    Some(DestructuredInstruction::new(
        Operation::Coprocessor1Fn(r#fn),
        components,
    ))
}
