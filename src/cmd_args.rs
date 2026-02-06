use core::num::ParseIntError;
use std::path::PathBuf;

use clap::{Parser, Subcommand, builder::ValueParser};
use seaside_type_aliases::{Address, Instruction};

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
    /// Runs an assembled MIPS program.
    Run {
        /// The path to a seaside executable file.
        executable_path: PathBuf,

        /// A list of arguments to the program.
        argv: Vec<String>,
    },

    /// Assembles a MIPS assembly file into a seaside executable.
    Assemble {
        /// The path of a file containing MIPS assembly code.
        source: PathBuf,

        /// The file in which to place the assembled executable.
        #[arg(short, long, alias = "out")]
        output_path: Option<PathBuf>,
    },

    /// Disassembles machine code into human-readable assembly.
    Disassemble {
        #[command(subcommand)]
        command: DisassemblyCommand,

        /// The starting address of the instruction(s) to disassemble.
        #[arg(long, alias = "addr", value_parser = ValueParser::new(parse_u32))]
        address: Option<Address>,
    },

    /// Prints the path to an important seaside file.
    #[command(subcommand)]
    Path(PathCommand),

    /// Runs experimental code.
    #[cfg(debug_assertions)]
    Experiment,
}

#[derive(Debug, Subcommand)]
pub enum DisassemblyCommand {
    /// Disassembles a single machine code instruction.
    Instruction {
        /// A machine code instruction.
        #[arg(value_parser = ValueParser::new(parse_u32))]
        instruction: Instruction,
    },

    /// Disassembles a file containing machine code instructions.
    Segment {
        /// A path to a file containing machine code instructions.
        path: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
pub enum PathCommand {
    /// Prints the path to the seaside binary.
    Binary,

    /// Prints the path to the global 'Seaside.toml' file.
    Config {
        /// Ensure the configuration file actually exists before printing it.
        #[arg(long, default_value_t = false)]
        ensure_exists: bool,
    },
}

fn parse_u32(input: &str) -> Result<u32, ParseIntError> {
    if let Some(bits) = input.strip_prefix("0b") {
        u32::from_str_radix(bits, 2)
    } else if let Some(octits) = input.strip_prefix("0o") {
        u32::from_str_radix(octits, 8)
    } else if let Some(hex_digits) = input.strip_prefix("0x") {
        u32::from_str_radix(hex_digits, 16)
    } else {
        input.parse::<u32>()
    }
}
