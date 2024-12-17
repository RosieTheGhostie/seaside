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
}
