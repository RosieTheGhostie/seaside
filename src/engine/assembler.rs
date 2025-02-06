use super::{Error, ErrorKind};
use crate::{assembler::Assembler, Config};
use minimal_logging::macros::grayln;
use std::{fs::read_to_string, path::PathBuf, str::FromStr, time::Instant};

pub fn assemble(
    config: Config,
    source: PathBuf,
    output_directory: Option<PathBuf>,
) -> Result<(), Error> {
    let start_time = Instant::now();
    let output_directory = output_directory.unwrap_or_else(|| PathBuf::from_str(".").unwrap());
    let source_code = read_to_string(&source)
        .map_err(|_| Error::new(ErrorKind::ExternalFailure, "failed to read source file"))?;
    let mut assembler = Assembler::init(&config, &source_code);
    assembler.build().map_err(Error::from)?;
    assembler.export(&output_directory).map_err(Error::from)?;
    let elapsed = start_time.elapsed();
    grayln!("assembled {source:?} in {elapsed:#?}");
    Ok(())
}
