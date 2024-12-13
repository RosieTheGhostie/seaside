mod cmd_args;
mod config;

use clap::Parser;
use cmd_args::{CmdArgs, Commands};
use config::Config;
use std::{
    error::Error as ErrorTrait,
    fs::read_to_string,
    io::{Error, ErrorKind},
    path::PathBuf,
};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn ErrorTrait>> {
    let args: CmdArgs = CmdArgs::parse();
    match args.command {
        Commands::Run {
            text: _,
            data: _,
            ktext: _,
            kdata: _,
        } => run(args)?,
        Commands::Experiment => experimental_code(args)?,
    };
    Ok(())
}

fn run(_args: CmdArgs) -> Result<(), Error> {
    todo!("the interpreter doesn't even exist yet lmao")
}

fn find_seaside_toml() -> Result<PathBuf, Error> {
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        if entry.file_name() == "Seaside.toml" {
            return Ok(entry.into_path());
        }
    }
    Err(Error::from(ErrorKind::NotFound))
}

fn experimental_code(args: CmdArgs) -> Result<(), Box<dyn ErrorTrait>> {
    let config_path = match args.config {
        Some(path) => path,
        None => find_seaside_toml()?,
    };
    let config: Config = toml::from_str(&read_to_string(config_path)?)?;
    println!("{}", toml::to_string_pretty(&config)?);
    Ok(())
}
