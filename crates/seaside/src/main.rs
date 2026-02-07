mod cmd_args;
mod engine;

use anyhow::Result;
use clap::Parser;
use minimal_logging::macros::{fatalln, grayln};

use cmd_args::{CmdArgs, Commands, DisassembleCommand, PathCommand};

fn main() {
    let args = CmdArgs::parse();
    let config = match engine::get_config(&args) {
        Ok(config) => config,
        Err(error) => {
            fatalln!("{error}");
            return;
        }
    };
    if let Err(err) = match args.command {
        Commands::Run {
            executable_path,
            argv,
        } => match engine::init_interpreter(config, executable_path, argv) {
            Ok(mut interpreter) => engine::run(&mut interpreter).map(|exit_code| {
                if let Some(exit_code) = exit_code {
                    grayln!("program terminated with exit code {exit_code}")
                } else {
                    grayln!("program dropped off the bottom")
                }
            }),
            Err(error) => Err(error),
        },

        Commands::Assemble {
            source,
            output_path,
        } => engine::assemble(config, source, output_path),

        Commands::Disassemble {
            command: DisassembleCommand::Instruction { instruction },
            address: start_address,
        } => engine::disassemble_instruction(instruction, start_address),

        Commands::Disassemble {
            command: DisassembleCommand::Segment { path: segment },
            address: start_address,
        } => engine::disassemble_segment(config, segment, start_address),

        Commands::Path(PathCommand::Binary) => print_exe_path(),
        Commands::Path(PathCommand::Config { ensure_exists }) => print_config_path(ensure_exists),

        #[cfg(debug_assertions)]
        Commands::Experiment => experimental_code(),
    } {
        fatalln!("{err}");
    }
}

fn print_exe_path() -> Result<()> {
    println!("{}", std::env::current_exe()?.display());
    Ok(())
}

fn print_config_path(ensure_exists: bool) -> Result<()> {
    println!("{}", engine::find_global_config(ensure_exists)?.display());
    Ok(())
}

#[cfg(debug_assertions)]
fn experimental_code() -> Result<()> {
    minimal_logging::macros::warnln!("no experimental code to run");
    Ok(())
}
