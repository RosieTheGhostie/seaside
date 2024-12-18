#![allow(dead_code)]
pub mod exception;
pub mod init;
pub mod memory;
pub mod register_file;

mod execute;
mod instruction;
mod syscalls;

pub use exception::Exception;
pub use memory::Memory;
pub use register_file::RegisterFile;
pub use syscalls::Syscalls;

use crate::type_aliases::address::Address;

pub struct Interpreter {
    memory: Memory,
    registers: RegisterFile,
    pc: Address,
    syscalls: Syscalls,
    pub exit_code: Option<u8>,
}

impl Interpreter {
    pub fn run(&mut self) -> Result<(), Exception> {
        while !self.memory.pc_past_end(self.pc) && self.exit_code.is_none() {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Exception> {
        let instruction = self.memory.get_instruction(self.pc)?;
        self.pc += 4;
        self.execute(instruction)
    }
}
