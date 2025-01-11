use super::{component::Component, fields, operation::Operation, DestructuredInstruction};
use crate::{constants::opcodes::Opcode, type_aliases::instruction::Instruction};

pub fn destructure(opcode: Opcode, instruction: Instruction) -> Option<DestructuredInstruction> {
    Some(DestructuredInstruction {
        operation: Operation::Opcode(opcode),
        components: [
            Component::Index(fields::jump_index(instruction)),
            Component::Empty,
            Component::Empty,
            Component::Empty,
            Component::Empty,
        ],
    })
}
