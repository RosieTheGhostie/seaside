pub mod data_memory;
pub mod init;
pub mod instruction_memory;
pub mod regions;

use super::Exception;
use data_memory::DataMemory;
use instruction_memory::InstructionMemory;
use regions::Region;
use seaside_int_utils::Endian;
use seaside_type_aliases::{Address, Instruction};

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

    fn read_u64(&self, address: Address, assert_aligned: bool) -> Result<u64, Exception> {
        self.instruction_memory
            .read_u64(address, assert_aligned)
            .or(self.data_memory.read_u64(address, assert_aligned))
    }

    fn get_slice(&self, address: Address) -> Result<&[u8], Exception> {
        // I'm checking data memory first on purpose.
        self.data_memory
            .get_slice(address)
            .or(self.instruction_memory.get_slice(address))
    }

    fn get_slice_mut(&mut self, address: Address) -> Result<&mut [u8], Exception> {
        // I'm checking data memory first on purpose.
        self.data_memory
            .get_slice_mut(address)
            .or(self.instruction_memory.get_slice_mut(address))
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

    fn write_u64(
        &mut self,
        address: Address,
        value: u64,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        self.instruction_memory
            .write_u64(address, value, assert_aligned)
            .or(self.data_memory.write_u64(address, value, assert_aligned))
    }
}

impl Memory {
    pub fn endian(&self) -> &Endian {
        &self.endian
    }

    pub fn get_instruction(&self, pc: Address) -> Result<Instruction, Exception> {
        self.instruction_memory.read_u32(pc, true)
    }

    pub fn get_exception_handler(&self) -> Option<Address> {
        self.instruction_memory.exception_handler
    }

    pub fn initial_pc(&self) -> Address {
        self.instruction_memory.initial_pc()
    }

    pub fn pc_past_end(&self, pc: Address) -> bool {
        self.instruction_memory.pc_past_end(pc)
    }

    pub fn free_heap_space(&self) -> &u32 {
        &self.data_memory.free_heap_space
    }

    pub fn free_heap_space_mut(&mut self) -> &mut u32 {
        &mut self.data_memory.free_heap_space
    }

    pub fn used_heap_space(&self) -> u32 {
        self.data_memory.used_heap_space()
    }

    pub fn next_heap_address(&self) -> &Address {
        &self.data_memory.next_heap_address
    }

    pub fn next_heap_address_mut(&mut self) -> &mut Address {
        &mut self.data_memory.next_heap_address
    }
}
