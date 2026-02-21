#[cfg(feature = "serde")]
mod deserialize;

use std::collections::{HashMap, hash_map};

use crate::{consts::services::Service, types::ServiceCode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "validator", derive(validator::Validate))]
pub struct Services {
    #[cfg_attr(feature = "serde", serde(flatten))]
    data: HashMap<ServiceCode, Service>,

    #[cfg_attr(feature = "serde", serde(skip))]
    #[cfg_attr(
        feature = "validator",
        validate(range(min = 1, message = "missing a service to exit program"))
    )]
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
