pub mod init;
mod regions;

use crate::config::{memory_map::Address, Endian};
use regions::{DataRegion, TextRegion};

pub struct Memory {
    instruction_memory: InstructionMemory,
    data_memory: DataMemory,
    endian: Endian,
}

struct InstructionMemory {
    text: TextRegion,
    ktext: TextRegion,
    writeable: bool,
}

struct DataMemory {
    r#extern: DataRegion,
    data: DataRegion,
    heap: DataRegion,
    stack: DataRegion,
    kdata: DataRegion,
    mmio: DataRegion,
}

impl Memory {
    pub fn initial_pc(&self) -> Address {
        self.instruction_memory.text.addresses.start
    }
}
