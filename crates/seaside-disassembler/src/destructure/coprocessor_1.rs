use super::{component::Component, fields, operation::Operation, DestructuredInstruction};
use num_traits::FromPrimitive;
use seaside_constants::{fn_codes::Coprocessor1Fn, number_fmt::NumberFormat};
use seaside_type_aliases::Instruction;

pub fn destructure(instruction: Instruction) -> Option<DestructuredInstruction> {
    use Coprocessor1Fn::*;
    let mut components = [Component::default(); 5];
    let ft = fields::ft(instruction);
    let fmt = fields::fmt(instruction);
    if fmt == 8 {
        components[0] = Component::Condition(fields::condition_from_index(ft));
        components[1] = Component::Cc(fields::cc_from_index(ft));
        components[2] = Component::Offset(fields::imm(instruction));
        return Some(DestructuredInstruction::new(
            Operation::BranchCoprocessor1,
            components,
        ));
    }
    let fmt = NumberFormat::from_u8(fmt)?;
    let fs = fields::fs(instruction);
    let fd = fields::fd(instruction);
    let r#fn = Coprocessor1Fn::from_u8(fields::r#fn(instruction))?;
    match r#fn {
        Add | Subtract | Multiply | Divide => {
            components[0] = Component::Fmt(fmt);
            components[1] = Component::Fpr(fd);
            components[2] = Component::Fpr(fs);
            components[3] = Component::Fpr(ft);
        }
        SquareRoot | AbsoluteValue | Move | Negate | RoundWord | TruncateWord | CeilingWord
        | FloorWord | ConvertToSingle | ConvertToDouble | ConvertToWord => {
            components[0] = Component::Fmt(fmt);
            components[1] = Component::Fpr(fd);
            components[2] = Component::Fpr(fs);
        }
        MoveConditional => {
            components[0] = Component::Condition(fields::condition_from_index(ft));
            components[1] = Component::Fmt(fmt);
            components[2] = Component::Fpr(fd);
            components[3] = Component::Fpr(fs);
            components[4] = Component::Cc(fields::cc_from_index(ft));
        }
        MoveZero | MoveNotZero => {
            components[0] = Component::Fmt(fmt);
            components[1] = Component::Fpr(fs);
            components[2] = Component::Fpr(fd);
            components[3] = Component::Gpr(ft);
        }
        CompareEqual | CompareLessThan | CompareLessEqual => {
            components[0] = Component::Fmt(fmt);
            components[1] = Component::Cc(fields::cc_from_index(fd));
            components[2] = Component::Fpr(fd);
            components[3] = Component::Fpr(ft);
        }
    }
    Some(DestructuredInstruction::new(
        Operation::Coprocessor1Fn(r#fn),
        components,
    ))
}
