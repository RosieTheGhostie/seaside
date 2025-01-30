pub mod error;

pub use error::{Error, ErrorKind};

use crate::{
    assembler::Assembler,
    byte_stream::ByteStream,
    cmd_args::CmdArgs,
    config::{Config, Validate},
    interpreter::Interpreter,
    type_aliases::{Address, Instruction},
};
use minimal_logging::macros::grayln;
use std::{
    env::{current_exe, set_current_dir},
    fs::read_to_string,
    path::{Path, PathBuf},
    str::FromStr,
    time::Instant,
};

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

pub fn disassemble(instruction: Instruction, address: Option<Address>) -> Result<(), Error> {
    match crate::disassembler::disassemble_advanced(
        instruction,
        address.unwrap_or_default(),
        address.is_some(),
    ) {
        Some(disassembly) => {
            println!("{disassembly}");
            Ok(())
        }
        None => Err(Error::from(ErrorKind::MalformedMachineCode)),
    }
}

pub fn disassemble_segment(
    config: Config,
    segment: PathBuf,
    start_address: Option<Address>,
) -> Result<(), Error> {
    let mut address = if let Some(address) = start_address {
        address
    } else if segment.ends_with("text") {
        config.memory_map.segments.text.address_range.base
    } else if segment.ends_with("ktext") {
        config.memory_map.segments.ktext.address_range.base
    } else {
        0
    };
    let bytes = std::fs::read(segment)
        .map_err(|_| Error::new(ErrorKind::NotFound, "couldn't find that segment"))?;
    for instruction in ByteStream::<'_, u32>::new(&bytes, config.endian) {
        disassemble(instruction, Some(address))?;
        address += 4;
    }
    Ok(())
}

fn find_seaside_toml() -> Result<PathBuf, Error> {
    let path = PathBuf::from("Seaside.toml");
    if path.exists() {
        return Ok(path);
    }
    match current_exe()
        .map_err(|_| Error::new(ErrorKind::ExternalFailure, "'std::env::current_exe' failed"))?
        .ancestors()
        .nth(3)
    {
        Some(seaside_root) => {
            let path = seaside_root.join(path);
            if path.exists() {
                Ok(path)
            } else {
                Err(Error::new(
                    ErrorKind::NotFound,
                    "couldn't find 'Seaside.toml'",
                ))
            }
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "couldn't find seaside's root directory",
        )),
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
