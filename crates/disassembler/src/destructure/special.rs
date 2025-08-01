use super::{Component, DestructuredInstruction, Operation};
use crate::fields;
use num_traits::FromPrimitive;
use seaside_constants::fn_codes::SpecialFn;
use seaside_type_aliases::Instruction;

pub fn destructure(instruction: Instruction) -> Option<DestructuredInstruction> {
    use SpecialFn::*;
    let rs = fields::rs(instruction);
    let rt = fields::rt(instruction);
    let rd = fields::rd(instruction);
    let r#fn = SpecialFn::from_u8(fields::r#fn(instruction))?;
    let mut components = [Component::default(); 5];
    match r#fn {
        ShiftLeftLogical | ShiftRightLogical | ShiftRightArithmetic => {
            components[0] = Component::CpuRegister(rd);
            components[1] = Component::CpuRegister(rt);
            components[2] = Component::Shamt(fields::shamt(instruction));
        }
        ShiftLeftLogicalVariable | ShiftRightLogicalVariable | ShiftRightArithmeticVariable => {
            components[0] = Component::CpuRegister(rd);
            components[1] = Component::CpuRegister(rt);
            components[2] = Component::CpuRegister(rs);
        }
        MoveZero | MoveNotZero | Add | AddUnsigned | Subtract | SubtractUnsigned | And | Or
        | Xor | Nor | SetLessThan | SetLessThanUnsigned => {
            components[0] = Component::CpuRegister(rd);
            components[1] = Component::CpuRegister(rs);
            components[2] = Component::CpuRegister(rt);
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
            components[0] = Component::CpuRegister(rs);
            components[1] = Component::CpuRegister(rt);
        }
        JumpRegister | MoveToHigh | MoveToLow => {
            components[0] = Component::CpuRegister(rs);
        }
        JumpAndLinkRegister => {
            components[0] = Component::CpuRegister(rd);
            components[1] = Component::CpuRegister(rs);
        }
        MoveFromHigh | MoveFromLow => components[0] = Component::CpuRegister(rd),
        Break => components[0] = Component::Code(fields::code(instruction)),
        MoveConditional => {
            components[0] = Component::Condition(fields::condition_from_cpu_register(rt));
            components[1] = Component::CpuRegister(rd);
            components[2] = Component::CpuRegister(rs);
            components[3] = Component::Cc(fields::cc_from_cpu_register(rt));
        }
        SystemCall => {}
    };
    Some(DestructuredInstruction::new(
        Operation::SpecialFn(r#fn),
        components,
    ))
}
