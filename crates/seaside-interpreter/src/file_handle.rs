use std::{
    fs::File,
    io::{stderr, stdin, stdout, Error, ErrorKind, Read, Result, Stderr, Stdin, Stdout, Write},
};

pub enum FileHandle {
    Stdin(Stdin),
    Stdout(Stdout),
    Stderr(Stderr),
    File(File),
}

impl Read for FileHandle {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Self::Stdin(ref mut stdin) => stdin.read(buf),
            Self::File(ref mut file) => file.read(buf),
            _ => Err(Error::from(ErrorKind::PermissionDenied)),
        }
    }
}

impl Write for FileHandle {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            Self::Stdout(ref mut stdout) => stdout.write(buf),
            Self::Stderr(ref mut stderr) => stderr.write(buf),
            Self::File(ref mut file) => file.write(buf),
            _ => Err(Error::from(ErrorKind::PermissionDenied)),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            Self::Stdout(ref mut stdout) => stdout.flush(),
            Self::Stderr(ref mut stderr) => stderr.flush(),
            Self::File(ref mut file) => file.flush(),
            _ => Err(Error::from(ErrorKind::PermissionDenied)),
        }
    }
}

impl FileHandle {
    pub fn new_stdin() -> Self {
        Self::Stdin(stdin())
    }

    pub fn new_stdout() -> Self {
        Self::Stdout(stdout())
    }

    pub fn new_stderr() -> Self {
        Self::Stderr(stderr())
    }
}
