use core::fmt::{self, Display, Formatter};

pub type ErrorGroupPrefix = [u8; 3];

pub trait ErrorGroup {
    const PREFIX: ErrorGroupPrefix;
    const _UTF8_PREFIX: &str = match str::from_utf8(&Self::PREFIX) {
        Ok(prefix) => prefix,
        Err(_) => panic!("ErrorGroup::PREFIX should be valid UTF-8"),
    };

    fn code(&self) -> ErrorCode;
}

impl ErrorGroup for std::io::Error {
    const PREFIX: ErrorGroupPrefix = *b"_IO";
    fn code(&self) -> ErrorCode {
        ErrorCode::new_for::<Self>(0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ErrorCode {
    pub prefix: ErrorGroupPrefix,
    pub number: u8,
}

impl ErrorCode {
    pub const fn new_for<G>(number: u8) -> Self
    where
        G: ErrorGroup,
    {
        Self {
            prefix: G::PREFIX,
            number,
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{:03}",
            String::from_utf8_lossy(&self.prefix),
            self.number,
        )
    }
}
