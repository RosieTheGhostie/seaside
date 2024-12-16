use crate::engine::Error;

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}
