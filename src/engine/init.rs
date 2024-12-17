use super::{Error, Interpreter};
use crate::config::Config;
use std::path::PathBuf;

pub fn init(
    config: Config,
    text: PathBuf,
    r#extern: Option<PathBuf>,
    data: Option<PathBuf>,
    ktext: Option<PathBuf>,
    kdata: Option<PathBuf>,
) -> Result<Interpreter, Error> {
    Interpreter::init(&config, text, r#extern, data, ktext, kdata)
}
