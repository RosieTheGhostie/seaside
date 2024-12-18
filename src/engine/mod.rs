#![allow(dead_code)]
pub mod error;
pub mod init;
pub mod interpreter;

pub use error::{Error, ErrorKind};
pub use interpreter::Interpreter;

pub fn run(mut interpreter: Interpreter) -> Result<Option<u8>, Error> {
    match interpreter.run() {
        Ok(()) => Ok(interpreter.exit_code),
        Err(exception) => Err(Error::new(ErrorKind::MipsException, exception)),
    }
}
