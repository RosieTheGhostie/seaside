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

use anyhow::Result;
use file_handle::FileHandle;
use memory::regions::Region;
use minimal_logging::macros::debugln;
use register_file::IndexByRegister;
use rng::Rng;
use seaside_config::{
    Config,
    features::{
        Service, Services,
        services::{
            mars::{self, Mars},
            spim::{self, Spim},
        },
    },
};
use seaside_constants::register::CpuRegister;
use seaside_type_aliases::Address;
use std::{
    collections::HashMap,
    fs::File,
    io::{Write, stdout},
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
            &config.features.services,
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
            .init_argv(argv, config.memory_map.segments.runtime_data.range.limit)
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
        services: &Services,
        freeable_heap_allocations: bool,
    ) -> Result<HashMap<u32, for<'a> fn(&'a mut InterpreterState) -> Result<(), Exception>>> {
        let mut service_fns = HashMap::new();
        for (&code, &service) in services.iter() {
            let r#fn = match service {
                Service::Spim(Spim::Print(spim::Print::Int)) => InterpreterState::print_int,
                Service::Mars(Mars::Print(mars::Print::Uint)) => InterpreterState::print_uint,
                Service::Mars(Mars::Print(mars::Print::Bin)) => InterpreterState::print_bin,
                Service::Mars(Mars::Print(mars::Print::Hex)) => InterpreterState::print_hex,
                Service::Spim(Spim::Print(spim::Print::Float)) => InterpreterState::print_float,
                Service::Spim(Spim::Print(spim::Print::Double)) => InterpreterState::print_double,
                Service::Spim(Spim::Print(spim::Print::Char)) => InterpreterState::print_char,
                Service::Spim(Spim::Print(spim::Print::String)) => InterpreterState::print_string,
                Service::Spim(Spim::Read(spim::Read::Int)) => InterpreterState::read_int,
                Service::Spim(Spim::Read(spim::Read::Float)) => InterpreterState::read_float,
                Service::Spim(Spim::Read(spim::Read::Double)) => InterpreterState::read_double,
                Service::Spim(Spim::Read(spim::Read::Char)) => InterpreterState::read_char,
                Service::Spim(Spim::Read(spim::Read::String)) => InterpreterState::read_string,
                Service::Spim(Spim::File(spim::File::Open)) => InterpreterState::open_file,
                Service::Spim(Spim::File(spim::File::Read)) => InterpreterState::read_file,
                Service::Spim(Spim::File(spim::File::Write)) => InterpreterState::write_file,
                Service::Spim(Spim::File(spim::File::Close)) => InterpreterState::close_file,
                Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Confirm))) => {
                    InterpreterState::confirm_dialog
                }
                Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Int))) => {
                    InterpreterState::input_dialog_int
                }
                Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Float))) => {
                    InterpreterState::input_dialog_float
                }
                Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Double))) => {
                    InterpreterState::input_dialog_double
                }
                Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::String))) => {
                    InterpreterState::input_dialog_string
                }
                Service::Mars(Mars::Dialog(mars::Dialog::Message(
                    mars::MessageDialog::General,
                ))) => InterpreterState::message_dialog,
                Service::Mars(Mars::Dialog(mars::Dialog::Message(mars::MessageDialog::Int))) => {
                    InterpreterState::message_dialog_int
                }
                Service::Mars(Mars::Dialog(mars::Dialog::Message(mars::MessageDialog::Float))) => {
                    InterpreterState::message_dialog_float
                }
                Service::Mars(Mars::Dialog(mars::Dialog::Message(mars::MessageDialog::Double))) => {
                    InterpreterState::message_dialog_double
                }
                Service::Mars(Mars::Dialog(mars::Dialog::Message(mars::MessageDialog::String))) => {
                    InterpreterState::message_dialog_string
                }
                Service::Spim(Spim::System(spim::System::Sbrk)) if freeable_heap_allocations => {
                    |state: &mut InterpreterState| state.sbrk(true)
                }
                Service::Spim(Spim::System(spim::System::Sbrk)) => {
                    |state: &mut InterpreterState| state.sbrk(false)
                }
                Service::Spim(Spim::System(spim::System::Exit)) => InterpreterState::exit,
                Service::Spim(Spim::System(spim::System::Exit2)) => InterpreterState::exit_2,
                Service::Mars(Mars::System(mars::System::Time)) => InterpreterState::time,
                Service::Mars(Mars::System(mars::System::Sleep)) => InterpreterState::sleep,
                Service::Mars(Mars::System(mars::System::MidiOut)) => InterpreterState::midi_out,
                Service::Mars(Mars::System(mars::System::MidiOutSync)) => {
                    InterpreterState::midi_out_sync
                }
                Service::Mars(Mars::Random(mars::Random::SetSeed)) => InterpreterState::set_seed,
                Service::Mars(Mars::Random(mars::Random::RandInt)) => InterpreterState::rand_int,
                Service::Mars(Mars::Random(mars::Random::RandIntRange)) => {
                    InterpreterState::rand_int_range
                }
                Service::Mars(Mars::Random(mars::Random::RandFloat)) => {
                    InterpreterState::rand_float
                }
                Service::Mars(Mars::Random(mars::Random::RandDouble)) => {
                    InterpreterState::rand_double
                }
            };
            service_fns.insert(code, r#fn);
        }
        Ok(service_fns)
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
        let mut stack_frame_base: Address = self.registers.read(CpuRegister::StackPtr);
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
            .write(CpuRegister::StackPtr, stack_frame_base);
        self.registers.write(CpuRegister::Arg0, argc);
        self.registers
            .write(CpuRegister::Arg1, stack_frame_base + 4);
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
