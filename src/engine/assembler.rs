//! Wraps the [`seaside_assembler`] crate.
//!
//! Provides the wrapper function [`assemble`], which runs the assembler routine.

use anyhow::Result;
use minimal_logging::macros::grayln;
use seaside_assembler::Assembler;
use seaside_config::Config;
use std::{fs::read_to_string, path::PathBuf, str::FromStr, time::Instant};

/// Assembles `source` into a format usable by the seaside interpreter.
///
/// If `output_directory` is [`None`], it defaults to the current working directory.
pub fn assemble(config: Config, source: PathBuf, output_directory: Option<PathBuf>) -> Result<()> {
    let start_time = Instant::now();
    let output_directory = output_directory.unwrap_or_else(|| PathBuf::from_str(".").unwrap());
    let source_code = read_to_string(&source)?;
    let mut assembler = Assembler::init(&config, &source_code);
    assembler.build()?;
    assembler.export(&output_directory)?;
    let elapsed = start_time.elapsed();
    grayln!("assembled {source:?} in {elapsed:#?}");
    Ok(())
}
