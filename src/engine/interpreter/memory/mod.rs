pub mod data_memory;
pub mod init;
pub mod instruction_memory;
pub mod regions;

use super::{instruction::Instruction, Exception};
use crate::{config::Endian, type_aliases::address::Address};
use data_memory::DataMemory;
use instruction_memory::InstructionMemory;
use regions::Region;

pub struct Memory {
    instruction_memory: InstructionMemory,
    data_memory: DataMemory,
    endian: Endian,
}

impl Region for Memory {
    fn contains(&self, address: Address) -> bool {
        self.instruction_memory.contains(address) || self.data_memory.contains(address)
    }

    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        self.instruction_memory
            .read_u8(address)
            .or(self.data_memory.read_u8(address))
    }

    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        self.instruction_memory
            .read_u16(address, assert_aligned)
            .or(self.data_memory.read_u16(address, assert_aligned))
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        self.instruction_memory
            .read_u32(address, assert_aligned)
            .or(self.data_memory.read_u32(address, assert_aligned))
    }

    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        self.instruction_memory
            .write_u8(address, value)
            .or(self.data_memory.write_u8(address, value))
    }

    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        self.instruction_memory
            .write_u16(address, value, assert_aligned)
            .or(self.data_memory.write_u16(address, value, assert_aligned))
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        self.instruction_memory
            .write_u32(address, value, assert_aligned)
            .or(self.data_memory.write_u32(address, value, assert_aligned))
    }
}

impl Memory {
    pub fn get_instruction(&self, pc: Address) -> Result<Instruction, Exception> {
        self.instruction_memory.read_u32(pc, true)
    }

    pub fn initial_pc(&self) -> Address {
        self.instruction_memory.initial_pc()
    }

    pub fn pc_past_end(&self, pc: Address) -> bool {
        self.instruction_memory.pc_past_end(pc)
    }
}
