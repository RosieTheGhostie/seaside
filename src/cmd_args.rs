use clap::{arg, command, Parser, Subcommand};
use std::path::PathBuf;

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
    /// Prints the file path of the seaside executable.
    ExePath,
    /// Runs experimental code.
    Experiment,
}
