use anyhow::{Context, Error, Result};
use seaside_constants::services::Services;
use seaside_error::EngineError;

use crate::Validate;

impl Validate for Services {
    fn validate(&self) -> Result<()> {
        if self.n_exits() > 0 {
            Ok(())
        } else {
            Err(Error::new(EngineError::InvalidConfig)).context("missing a service to exit program")
        }
    }
}
