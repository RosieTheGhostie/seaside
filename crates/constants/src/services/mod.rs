pub mod mars;
pub mod service;
pub mod spim;

pub use service::Service;

#[cfg(feature = "serde")]
mod deserialize;

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

#[cfg(feature = "seaside-int-utils")]
impl seaside_int_utils::AllZeroes for Services {
    fn all_zeroes() -> Self {
        Self {
            ..Default::default()
        }
    }
}
