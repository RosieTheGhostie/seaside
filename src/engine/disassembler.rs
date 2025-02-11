//! Wraps the [`seaside_disassembler`] crate.
//!
//! Provides the wrapper functions [`disassemble_instruction`] and [`disassemble_segment`],
//! which disassemble their respective objects into a human-readable assembly representation.

use anyhow::{Error, Result};
use seaside_config::Config;
use seaside_error::EngineError;
use seaside_int_utils::byte_stream::ByteStream;
use seaside_type_aliases::{Address, Instruction};
use std::path::PathBuf;

/// Prints the human-readable assembly form of `instruction`.
///
/// If `address` is not [`None`], that value is interpreted as the instruction's address for the
/// purposes of branches and jumps.
pub fn disassemble_instruction(instruction: Instruction, address: Option<Address>) -> Result<()> {
    match seaside_disassembler::disassemble_advanced(
        instruction,
        address.unwrap_or_default(),
        address.is_some(),
    ) {
        Some(disassembly) => {
            println!("{disassembly}");
            Ok(())
        }
        None => Err(Error::new(EngineError::MalformedMachineCode)),
    }
}

pub fn disassemble_segment(
    config: Config,
    segment: PathBuf,
    start_address: Option<Address>,
) -> Result<(), Error> {
    let mut address = if let Some(address) = start_address {
        address
    } else if segment.ends_with("text") {
        config.memory_map.segments.text.address_range.base
    } else if segment.ends_with("ktext") {
        config.memory_map.segments.ktext.address_range.base
    } else {
        0
    };
    let bytes = std::fs::read(segment)?;
    for instruction in ByteStream::<'_, u32>::new(&bytes, config.endian) {
        disassemble_instruction(instruction, Some(address))?;
        address += 4;
    }
    Ok(())
}
