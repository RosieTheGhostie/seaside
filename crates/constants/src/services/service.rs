use seaside_service_group::NestedServiceGroup;

use super::{
    mars::Mars,
    spim::{self, Spim},
};

#[derive(Clone, Copy, Debug, Eq, NestedServiceGroup, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "String"))]
pub enum Service {
    Spim(Spim),
    Mars(Mars),
}

impl Service {
    pub const fn is_exit(&self) -> bool {
        matches!(
            self,
            Self::Spim(Spim::System(spim::System::Exit | spim::System::Exit2))
        )
    }
}

// This is needed to derive the `Serialize` trait for some reason.
#[cfg(feature = "serde")]
impl From<Service> for String {
    fn from(service: Service) -> Self {
        service.to_string()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Service {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
