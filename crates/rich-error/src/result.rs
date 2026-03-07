use crate::{ErrorSeverity, RichError};

pub type RichResult<T> = Result<(T, Box<[RichError]>), Box<[RichError]>>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RichResultBuilder {
    errors: Vec<RichError>,
}

impl RichResultBuilder {
    pub const fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub const fn errors(&self) -> &[RichError] {
        self.errors.as_slice()
    }

    pub fn take_errors(self) -> Box<[RichError]> {
        self.errors.into_boxed_slice()
    }

    pub fn add_error(&mut self, error: RichError) {
        self.errors.push(error);
    }

    pub fn bail<T>(&mut self, error: RichError) -> Result<T, Bailed> {
        self.add_error(error);
        Err(Bailed)
    }

    pub fn pop_latest(&mut self) -> Option<RichError> {
        self.errors.pop()
    }

    pub fn pop_latest_if<F>(&mut self, predicate: F) -> Option<RichError>
    where
        F: FnOnce(&mut RichError) -> bool,
    {
        self.errors.pop_if(predicate)
    }

    pub fn finish<T>(self, ok: T) -> RichResult<T> {
        self.finish_with(|| ok)
    }

    pub fn finish_with<T>(self, if_ok: impl FnOnce() -> T) -> RichResult<T> {
        let errors = self.take_errors();
        let is_ok = !errors
            .iter()
            .any(|error| matches!(error.severity, ErrorSeverity::Error));

        if is_ok {
            Ok((if_ok(), errors))
        } else {
            Err(errors)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[error("bailed")]
pub struct Bailed;

#[macro_export]
macro_rules! map_err {
    ($builder:expr, $err:expr $(,)?) => {
        |_| {
            $builder.add_error($err);
            $crate::result::Bailed
        }
    };
    ($err:ident -> $builder:expr) => {
        |$err| {
            $builder.add_error($err);
            $crate::result::Bailed
        }
    };
}
