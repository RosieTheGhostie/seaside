use std::{
    fs::File,
    io::{self, Stderr, Stdin, Stdout, prelude::*},
};

pub enum FileHandle {
    Stdin(Stdin),
    Stdout(Stdout),
    Stderr(Stderr),
    File(File),
}

impl Read for FileHandle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Stdin(stdin) => stdin.read(buf),
            Self::File(file) => file.read(buf),
            _ => Err(io::Error::from(io::ErrorKind::PermissionDenied)),
        }
    }
}

impl Write for FileHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Stdout(stdout) => stdout.write(buf),
            Self::Stderr(stderr) => stderr.write(buf),
            Self::File(file) => file.write(buf),
            _ => Err(io::Error::from(io::ErrorKind::PermissionDenied)),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Stdout(stdout) => stdout.flush(),
            Self::Stderr(stderr) => stderr.flush(),
            Self::File(file) => file.flush(),
            _ => Err(io::Error::from(io::ErrorKind::PermissionDenied)),
        }
    }
}

impl FileHandle {
    pub fn new_stdin() -> Self {
        Self::Stdin(io::stdin())
    }

    pub fn new_stdout() -> Self {
        Self::Stdout(io::stdout())
    }

    pub fn new_stderr() -> Self {
        Self::Stderr(io::stderr())
    }
}
