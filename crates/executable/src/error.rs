use std::io;

use seaside_type_aliases::ServiceCode;
use thiserror::Error as ThisError;
use validator::ValidationErrors;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("{0}")]
    Invalid(#[from] ValidationErrors),

    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Serialize(#[from] rmp_serde::encode::Error),

    #[error("{0}")]
    Deserialize(#[from] rmp_serde::decode::Error),

    #[error("attempted to reuse service code {0}")]
    ServiceCodeInUse(ServiceCode),
}
