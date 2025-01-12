use super::{component::Component, fields, operation::Operation, DestructuredInstruction};
use crate::{constants::fn_codes::SpecialFn, type_aliases::instruction::Instruction};
use num_traits::FromPrimitive;

pub fn destructure(instruction: Instruction) -> Option<DestructuredInstruction> {
    use SpecialFn::*;
    let rs = fields::rs(instruction);
    let rt = fields::rt(instruction);
    let rd = fields::rd(instruction);
    let r#fn = SpecialFn::from_u8(fields::r#fn(instruction))?;
    let mut components = [Component::default(); 5];
    match r#fn {
        ShiftLeftLogical | ShiftRightLogical | ShiftRightArithmetic => {
            components[0] = Component::Gpr(rd);
            components[1] = Component::Gpr(rt);
            components[2] = Component::Shamt(fields::shamt(instruction));
        }
        ShiftLeftLogicalVariable | ShiftRightLogicalVariable | ShiftRightArithmeticVariable => {
            components[0] = Component::Gpr(rd);
            components[1] = Component::Gpr(rt);
            components[2] = Component::Gpr(rs);
        }
        MoveZero | MoveNotZero | Add | AddUnsigned | Subtract | SubtractUnsigned | And | Or
        | Xor | Nor | SetLessThan | SetLessThanUnsigned => {
            components[0] = Component::Gpr(rd);
            components[1] = Component::Gpr(rs);
            components[2] = Component::Gpr(rt);
        }
        Multiply
        | MultiplyUnsigned
        | Divide
        | DivideUnsigned
        | TrapGreaterEqual
        | TrapGreaterEqualUnsigned
        | TrapLessThan
        | TrapLessThanUnsigned
        | TrapEqual
        | TrapNotEqual => {
            components[0] = Component::Gpr(rs);
            components[1] = Component::Gpr(rt);
        }
        JumpRegister | JumpAndLinkRegister | MoveToHigh | MoveToLow => {
            components[0] = Component::Gpr(rs);
        }
        MoveFromHigh | MoveFromLow => components[0] = Component::Gpr(rd),
        Break => components[0] = Component::Code(fields::code(instruction)),
        MoveConditional => {
            components[0] = Component::Condition(fields::condition_from_index(rt));
            components[1] = Component::Gpr(rd);
            components[2] = Component::Gpr(rs);
            components[3] = Component::Cc(fields::cc_from_index(rt));
        }
        SystemCall => {}
    };
    Some(DestructuredInstruction {
        operation: Operation::SpecialFn(r#fn),
        components,
    })
}