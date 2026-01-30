use seaside_type_aliases::ServiceCode;
use thiserror::Error as ThisError;
use validator::ValidationErrors;

#[derive(Clone, Debug, PartialEq, ThisError)]
pub enum Error {
    #[error("{0}")]
    Invalid(#[from] ValidationErrors),

    #[error("attempted to reuse service code {0}")]
    ServiceCodeInUse(ServiceCode),
}
