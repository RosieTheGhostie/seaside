#![allow(dead_code)]
pub mod error;
pub mod interpreter;

pub use error::{Error, ErrorKind};
pub use interpreter::Interpreter;

use crate::config::Config;
use std::path::{Path, PathBuf};

pub fn init(config: Config, directory: PathBuf) -> Result<Interpreter, Error> {
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

fn get_file(directory: &Path, name: &str) -> Option<PathBuf> {
    let path = directory.join(name);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}
