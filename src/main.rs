mod cmd_args;
mod config;
mod engine;

use clap::Parser;
use cmd_args::{CmdArgs, Commands};
use config::Config;
use engine::{Error as EngineError, ErrorKind as EngineErrorKind};
use minimal_logging::{
    attributes::to_be_implemented,
    macros::{debugln, fatalln},
};
use std::{fs::read_to_string, path::PathBuf};
use walkdir::WalkDir;

fn main() {
    let args: CmdArgs = CmdArgs::parse();
    if let Err(error) = match args.command {
        Commands::Run {
            text: _,
            data: _,
            ktext: _,
            kdata: _,
        } => run(args),
        Commands::Experiment => experimental_code(args),
    } {
        fatalln!("{error}");
    }
}

#[to_be_implemented(Ok(()))]
fn run(_args: CmdArgs) -> Result<(), EngineError>;

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

fn experimental_code(args: CmdArgs) -> Result<(), EngineError> {
    let config_path = match args.config {
        Some(path) => path,
        None => find_seaside_toml()?,
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
    let debug_config_view = match toml::to_string_pretty(&config) {
        Ok(string) => string,
        Err(error) => return Err(EngineError::new(EngineErrorKind::InternalLogicIssue, error)),
    };
    debugln!("Parsed Config:\n{debug_config_view}");
    Ok(())
}
