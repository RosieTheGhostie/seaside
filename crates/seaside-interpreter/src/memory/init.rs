use super::{DataMemory, DataRegion, InstructionMemory, Memory, TextRegion};
use anyhow::Result;
use seaside_config::{
    Config,
    memory_map::{RuntimeData, Segment},
};
use seaside_int_utils::Endian;
use std::path::PathBuf;

impl Memory {
    pub fn init(
        config: &Config,
        text: PathBuf,
        r#extern: Option<PathBuf>,
        data: Option<PathBuf>,
        ktext: Option<PathBuf>,
        kdata: Option<PathBuf>,
    ) -> Result<Self> {
        let segments = &config.memory_map.segments;
        let instruction_memory = InstructionMemory::new(
            init_text_region(&segments.text, Some(text), config.endian)?,
            init_text_region(&segments.ktext, ktext, config.endian)?,
            config.memory_map.exception_handler,
            config.features.self_modifying_code,
        );
        let [heap, stack] = init_heap_and_stack(&segments.runtime_data);
        let data_memory = DataMemory::new(
            init_data_region(&segments.r#extern, r#extern)?,
            init_data_region(&segments.data, data)?,
            heap,
            stack,
            init_data_region(&segments.kdata, kdata)?,
            init_data_region(&segments.mmio, None)?,
        );
        Ok(Self {
            instruction_memory,
            data_memory,
            endian: config.endian,
        })
    }
}

fn init_text_region(
    segment: &Segment,
    path: Option<PathBuf>,
    endian: Endian,
) -> Result<TextRegion> {
    let mut region = TextRegion::new(segment.range.base, segment.allocate as usize);
    if let Some(path) = path {
        region.populate(std::fs::read(path)?, endian);
    }
    Ok(region)
}

fn init_data_region(segment: &Segment, path: Option<PathBuf>) -> Result<DataRegion> {
    let mut region = DataRegion::new(segment.range.base, segment.allocate as usize);
    if let Some(path) = path {
        region.populate(std::fs::read(path)?);
    }
    Ok(region)
}

fn init_heap_and_stack(runtime_data: &RuntimeData) -> [DataRegion; 2] {
    let heap_low_address = runtime_data.range.base;
    let stack_low_address = runtime_data.range.limit - runtime_data.stack_size + 1;
    [
        DataRegion::new(heap_low_address, runtime_data.heap_size as usize),
        DataRegion::new(stack_low_address, runtime_data.stack_size as usize),
    ]
}
