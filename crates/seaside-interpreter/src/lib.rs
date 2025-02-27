pub mod exception;
pub mod memory;
pub mod register_file;
pub mod syscall_failure;

mod execute;
mod file_handle;
mod rng;

pub use exception::Exception;
pub use memory::Memory;
pub use register_file::RegisterFile;
pub use syscall_failure::SyscallFailureKind;

use anyhow::{anyhow, Result};
use file_handle::FileHandle;
use memory::regions::Region;
use minimal_logging::macros::debugln;
use rng::Rng;
use seaside_config::{features::syscalls::Syscalls, Config};
use seaside_constants::register;
use seaside_type_aliases::Address;
use std::{
    collections::HashMap,
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
};

pub struct Interpreter {
    pub state: InterpreterState,
    services: HashMap<u32, for<'a> fn(&'a mut InterpreterState) -> Result<(), Exception>>,
    pub freeable_heap_allocations: bool,
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
    pub fn init(
        config: &Config,
        text: PathBuf,
        r#extern: Option<PathBuf>,
        data: Option<PathBuf>,
        ktext: Option<PathBuf>,
        kdata: Option<PathBuf>,
        argv: Vec<String>,
    ) -> Result<Self> {
        let memory = Memory::init(config, text, r#extern, data, ktext, kdata)?;
        let pc = memory.initial_pc();
        let services = Self::init_services(
            &config.features.syscalls,
            config.features.freeable_heap_allocations,
        )?;
        let registers = RegisterFile::init(&config.register_defaults);
        let mut files: HashMap<u32, FileHandle> = HashMap::new();
        files.insert(0, FileHandle::new_stdin());
        files.insert(1, FileHandle::new_stdout());
        files.insert(2, FileHandle::new_stderr());
        let mut interpreter = Self {
            state: InterpreterState {
                memory,
                registers,
                pc,
                files,
                next_fd: 3,
                rngs: HashMap::new(),
                stdout_pending_flush: false,
                exit_code: None,
            },
            services,
            freeable_heap_allocations: config.features.freeable_heap_allocations,
            show_crash_handler: config.features.show_crash_handler,
        };
        interpreter
            .state
            .init_argv(
                argv,
                config.memory_map.segments.runtime_data.address_range.limit,
            )
            .map(|_| interpreter)
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        while !self.state.memory.pc_past_end(self.state.pc) && self.state.exit_code.is_none() {
            if let Err(exception) = self.step() {
                let _ = self.state.flush_stdout_if_necessary();
                match self.state.memory.get_exception_handler() {
                    Some(exception_handler) => {
                        self.state.trigger_exception(exception, exception_handler)
                    }
                    None => return Err(exception),
                }
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

    fn init_services(
        syscalls: &Syscalls,
        freeable_heap_allocations: bool,
    ) -> Result<HashMap<u32, for<'a> fn(&'a mut InterpreterState) -> Result<(), Exception>>> {
        use seaside_config::properties::features::syscalls::*;

        let mut services = HashMap::new();
        for (&id, &service_code) in syscalls.iter() {
            let service = match id.to_be_bytes() {
                [mars_print::ID, mars_print::INT] => InterpreterState::print_int,
                [mars_print::ID, mars_print::UINT] => InterpreterState::print_uint,
                [mars_print::ID, mars_print::BIN] => InterpreterState::print_bin,
                [mars_print::ID, mars_print::HEX] => InterpreterState::print_hex,
                [mars_print::ID, mars_print::FLOAT] => InterpreterState::print_float,
                [mars_print::ID, mars_print::DOUBLE] => InterpreterState::print_double,
                [mars_print::ID, mars_print::CHAR] => InterpreterState::print_char,
                [mars_print::ID, mars_print::STRING] => InterpreterState::print_string,
                [mars_read::ID, mars_read::INT] => InterpreterState::read_int,
                [mars_read::ID, mars_read::FLOAT] => InterpreterState::read_float,
                [mars_read::ID, mars_read::DOUBLE] => InterpreterState::read_double,
                [mars_read::ID, mars_read::CHAR] => InterpreterState::read_char,
                [mars_read::ID, mars_read::STRING] => InterpreterState::read_string,
                [mars_file::ID, mars_file::OPEN] => InterpreterState::open_file,
                [mars_file::ID, mars_file::READ] => InterpreterState::read_file,
                [mars_file::ID, mars_file::WRITE] => InterpreterState::write_file,
                [mars_file::ID, mars_file::CLOSE] => InterpreterState::close_file,
                [mars_input_dialog::ID, mars_input_dialog::CONFIRM] => {
                    InterpreterState::confirm_dialog
                }
                [mars_input_dialog::ID, mars_input_dialog::INT] => {
                    InterpreterState::input_dialog_int
                }
                [mars_input_dialog::ID, mars_input_dialog::FLOAT] => {
                    InterpreterState::input_dialog_float
                }
                [mars_input_dialog::ID, mars_input_dialog::DOUBLE] => {
                    InterpreterState::input_dialog_double
                }
                [mars_input_dialog::ID, mars_input_dialog::STRING] => {
                    InterpreterState::input_dialog_string
                }
                [mars_message_dialog::ID, mars_message_dialog::GENERAL] => {
                    InterpreterState::message_dialog
                }
                [mars_message_dialog::ID, mars_message_dialog::INT] => {
                    InterpreterState::message_dialog_int
                }
                [mars_message_dialog::ID, mars_message_dialog::FLOAT] => {
                    InterpreterState::message_dialog_float
                }
                [mars_message_dialog::ID, mars_message_dialog::DOUBLE] => {
                    InterpreterState::message_dialog_double
                }
                [mars_message_dialog::ID, mars_message_dialog::STRING] => {
                    InterpreterState::message_dialog_string
                }
                [mars_system::ID, mars_system::SBRK] if freeable_heap_allocations => {
                    |state: &mut InterpreterState| state.sbrk(true)
                }
                [mars_system::ID, mars_system::SBRK] => {
                    |state: &mut InterpreterState| state.sbrk(false)
                }
                [mars_system::ID, mars_system::EXIT] => InterpreterState::exit,
                [mars_system::ID, mars_system::EXIT_2] => InterpreterState::exit_2,
                [mars_system::ID, mars_system::TIME] => InterpreterState::time,
                [mars_system::ID, mars_system::SLEEP] => InterpreterState::sleep,
                [mars_system::ID, mars_system::MIDI_OUT] => InterpreterState::midi_out,
                [mars_system::ID, mars_system::MIDI_OUT_SYNC] => InterpreterState::midi_out_sync,
                [mars_random::ID, mars_random::SET_SEED] => InterpreterState::set_seed,
                [mars_random::ID, mars_random::INT] => InterpreterState::rand_int,
                [mars_random::ID, mars_random::INT_RANGE] => InterpreterState::rand_int_range,
                [mars_random::ID, mars_random::FLOAT] => InterpreterState::rand_float,
                [mars_random::ID, mars_random::DOUBLE] => InterpreterState::rand_double,
                _ => return Err(anyhow!("0x{id:04x} is not a known system service ID")),
            };
            services.insert(service_code, service);
        }
        Ok(services)
    }
}

impl InterpreterState {
    pub fn trigger_exception(&mut self, exception: Exception, exception_handler: Address) {
        self.registers.vaddr = exception.vaddr().unwrap_or_default();
        self.registers.status |= 0x00000002; // sets bit 1
        self.registers.cause &= 0xffffff83; // clears bits 2-6
        self.registers.cause |= exception.code() << 2;
        self.registers.epc = self.pc - 4;
        self.pc = exception_handler;
    }

    pub fn print_crash_handler(&self) {
        debugln!(
            "Interpreter State (pc: {:#08x})\n{}",
            self.pc,
            self.registers,
        );
    }

    pub fn init_argv(&mut self, argv: Vec<String>, stack_base: Address) -> Result<()> {
        let argc = argv.len() as u32;
        if argc == 0 {
            return Ok(());
        }

        let mut current: Address = stack_base;
        let mut arg_addresses: Vec<Address> = Vec::with_capacity(argv.len());
        for arg in &argv {
            current -= 1;
            for byte in arg.bytes().rev() {
                self.memory.write_u8(current, byte)?;
                current -= 1;
            }
            arg_addresses.push(current + 1);
        }
        let mut stack_frame_base: Address = self.registers.read_u32_from_cpu(register::SP)?;
        if current < stack_frame_base {
            stack_frame_base = current - (current % 4) - 4;
        }
        stack_frame_base -= 4;
        for &arg_address in arg_addresses.iter().rev() {
            self.memory.write_u32(stack_frame_base, arg_address, true)?;
            stack_frame_base -= 4;
        }
        self.memory.write_u32(stack_frame_base, argc, true)?;
        self.registers
            .write_u32_to_cpu(register::SP, stack_frame_base)?;
        self.registers.write_u32_to_cpu(register::A0, argc)?;
        self.registers
            .write_u32_to_cpu(register::A1, stack_frame_base + 4)?;
        Ok(())
    }

    pub fn make_file_handle(&mut self, file: File) -> &mut FileHandle {
        let fd: u32 = self.next_fd;
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
        let seed: u64 = rand::random();
        self.set_rng_seed(id, seed);
        self.rngs.get_mut(&id).unwrap()
    }

    pub fn set_rng_seed(&mut self, id: u32, seed: u64) {
        self.rngs.insert(id, Rng::new(seed));
    }

    pub fn flush_stdout_if_necessary(&mut self) -> Result<()> {
        if self.stdout_pending_flush {
            self.stdout_pending_flush = false;
            stdout().flush()?;
        }
        Ok(())
    }
}
