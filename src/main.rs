mod cmd_args;
mod config;
mod constants;
mod engine;
mod interpreter;
mod sign_extend;
mod type_aliases;

use clap::Parser;
use cmd_args::{CmdArgs, Commands};
use config::Config;
use minimal_logging::macros::{fatalln, grayln, warnln};
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
        Commands::Run { directory } => match engine::init_interpreter(config, directory) {
            Ok(mut interpreter) => engine::run(&mut interpreter).map(|exit_code| {
                if let Some(exit_code) = exit_code {
                    grayln!("program terminated with exit code {exit_code}")
                } else {
                    grayln!("program dropped off the bottom")
                }
            }),
            Err(error) => Err(error),
        },
        Commands::ExePath => print_exe_path(),
        Commands::Experiment => experimental_code(),
    } {
        fatalln!("{error}");
    }
}

fn print_exe_path() -> Result<(), engine::Error> {
    match current_exe() {
        Ok(path) => {
            println!("{}", path.display());
            Ok(())
        }
        Err(_) => Err(engine::Error::new(
            engine::ErrorKind::ExternalFailure,
            "'std::env::current_exe' failed",
        )),
    }
}

fn experimental_code() -> Result<(), engine::Error> {
    warnln!("no experimental code to run");
    Ok(())
}
