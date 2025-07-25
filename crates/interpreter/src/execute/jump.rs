use crate::{Exception, Interpreter, InterpreterState, register_file::IndexByRegister};
use seaside_constants::{Opcode, register::CpuRegister};
use seaside_disassembler::fields;
use seaside_int_utils::SignExtend;
use seaside_type_aliases::Instruction;

impl Interpreter {
    pub fn execute_jump_format(
        &mut self,
        opcode: Opcode,
        instruction: Instruction,
    ) -> Result<(), Exception> {
        let jump_index = fields::jump_index(instruction);
        let address = (self.state.pc & 0xf0000000) | (jump_index << 2);
        if opcode == Opcode::JumpAndLink {
            self.state.link();
        }
        self.state.pc = address;
        Ok(())
    }
}

impl InterpreterState {
    pub fn branch(&mut self, offset: u16) {
        let offset = <u16 as SignExtend<i32>>::sign_extend(&offset) << 2;
        self.pc = u32::wrapping_add_signed(self.pc, offset);
    }

    pub fn link(&mut self) {
        self.registers.write(CpuRegister::ReturnAddr, self.pc);
    }
}
