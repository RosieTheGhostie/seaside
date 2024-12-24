pub mod error;

pub use error::{Error, ErrorKind};

use crate::{
    cmd_args::CmdArgs,
    config::{Config, Validate},
    interpreter::Interpreter,
};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn get_config(args: &CmdArgs) -> Result<Config, Error> {
    let config_path = match &args.config {
        Some(path) => path,
        None => &find_seaside_toml()?,
    };
    let file_contents = match read_to_string(config_path) {
        Ok(contents) => contents,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::ExternalFailure,
                "failed to read config file",
            ))
        }
    };
    let config: Config = match toml::from_str(&file_contents) {
        Ok(config) => config,
        Err(error) => return Err(Error::new(ErrorKind::InvalidConfig, error)),
    };
    config.validate().map(|_| config)
}

pub fn init_interpreter(config: Config, directory: PathBuf) -> Result<Interpreter, Error> {
    if !directory.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidProjectDirectory,
            "expected project path to be a directory",
        ));
    }
    let text = match get_file(&directory, "text") {
        Some(text) => text,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidProjectDirectory,
                "missing 'text' file in project directory",
            ));
        }
    };
    let r#extern = get_file(&directory, "extern");
    let data = get_file(&directory, "data");
    let ktext = get_file(&directory, "ktext");
    let kdata = get_file(&directory, "kdata");
    Interpreter::init(&config, text, r#extern, data, ktext, kdata)
}

pub fn run(interpreter: &mut Interpreter) -> Result<Option<u8>, Error> {
    match interpreter.run() {
        Ok(()) => Ok(interpreter.exit_code),
        Err(exception) => {
            if interpreter.show_crash_handler {
                interpreter.print_crash_handler();
            }
            Err(Error::new(ErrorKind::MipsException, exception))
        }
    }
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
    Err(Error::new(
        ErrorKind::NotFound,
        "couldn't find `Seaside.toml`",
    ))
}

fn get_file(directory: &Path, name: &str) -> Option<PathBuf> {
    let path = directory.join(name);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}
