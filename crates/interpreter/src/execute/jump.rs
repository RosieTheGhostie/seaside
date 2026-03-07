use seaside_core::{prelude::*, traits::SignExtend};
use seaside_disassembler::fields;

use crate::{Exception, Interpreter, InterpreterState, register_file::IndexByRegister};

impl Interpreter {
    pub fn execute_jump_format(
        &mut self,
        opcode: Opcode,
        instruction: Instruction,
    ) -> Result<(), Exception> {
        let jump_index = fields::jump_index(instruction);
        let address = (self.state.pc & 0xf000_0000) | (jump_index << 2);
        if opcode == Opcode::JumpAndLink {
            self.state.link();
        }

        self.state.pc = address;
        Ok(())
    }
}

impl InterpreterState {
    pub fn branch(&mut self, offset: u16) {
        let n_instructions: Offset = offset.sign_extend();
        let offset = n_instructions << 2;
        self.pc = Address::wrapping_add_signed(self.pc, offset);
    }

    pub fn link(&mut self) {
        self.registers.write(CpuRegister::ReturnAddr, self.pc);
    }
}
