use seaside_executable::{
    MemoryMap, Segment, Segments,
    memory_map::{self, SegmentInfo},
};
use seaside_int_utils::Endian;

use super::{DataMemory, DataRegion, InstructionMemory, Memory, TextRegion};

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
            executable_flags.contains(seaside_executable::Flags::SELF_MODIFYING_CODE),
        );
        let [stack, heap] = init_stack_and_heap(&memory_map.segments.stack_and_heap);
        let data_memory = DataMemory::new(
            init_data_region(&memory_map.segments.r#extern, segments.r#extern.as_ref()),
            init_data_region(&memory_map.segments.data, segments.data.as_ref()),
            heap,
            stack,
            init_data_region(&memory_map.segments.kdata, segments.kdata.as_ref()),
            init_data_region(&memory_map.segments.mmio, None),
        );

        Self {
            instruction_memory,
            data_memory,
            endian,
        }
    }
}

fn init_text_region(
    segment_info: &SegmentInfo,
    segment: Option<&Segment>,
    endian: Endian,
) -> TextRegion {
    let mut region = TextRegion::new(segment_info.range.base(), segment_info.allocate as _);
    if let Some(segment) = segment {
        region.populate_instructions(segment.iter_as_text(endian));
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
