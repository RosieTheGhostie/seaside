pub mod assembler;
pub mod config;
pub mod disassembler;
pub mod error;
pub mod interpreter;

pub use assembler::assemble;
pub use config::get_config;
pub use disassembler::{disassemble, disassemble_segment};
pub use error::{Error, ErrorKind};
pub use interpreter::{init_interpreter, run};

use std::path::{Path, PathBuf};

fn get_file<P>(directory: &Path, name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let path = directory.join(name);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}
