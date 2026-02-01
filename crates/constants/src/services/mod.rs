pub mod mars;
pub mod service;
pub mod spim;

pub use service::Service;

use std::collections::{HashMap, hash_map};

use seaside_type_aliases::ServiceCode;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Services {
    #[cfg_attr(feature = "serde", serde(flatten))]
    data: HashMap<ServiceCode, Service>,

    #[cfg_attr(feature = "serde", serde(skip))]
    n_exits: usize,
}

impl Services {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            n_exits: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
            n_exits: 0,
        }
    }

    pub const fn n_exits(&self) -> usize {
        self.n_exits
    }

    pub fn insert(&mut self, code: ServiceCode, service: Service) -> Option<Service> {
        self.n_exits += service.is_exit() as usize;
        self.data.insert(code, service)
    }

    pub fn remove(&mut self, code: ServiceCode) -> Option<Service> {
        self.data.remove(&code).inspect(|service| {
            self.n_exits -= service.is_exit() as usize;
        })
    }

    pub fn iter(&self) -> hash_map::Iter<'_, ServiceCode, Service> {
        self.data.iter()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Services {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use core::fmt::{self, Formatter};

        use seaside_type_aliases::ServiceCode;
        use serde::de::{Error, MapAccess, Visitor};

        pub struct ServicesVisitor;

        impl<'de> Visitor<'de> for ServicesVisitor {
            type Value = Services;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a mapping from service codes to services")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut services = Services::with_capacity(access.size_hint().unwrap_or(0));
                while let Some((code, service)) = access.next_entry::<String, Service>()? {
                    let code: ServiceCode = code.parse().map_err(Error::custom)?;
                    services.insert(code, service);
                }

                Ok(services)
            }
        }

        deserializer.deserialize_map(ServicesVisitor)
    }
}

#[cfg(feature = "seaside-int-utils")]
impl seaside_int_utils::AllZeroes for Services {
    fn all_zeroes() -> Self {
        Self {
            ..Default::default()
        }
    }
}
