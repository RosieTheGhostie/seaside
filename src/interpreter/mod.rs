#![allow(dead_code)]
pub mod exception;
pub mod memory;
pub mod register_file;

mod execute;
mod file_handle;
mod rng;
mod syscalls;

pub use exception::Exception;
pub use memory::Memory;
pub use register_file::RegisterFile;
pub use syscalls::Syscalls;

use crate::{config::Config, engine::Error, type_aliases::address::Address};
use file_handle::FileHandle;
use minimal_logging::macros::debugln;
use rng::Rng;
use std::{
    collections::HashMap,
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
};

pub struct Interpreter {
    memory: Memory,
    registers: RegisterFile,
    pc: Address,
    syscalls: Syscalls,
    files: HashMap<u32, FileHandle>,
    next_fd: u32,
    rngs: HashMap<u32, Rng>,
    pub show_crash_handler: bool,
    stdout_pending_flush: bool,
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
        let mut files: HashMap<u32, FileHandle> = HashMap::new();
        files.insert(0, FileHandle::new_stdin());
        files.insert(1, FileHandle::new_stdout());
        files.insert(2, FileHandle::new_stderr());
        Ok(Self {
            memory,
            registers,
            pc,
            syscalls,
            files,
            next_fd: 3,
            rngs: HashMap::new(),
            show_crash_handler: config.features.show_crash_handler,
            stdout_pending_flush: false,
            exit_code: None,
        })
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        while !self.memory.pc_past_end(self.pc) && self.exit_code.is_none() {
            if let Err(exception) = self.step() {
                let _ = self.flush_stdout_if_necessary();
                match self.memory.get_exception_handler() {
                    Some(exception_handler) => self.trigger_exception(exception, exception_handler),
                    None => return Err(exception),
                }
            };
        }
        let _ = self.flush_stdout_if_necessary();
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

    fn make_file_handle(&mut self, file: File) -> &mut FileHandle {
        let fd: u32 = self.next_fd;
        self.files.insert(fd, FileHandle::File(file));
        self.next_fd += 1;
        self.files.get_mut(&fd).unwrap()
    }

    fn close_file_handle(&mut self, fd: u32) -> bool {
        if let Some(FileHandle::File(_)) = self.files.get_mut(&fd) {
            self.files.remove(&fd);
            true
        } else {
            false
        }
    }

    fn make_rng(&mut self, id: u32) -> &mut Rng {
        let seed: u64 = rand::random();
        self.set_rng_seed(id, seed);
        self.rngs.get_mut(&id).unwrap()
    }

    fn set_rng_seed(&mut self, id: u32, seed: u64) {
        self.rngs.insert(id, Rng::new(seed));
    }

    fn flush_stdout_if_necessary(&mut self) -> Result<(), std::io::Error> {
        if self.stdout_pending_flush {
            self.stdout_pending_flush = false;
            stdout().flush()
        } else {
            Ok(())
        }
    }
}
