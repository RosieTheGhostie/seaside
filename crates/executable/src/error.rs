use seaside_constants::StaticSegment;
use seaside_type_aliases::ServiceCode;
use thiserror::Error as ThisError;

use crate::tag::Tag;

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error("{0} is not a text segment")]
    NotATextSegment(StaticSegment),

    #[error("{0} is not a data segment")]
    NotADataSegment(StaticSegment),

    #[error("cannot plan existing section {:?}", String::from_utf8_lossy(.0))]
    SectionAlreadyExists(Tag),

    #[error("attempted to reuse service code {0}")]
    ServiceCodeInUse(ServiceCode),
}
