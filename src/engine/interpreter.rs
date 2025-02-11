//! Wraps the [`seaside_interpreter`] crate.
//!
//! Provides the wrapper functions [`init_interpreter`] and [`run`], which initialize and run the
//! interpreter, respectively.

use super::resolve_if_exists;
use anyhow::{Context, Error, Result};
use seaside_config::Config;
use seaside_error::EngineError;
use seaside_interpreter::Interpreter;
use std::{env::set_current_dir, path::PathBuf};

/// Initializes the interpreter in preparation for execution via the [`run`] function.
pub fn init_interpreter(
    config: Config,
    mut directory: PathBuf,
    argv: Vec<String>,
) -> Result<Interpreter> {
    if !directory.is_dir() {
        return Err(Error::new(EngineError::InvalidProjectDirectory))
            .with_context(|| "expected project path to be a directory");
    }
    if config.project_directory_is_cwd {
        directory = match set_current_dir(&directory) {
            Ok(()) => ".".parse()?,
            Err(_) => {
                return Err(Error::new(EngineError::ExternalFailure)).with_context(|| {
                    format!("failed to change the cwd to {}", directory.display())
                });
            }
        };
    }
    let text = resolve_if_exists(&directory, "text").ok_or_else(|| {
        Error::new(EngineError::InvalidProjectDirectory)
            .context("missing 'text' file in project directory")
    })?;
    let r#extern = resolve_if_exists(&directory, "extern");
    let data = resolve_if_exists(&directory, "data");
    let ktext = resolve_if_exists(&directory, "ktext");
    let kdata = resolve_if_exists(&directory, "kdata");
    Interpreter::init(&config, text, r#extern, data, ktext, kdata, argv)
}

/// Runs `interpreter`.
///
/// Upon success, this returns the exit code. If the interpreter raises an [`Exception`], this wraps
/// it in an [`Error`] and, if enabled in the config, prints the crash handler.
///
/// [`Exception`]: crate::interpreter::Exception
pub fn run(interpreter: &mut Interpreter) -> Result<Option<u8>> {
    match interpreter.run() {
        Ok(()) => Ok(interpreter.exit_code),
        Err(exception) => {
            if interpreter.show_crash_handler {
                interpreter.print_crash_handler();
            }
            Err(exception.into())
        }
    }
}
