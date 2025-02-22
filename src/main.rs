mod cmd_args;
mod engine;

use anyhow::{Error, Result};
use clap::Parser;
use cmd_args::{AssemblyArgs, CmdArgs, Commands, DisassemblyArgs, DisassemblyTarget, RunArgs};
use minimal_logging::macros::{fatalln, grayln};
use seaside_config::Config;
use std::env::current_exe;

fn main() {
    let args: CmdArgs = CmdArgs::parse();
    let config: Config = match engine::get_config(&args) {
        Ok(config) => config,
        Err(error) => {
            fatalln!("{error}");
            return;
        }
    };
    if let Err(error) = match args.command {
        Commands::Run(RunArgs { directory, argv }) => {
            match engine::init_interpreter(config, directory, argv) {
                Ok(mut interpreter) => engine::run(&mut interpreter).map(|exit_code| {
                    if let Some(exit_code) = exit_code {
                        grayln!("program terminated with exit code {exit_code}")
                    } else {
                        grayln!("program dropped off the bottom")
                    }
                }),
                Err(error) => Err(error),
            }
        }
        Commands::Assemble(AssemblyArgs {
            source,
            output_directory,
        }) => engine::assemble(config, source, output_directory),
        Commands::Disassemble(DisassemblyArgs {
            target:
                DisassemblyTarget {
                    instruction: Some(instruction),
                    segment: None,
                },
            address: start_address,
        }) => engine::disassemble_instruction(instruction, start_address),
        Commands::Disassemble(DisassemblyArgs {
            target:
                DisassemblyTarget {
                    instruction: None,
                    segment: Some(segment),
                },
            address: start_address,
        }) => engine::disassemble_segment(config, segment, start_address),
        Commands::ExePath => print_exe_path(),
        #[cfg(debug_assertions)]
        Commands::Experiment => experimental_code(),
        _ => unreachable!("disassemble subcommand will always have exactly one argument"),
    } {
        fatalln!("{error}");
    }
}

fn print_exe_path() -> Result<(), Error> {
    println!("{}", current_exe()?.display());
    Ok(())
}

#[cfg(debug_assertions)]
fn experimental_code() -> Result<()> {
    // minimal_logging::macros::warnln!("no experimental code to run");
    seaside_config_editor::new_editor().run();
    Ok(())
}
