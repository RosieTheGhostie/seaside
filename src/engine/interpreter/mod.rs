#![allow(dead_code)]
pub mod exception;
pub mod memory;
pub mod register_file;

mod execute;
mod instruction;
mod syscalls;

pub use exception::Exception;
pub use memory::Memory;
pub use register_file::RegisterFile;
pub use syscalls::Syscalls;

use super::Error;
use crate::config::Config;
use crate::type_aliases::address::Address;
use std::path::PathBuf;

pub struct Interpreter {
    memory: Memory,
    registers: RegisterFile,
    pc: Address,
    syscalls: Syscalls,
    pub exit_code: Option<u8>,
}

impl Interpreter {
    pub fn init(
        config: &Config,
        text: PathBuf,
        r#extern: Option<PathBuf>,
        data: Option<PathBuf>,
        ktext: Option<PathBuf>,
        kdata: Option<PathBuf>,
    ) -> Result<Self, Error> {
        let memory = Memory::init(config, text, r#extern, data, ktext, kdata)?;
        let pc = memory.initial_pc();
        let syscalls = Syscalls::from(&config.features.syscalls);
        let registers = RegisterFile::init(&config.register_defaults);
        Ok(Self {
            memory,
            registers,
            pc,
            syscalls,
            exit_code: None,
        })
    }

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
