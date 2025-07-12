pub mod component;
pub mod destructured_instruction;

mod coprocessor_0;
mod coprocessor_1;
mod immediate;
mod jump;
mod operation;
mod special;
mod special_2;

pub use component::Component;
pub use destructured_instruction::DestructuredInstruction;
pub use operation::Operation;

use crate::fields;
use seaside_constants::InstructionFormat;
use seaside_type_aliases::{Address, Instruction};

pub fn destructure(instruction: Instruction, address: Address) -> Option<DestructuredInstruction> {
    use InstructionFormat::*;
    let opcode = fields::opcode(instruction)?;
    Some(
        match InstructionFormat::from(opcode) {
            Special => special::destructure(instruction),
            Immediate => immediate::destructure(opcode, instruction),
            Jump => jump::destructure(opcode, instruction),
            Coprocessor0 => coprocessor_0::destructure(instruction),
            Coprocessor1 => coprocessor_1::destructure(instruction),
            Special2 => special_2::destructure(instruction),
        }?
        .with_address(address),
    )
}
