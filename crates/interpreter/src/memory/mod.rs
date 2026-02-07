pub mod data_memory;
pub mod instruction_memory;

pub use data_memory::DataMemory;
pub use instruction_memory::InstructionMemory;

mod regions;

use seaside_executable::{
    MemoryMap, Segment, Segments,
    memory_map::{self, SegmentInfo},
};
use seaside_int_utils::Endian;
use seaside_type_aliases::{Address, Instruction, Size};

use crate::Exception;
use regions::{DataRegion, TextRegion};
pub(crate) use regions::{
    ReadableRegion, Region, SliceableRegion, SliceableRegionMut, WriteableRegion,
};

pub struct Memory {
    instruction_memory: InstructionMemory,
    data_memory: DataMemory,
    endian: Endian,
}

impl Memory {
    pub fn new(
        memory_map: &MemoryMap,
        segments: &Segments,
        executable_flags: seaside_executable::Flags,
    ) -> Self {
        let endian = executable_flags.endian();
        let instruction_memory = InstructionMemory::new(
            init_text_region(&memory_map.segments.text, Some(&segments.text), endian),
            init_text_region(&memory_map.segments.ktext, segments.ktext.as_ref(), endian),
            memory_map.exception_handler,
            executable_flags.self_modifying_code(),
        );
        let [stack, heap] = init_stack_and_heap(&memory_map.segments.stack_and_heap);
        let data_memory = DataMemory::new(
            init_data_region(&memory_map.segments.r#extern, segments.r#extern.as_ref()),
            init_data_region(&memory_map.segments.data, segments.data.as_ref()),
            init_data_region(&memory_map.segments.kdata, segments.kdata.as_ref()),
            stack,
            heap,
            init_data_region(&memory_map.segments.mmio, None),
        );

        Self {
            instruction_memory,
            data_memory,
            endian,
        }
    }

    pub const fn endian(&self) -> Endian {
        self.endian
    }

    pub fn get_instruction(&self, pc: Address) -> Result<Instruction, Exception> {
        self.instruction_memory.read_u32(pc, true)
    }

    pub const fn get_exception_handler(&self) -> Option<Address> {
        self.instruction_memory.exception_handler
    }

    pub const fn initial_pc(&self) -> Address {
        self.instruction_memory.initial_pc()
    }

    pub const fn pc_past_end(&self, pc: Address) -> bool {
        self.instruction_memory.pc_past_end(pc)
    }

    pub const fn stack_base(&self) -> Address {
        self.data_memory.stack_base()
    }

    pub const fn free_heap_space(&self) -> Size {
        self.data_memory.free_heap_space
    }

    pub const fn free_heap_space_mut(&mut self) -> &mut Size {
        &mut self.data_memory.free_heap_space
    }

    pub fn used_heap_space(&self) -> Size {
        self.data_memory.used_heap_space()
    }

    pub const fn next_heap_address(&self) -> Address {
        self.data_memory.next_heap_address
    }

    pub const fn next_heap_address_mut(&mut self) -> &mut Address {
        &mut self.data_memory.next_heap_address
    }
}

impl Region for Memory {
    fn contains(&self, address: Address) -> bool {
        self.instruction_memory.contains(address) || self.data_memory.contains(address)
    }
}

impl ReadableRegion for Memory {
    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        self.instruction_memory
            .read_u8(address)
            .or_else(|_| self.data_memory.read_u8(address))
    }

    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        self.instruction_memory
            .read_u16(address, assert_aligned)
            .or_else(|_| self.data_memory.read_u16(address, assert_aligned))
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        self.instruction_memory
            .read_u32(address, assert_aligned)
            .or_else(|_| self.data_memory.read_u32(address, assert_aligned))
    }

    fn read_u64(&self, address: Address, assert_aligned: bool) -> Result<u64, Exception> {
        self.instruction_memory
            .read_u64(address, assert_aligned)
            .or_else(|_| self.data_memory.read_u64(address, assert_aligned))
    }
}

impl WriteableRegion for Memory {
    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        self.instruction_memory
            .write_u8(address, value)
            .or_else(|_| self.data_memory.write_u8(address, value))
    }

    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        self.instruction_memory
            .write_u16(address, value, assert_aligned)
            .or_else(|_| self.data_memory.write_u16(address, value, assert_aligned))
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        self.instruction_memory
            .write_u32(address, value, assert_aligned)
            .or_else(|_| self.data_memory.write_u32(address, value, assert_aligned))
    }

    fn write_u64(
        &mut self,
        address: Address,
        value: u64,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        self.instruction_memory
            .write_u64(address, value, assert_aligned)
            .or_else(|_| self.data_memory.write_u64(address, value, assert_aligned))
    }
}

impl SliceableRegion for Memory {
    fn get_slice(&self, address: Address) -> Result<&[u8], Exception> {
        self.data_memory.get_slice(address)
    }
}

impl SliceableRegionMut for Memory {
    fn get_slice_mut(&mut self, address: Address) -> Result<&mut [u8], Exception> {
        self.data_memory.get_slice_mut(address)
    }
}

fn init_text_region(
    segment_info: &SegmentInfo,
    segment: Option<&Segment>,
    endian: Endian,
) -> TextRegion {
    let mut region = TextRegion::new(segment_info.range.base(), segment_info.allocate as _);
    if let Some(segment) = segment {
        region.populate_instructions(segment.iter_as_text(endian), endian);
    }

    region
}

fn init_data_region(segment_info: &SegmentInfo, segment: Option<&Segment>) -> DataRegion {
    let mut region = DataRegion::new(segment_info.range.base(), segment_info.allocate as _);
    if let Some(segment) = segment {
        region.populate(&**segment);
    }

    region
}

fn init_stack_and_heap(stack_and_heap: &memory_map::StackAndHeap) -> [DataRegion; 2] {
    let stack_low_address = stack_and_heap.range.limit() - stack_and_heap.allocate_stack + 1;
    let heap_low_address = stack_and_heap.range.base();
    [
        DataRegion::new(stack_low_address, stack_and_heap.allocate_stack as _),
        DataRegion::new(heap_low_address, stack_and_heap.allocate_heap as _),
    ]
}
