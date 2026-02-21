//! Wraps the [`seaside_interpreter`] crate.
//!
//! Provides the wrapper functions [`init_interpreter`] and [`run`], which initialize and run the
//! interpreter, respectively.

use std::path::Path;

use anyhow::{Context, Error, Result};
use seaside_config::Config;
use seaside_core::EngineError;
use seaside_executable::Executable;
use seaside_interpreter::Interpreter;

/// Initializes the interpreter in preparation for execution via the [`run`] function.
pub fn init_interpreter<P>(
    config: Config,
    executable_path: P,
    argv: Vec<String>,
) -> Result<Interpreter>
where
    P: AsRef<Path>,
{
    let executable_path = executable_path.as_ref();
    if config.executable_parent_is_cwd
        && let Some(executable_parent) = executable_path.parent()
        && !executable_parent.as_os_str().is_empty()
    {
        std::env::set_current_dir(executable_parent)
            .map_err(|_| Error::new(EngineError::ExternalFailure))
            .with_context(|| {
                format!(
                    "failed to change the cwd to {}",
                    executable_parent.display()
                )
            })?;
    }

    let executable_file = std::fs::File::open(executable_path)?;
    let executable = Executable::from_reader(executable_file)?;
    Interpreter::init(&config, executable, argv)
}

/// Runs `interpreter`.
///
/// Upon success, this returns the exit code. If the interpreter raises an
/// [`Exception`](seaside_interpreter::Exception), this wraps it in an [`Error`] and, if enabled in
/// the config, prints the crash handler.
pub fn run(interpreter: &mut Interpreter) -> Result<Option<u8>> {
    match interpreter.run() {
        Ok(()) => Ok(interpreter.state.exit_code),
        Err(exception) => {
            if interpreter.show_crash_handler {
                interpreter.state.print_crash_handler();
            }

            Err(exception.into())
        }
    }
}
