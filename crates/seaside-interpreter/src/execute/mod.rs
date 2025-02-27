mod coprocessor_0;
mod coprocessor_1;
mod immediate;
mod jump;
mod special;
mod special_2;
mod syscall;

use crate::{Exception, Interpreter};
use num_traits::FromPrimitive;
use seaside_constants::{InstructionFormat, Opcode};
use seaside_disassembler::fields;
use seaside_type_aliases::Instruction;

impl Interpreter {
    pub fn execute(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use InstructionFormat::*;
        let opcode = match Opcode::from_u8(fields::opcode(instruction)) {
            Some(opcode) => opcode,
            None => return Err(Exception::ReservedInstruction),
        };
        match InstructionFormat::from(opcode) {
            Special => self.execute_special(instruction),
            Immediate => self.execute_immediate_format(opcode, instruction),
            Jump => self.execute_jump_format(opcode, instruction),
            Coprocessor0 => self.execute_coprocessor_0(instruction),
            Coprocessor1 => self.execute_coprocessor_1(instruction),
            Special2 => self.execute_special_2(instruction),
        }
    }
}
