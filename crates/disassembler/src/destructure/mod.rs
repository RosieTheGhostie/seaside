pub mod component;
pub mod destructured_instruction;

pub use component::Component;
pub use destructured_instruction::DestructuredInstruction;
pub use operation::Operation;

mod coprocessor_0;
mod coprocessor_1;
mod immediate;
mod jump;
mod operation;
mod special;
mod special_2;

use seaside_core::{consts::formats::InstructionFormat, prelude::*};

use crate::fields;

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
