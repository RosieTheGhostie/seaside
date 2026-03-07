//! Wraps the [`seaside_disassembler`] crate.
//!
//! Provides the wrapper functions [`disassemble_instruction`] and [`disassemble_segment`],
//! which disassemble their respective objects into a human-readable assembly representation.

use std::path::Path;

use anyhow::{Error, Result};
use seaside_config::Config;
use seaside_core::{ByteStream, EngineError, prelude::*};

/// Prints the human-readable assembly form of `instruction`.
///
/// If `address` is not [`None`], that value is interpreted as the instruction's address for the
/// purposes of branches and jumps.
pub fn disassemble_instruction(instruction: Instruction, address: Option<Address>) -> Result<()> {
    if let Some(disassembly) = seaside_disassembler::disassemble_advanced(
        instruction,
        address.unwrap_or_default(),
        address.is_some(),
    ) {
        println!("{disassembly}");
        Ok(())
    } else {
        Err(Error::new(EngineError::MalformedMachineCode))
    }
}

/// Prints the human-readable assembly form of the segment at the specified path.
///
/// If `address` is not [`None`], that value is interpreted as the instruction's address for the
/// purposes of branches and jumps.
pub fn disassemble_segment<P>(
    config: Config,
    segment: P,
    start_address: Option<Address>,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let segment = segment.as_ref();
    let mut address = if let Some(address) = start_address {
        address
    } else if segment.ends_with("text") {
        config.memory_map.segments.text.range.base()
    } else if segment.ends_with("ktext") {
        config.memory_map.segments.ktext.range.base()
    } else {
        0
    };
    let bytes = std::fs::read(segment)?;
    for instruction in ByteStream::<'_, Instruction>::new(&bytes, config.endian) {
        disassemble_instruction(instruction, Some(address))?;
        address += size_of::<Instruction>() as UnsignedOffset;
    }

    Ok(())
}
