use crate::type_aliases::instruction::Instruction;
use clap::{arg, builder::ValueParser, command, Args, Parser, Subcommand};
use std::{num::ParseIntError, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CmdArgs {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long)]
    /// An explicit path to 'Seaside.toml'.
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Runs an assembled MIPS program in the specified project directory.
    Run { directory: PathBuf },
    /// Disassembles the input machine code into human-readable assembly.
    Disassemble(DisassemblyArgs),
    /// Prints the file path of the seaside executable.
    ExePath,
    /// Runs experimental code.
    #[cfg(debug_assertions)]
    Experiment,
}

#[derive(Args, Debug)]
pub struct DisassemblyArgs {
    #[command(flatten)]
    pub target: DisassemblyTarget,
    /// The starting address of the instruction(s) to disassemble.
    #[arg(long, alias = "addr", value_parser = ValueParser::new(parse_u32))]
    pub address: Option<u32>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct DisassemblyTarget {
    /// A machine code instruction.
    #[arg(long, value_parser = ValueParser::new(parse_u32))]
    pub instruction: Option<Instruction>,
    /// The path of a file containing machine code instructions.
    #[arg(long)]
    pub segment: Option<PathBuf>,
}

fn parse_u32(input: &str) -> Result<Instruction, ParseIntError> {
    if let Some(bits) = input.strip_prefix("0b") {
        Instruction::from_str_radix(bits, 2)
    } else if let Some(octits) = input.strip_prefix("0o") {
        Instruction::from_str_radix(octits, 8)
    } else if let Some(hex_digits) = input.strip_prefix("0x") {
        Instruction::from_str_radix(hex_digits, 16)
    } else {
        input.parse::<Instruction>()
    }
}
