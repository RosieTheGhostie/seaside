use super::{Error, ErrorKind};
use crate::{
    byte_stream::ByteStream,
    type_aliases::{Address, Instruction},
    Config,
};
use std::path::PathBuf;

pub fn disassemble(instruction: Instruction, address: Option<Address>) -> Result<(), Error> {
    match crate::disassembler::disassemble_advanced(
        instruction,
        address.unwrap_or_default(),
        address.is_some(),
    ) {
        Some(disassembly) => {
            println!("{disassembly}");
            Ok(())
        }
        None => Err(Error::from(ErrorKind::MalformedMachineCode)),
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
    let bytes = std::fs::read(segment)
        .map_err(|_| Error::new(ErrorKind::NotFound, "couldn't find that segment"))?;
    for instruction in ByteStream::<'_, u32>::new(&bytes, config.endian) {
        disassemble(instruction, Some(address))?;
        address += 4;
    }
    Ok(())
}
