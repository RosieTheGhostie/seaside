use super::{get_file, Error, ErrorKind};
use crate::{interpreter::Interpreter, Config};
use std::{env::set_current_dir, path::PathBuf};

pub fn init_interpreter(
    config: Config,
    mut directory: PathBuf,
    argv: Vec<String>,
) -> Result<Interpreter, Error> {
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
    let text = get_file(&directory, "text").ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidProjectDirectory,
            "missing 'text' file in project directory",
        )
    })?;
    let r#extern = get_file(&directory, "extern");
    let data = get_file(&directory, "data");
    let ktext = get_file(&directory, "ktext");
    let kdata = get_file(&directory, "kdata");
    Interpreter::init(&config, text, r#extern, data, ktext, kdata, argv)
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
