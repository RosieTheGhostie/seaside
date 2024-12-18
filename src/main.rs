mod cmd_args;
mod config;
mod constants;
mod engine;
mod sign_extend;
mod type_aliases;

use clap::Parser;
use cmd_args::{CmdArgs, Commands};
use config::{Config, Validate};
use engine::{run, Error as EngineError, ErrorKind as EngineErrorKind};
use minimal_logging::macros::{fatalln, warnln};
use std::{fs::read_to_string, path::PathBuf};
use walkdir::WalkDir;

fn main() {
    let args: CmdArgs = CmdArgs::parse();
    let config: Config = match get_config(&args) {
        Ok(config) => config,
        Err(error) => {
            fatalln!("{error}");
            return;
        }
    };
    if let Err(error) = match args.command {
        Commands::Run { directory } => match engine::init::init(config, directory) {
            Ok(interpreter) => run(interpreter).map(|exit_code| {
                if let Some(exit_code) = exit_code {
                    println!("program terminated with exit code {exit_code}")
                } else {
                    println!("program dropped off the bottom")
                }
            }),
            Err(error) => Err(error),
        },
        Commands::Experiment => experimental_code(),
    } {
        fatalln!("{error}");
    }
}

fn find_seaside_toml() -> Result<PathBuf, EngineError> {
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        if entry.file_name() == "Seaside.toml" {
            return Ok(entry.into_path());
        }
    }
    Err(EngineError::new(
        EngineErrorKind::NotFound,
        "couldn't find `Seaside.toml`",
    ))
}

fn get_config(args: &CmdArgs) -> Result<Config, EngineError> {
    let config_path = match &args.config {
        Some(path) => path,
        None => &find_seaside_toml()?,
    };
    let file_contents = match read_to_string(config_path) {
        Ok(contents) => contents,
        Err(_) => {
            return Err(EngineError::new(
                EngineErrorKind::ExternalFailure,
                "failed to read config file",
            ))
        }
    };
    let config: Config = match toml::from_str(&file_contents) {
        Ok(config) => config,
        Err(error) => return Err(EngineError::new(EngineErrorKind::InvalidConfig, error)),
    };
    config.validate().map(|_| config)
}

fn experimental_code() -> Result<(), EngineError> {
    warnln!("no experimental code to run");
    Ok(())
}
