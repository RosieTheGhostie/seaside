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

use crate::{config::Config, engine::Error, type_aliases::address::Address};
use minimal_logging::macros::debugln;
use std::path::PathBuf;

pub struct Interpreter {
    memory: Memory,
    registers: RegisterFile,
    pc: Address,
    syscalls: Syscalls,
    pub show_crash_handler: bool,
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
            show_crash_handler: config.features.show_crash_handler,
            exit_code: None,
        })
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        while !self.memory.pc_past_end(self.pc) && self.exit_code.is_none() {
            if let Err(exception) = self.step() {
                match self.memory.get_exception_handler() {
                    Some(exception_handler) => self.trigger_exception(exception, exception_handler),
                    None => return Err(exception),
                }
            };
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Exception> {
        let instruction = self.memory.get_instruction(self.pc)?;
        self.pc += 4;
        self.execute(instruction)
    }

    pub fn trigger_exception(&mut self, exception: Exception, exception_handler: Address) {
        self.registers.vaddr = exception.vaddr().unwrap_or_default();
        self.registers.status |= 0x00000002; // sets bit 1
        self.registers.cause &= 0xFFFFFF83; // clears bits 2-6
        self.registers.cause |= exception.code() << 2;
        self.registers.epc = self.pc - 4;
        self.pc = exception_handler;
    }

    pub fn print_crash_handler(&self) {
        debugln!(
            "Interpreter State (pc: {:#08x})\n{}",
            self.pc,
            self.registers,
        )
    }
}
