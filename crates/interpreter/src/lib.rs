pub mod exception;
pub mod math;
pub mod memory;
pub mod register_file;
pub mod syscall_failure;

pub use exception::Exception;
pub use memory::Memory;
pub use register_file::RegisterFile;
pub use syscall_failure::SyscallFailureKind;

mod execute;
mod file_handle;
mod rng;
mod service;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*},
};

use anyhow::Result;
use minimal_logging::macros::debugln;
use seaside_config::Config;
use seaside_core::{ServiceCode, prelude::*, size};
use seaside_executable::Executable;

use file_handle::FileHandle;
use memory::WriteableRegion;
use register_file::IndexByRegister;
use rng::Rng;
use service::ServiceFn;

pub struct Interpreter {
    pub state: InterpreterState,
    services: HashMap<ServiceCode, ServiceFn>,
    pub show_crash_handler: bool,
}

pub struct InterpreterState {
    pub memory: Memory,
    pub registers: RegisterFile,
    pub pc: Address,
    pub files: HashMap<u32, FileHandle>,
    pub next_fd: u32,
    pub rngs: HashMap<u32, Rng>,
    pub stdout_pending_flush: bool,
    pub exit_code: Option<u8>,
}

impl Interpreter {
    pub const DEFAULT_ARGUMENT_ALLOCATION_SIZE: Size = 4 * size::unsigned::KiB;
    pub const STACK_ALIGNMENT: u8 = 4;

    pub fn init(config: &Config, executable: Executable, argv: Vec<String>) -> Result<Self> {
        let seaside_executable::Body {
            flags,
            memory_map,
            services,
            debug_info: _, // TODO: implement debugging
            segments,
        } = executable.body;

        let memory = Memory::new(&memory_map, &segments, flags);
        let pc = memory.initial_pc();
        let services = Self::init_services(&services, flags);

        let mut registers = RegisterFile::default();
        registers.write(
            CpuRegister::GlobalPtr,
            memory_map.segments.r#extern.range.upper_midpoint(),
        );
        registers.write(
            CpuRegister::StackPtr,
            (memory.stack_base() & Self::STACK_ALIGNMENT_MASK)
                - Self::DEFAULT_ARGUMENT_ALLOCATION_SIZE,
        );

        let mut files: HashMap<u32, FileHandle> = HashMap::new();
        files.insert(0, FileHandle::new_stdin());
        files.insert(1, FileHandle::new_stdout());
        files.insert(2, FileHandle::new_stderr());

        let mut state = InterpreterState {
            memory,
            registers,
            pc,
            files,
            next_fd: 3,
            rngs: HashMap::new(),
            stdout_pending_flush: false,
            exit_code: None,
        };
        state.init_argv(argv)?;

        Ok(Self {
            state,
            services,
            show_crash_handler: config.features.show_crash_handler,
        })
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        while !self.state.memory.pc_past_end(self.state.pc) && self.state.exit_code.is_none() {
            if let Err(exception) = self.step() {
                let _ = self.state.flush_stdout_if_necessary();
                let Some(exception_handler) = self.state.memory.get_exception_handler() else {
                    return Err(exception);
                };

                self.state.trigger_exception(exception, exception_handler)
            };
        }

        let _ = self.state.flush_stdout_if_necessary();

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Exception> {
        let instruction = self.state.memory.get_instruction(self.state.pc)?;
        self.state.pc += 4;
        self.execute(instruction)
    }

    const STACK_ALIGNMENT_MASK: Address = Address::MAX << Self::STACK_ALIGNMENT.ilog2();
}

impl InterpreterState {
    pub fn trigger_exception(&mut self, exception: Exception, exception_handler: Address) {
        self.registers.vaddr = exception.vaddr().unwrap_or_default();
        self.registers.status.set_exception_level(true);
        self.registers.cause &= 0xffffff83; // clears bits 2-6
        self.registers.cause |= exception.code() << 2;
        self.registers.epc = self.pc - 4;
        self.pc = exception_handler;
    }

    pub fn print_crash_handler(&self) {
        debugln!(
            "Interpreter State (pc: {:#010x})\n{}",
            self.pc,
            self.registers,
        );
    }

    /// Initialize memory and registers with any program arguments that might exist.
    ///
    /// This implementation is more or less a direct translation of MARS', which (as of writing
    /// this) can be found [here] in the `storeProgramArguments` method of the `ProgramArgumentList`
    /// class.
    ///
    /// [here]: https://github.com/dpetersanderson/MARS/blob/main/mars/simulator/ProgramArgumentList.java
    pub fn init_argv(&mut self, argv: Vec<String>) -> Result<(), Exception> {
        const DEFAULT_ARGUMENT_ALLOCATION_SIZE: Size = 4 * size::unsigned::KiB;

        let argc: Size = argv.len() as _;
        if argc == 0 {
            return Ok(());
        }

        let stack_base: Address = self.memory.stack_base();
        let mut current: Address = stack_base;

        let mut arg_addresses: Vec<Address> = Vec::with_capacity(argv.len());
        for arg in &argv {
            // Add a nul byte to the end of the string.
            current -= 1;

            // Write the string to memory.
            //
            // We're writing things backwards because the stack grows towards 0x00000000.
            for byte in arg.bytes().rev() {
                self.memory.write_u8(current, byte)?;
                current -= 1;
            }

            // `current` will point to the byte just before the start of the string.
            arg_addresses.push(current + 1);
        }

        // We need the stack pointer to be word-aligned, so we unset the lower two bits.
        let mut stack_ptr: Address = (stack_base - DEFAULT_ARGUMENT_ALLOCATION_SIZE) & 0xffff_fffc;
        if current < stack_ptr {
            // Compensate for default argument allocation being too small.
            stack_ptr = current - (current % 4) - 4;
        }

        // Add a 0 byte to the end of the `argv` array (for some reason).
        stack_ptr -= 4;

        // Write `argv` onto the stack.
        for arg_address in arg_addresses.into_iter().rev() {
            self.memory.write_u32(stack_ptr, arg_address, true)?;
            stack_ptr -= 4;
        }

        self.memory.write_u32(stack_ptr, argc, true)?;
        self.registers.write(CpuRegister::StackPtr, stack_ptr);
        self.registers.write(CpuRegister::Arg0, argc);
        self.registers.write(CpuRegister::Arg1, stack_ptr + 4);

        Ok(())
    }

    pub fn make_file_handle(&mut self, file: File) -> &mut FileHandle {
        let fd = self.next_fd;
        self.files.insert(fd, FileHandle::File(file));
        self.next_fd += 1;
        self.files.get_mut(&fd).unwrap()
    }

    pub fn close_file_handle(&mut self, fd: u32) -> bool {
        if let Some(FileHandle::File(_)) = self.files.get_mut(&fd) {
            self.files.remove(&fd);
            true
        } else {
            false
        }
    }

    pub fn make_rng(&mut self, id: u32) -> &mut Rng {
        self.set_rng_seed(id, rand::random());
        self.rngs.get_mut(&id).unwrap()
    }

    pub fn set_rng_seed(&mut self, id: u32, seed: u64) {
        self.rngs.insert(id, Rng::new(seed));
    }

    pub fn read_line_from_stdin() -> Result<String, Exception> {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok() {
            Ok(buffer)
        } else {
            Err(Exception::SyscallFailure(
                SyscallFailureKind::StdinReadFailed,
            ))
        }
    }

    pub fn flush_stdout_if_necessary(&mut self) -> Result<(), Exception> {
        if self.stdout_pending_flush {
            self.stdout_pending_flush = false;
            io::stdout()
                .flush()
                .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdoutFlushFailed))
        } else {
            Ok(())
        }
    }
}
