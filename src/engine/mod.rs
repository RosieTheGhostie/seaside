pub mod error;

pub use error::{Error, ErrorKind};

use crate::{
    cmd_args::CmdArgs,
    config::{Config, Validate},
    interpreter::Interpreter,
};
use std::{
    env::set_current_dir,
    fs::read_to_string,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn get_config(args: &CmdArgs) -> Result<Config, Error> {
    let config_path: &PathBuf;
    let stupid_binding: PathBuf;
    if let Some(path) = &args.config {
        config_path = path;
    } else {
        stupid_binding = find_seaside_toml()?;
        config_path = &stupid_binding;
    }
    let file_contents = read_to_string(config_path)
        .map_err(|_| Error::new(ErrorKind::ExternalFailure, "failed to read config file"))?;
    let config: Config = toml::from_str(&file_contents)
        .map_err(|error| Error::new(ErrorKind::InvalidConfig, error))?;
    config.validate().map(|_| config)
}

pub fn init_interpreter(config: Config, mut directory: PathBuf) -> Result<Interpreter, Error> {
    if !directory.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidProjectDirectory,
            "expected project path to be a directory",
        ));
    }
    if config.project_directory_is_cwd {
        directory = match set_current_dir(&directory) {
            Ok(()) => ".".parse().unwrap(),
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::ExternalFailure,
                    format!("failed to change the cwd to {}", directory.display()),
                ));
            }
        };
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
