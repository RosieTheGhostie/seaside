use super::{
    mars::Mars,
    spim::{self, Spim},
};
use seaside_service_group::NestedServiceGroup;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, NestedServiceGroup, PartialEq, Serialize)]
#[serde(into = "String")]
pub enum Service {
    Spim(Spim),
    Mars(Mars),
}

// This is needed to derive the `Serialize` trait for some reason.
impl From<Service> for String {
    fn from(value: Service) -> Self {
        value.to_string()
    }
}

impl<'de> Deserialize<'de> for Service {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl Service {
    pub const fn is_exit(&self) -> bool {
        matches!(
            self,
            Self::Spim(Spim::System(spim::System::Exit | spim::System::Exit2))
        )
    }
}
