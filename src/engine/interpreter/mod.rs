#![allow(dead_code)]
pub mod exception;
mod execute;
pub mod init;
mod instruction;
pub mod memory;
pub mod register_file;
mod syscalls;

use crate::config::memory_map::Address;
use memory::Memory;
use register_file::RegisterFile;
use syscalls::Syscalls;

pub struct Interpreter {
    memory: Memory,
    registers: RegisterFile,
    pc: Address,
    syscalls: Syscalls,
}
