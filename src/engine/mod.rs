#![allow(dead_code)]
pub mod error;
pub mod init;
pub mod interpreter;

pub use error::{Error, ErrorKind};
pub use interpreter::Interpreter;
