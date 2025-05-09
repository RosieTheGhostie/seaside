pub mod mars;
pub mod service;
pub mod spim;

pub use service::Service;

use crate::Validate;
use anyhow::{Error, Result};
use seaside_error::EngineError;
use seaside_int_utils::AllZeroes;
use serde::{
    Deserialize, Serialize,
    de::{MapAccess, Visitor},
};
use std::{
    collections::{HashMap, hash_map::Iter as HashMapIter},
    fmt::{Formatter, Result as FmtResult},
    result::Result as StdResult,
};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Services {
    #[serde(flatten)]
    data: HashMap<u32, Service>,
    #[serde(skip)]
    n_exits: usize,
}

impl<'de> Deserialize<'de> for Services {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ServicesVisitor)
    }
}

impl Validate for Services {
    fn validate(&self) -> Result<()> {
        if self.n_exits > 0 {
            Ok(())
        } else {
            Err(Error::new(EngineError::InvalidConfig).context("missing a service to exit program"))
        }
    }
}

impl AllZeroes for Services {
    fn all_zeroes() -> Self {
        Self {
            ..Default::default()
        }
    }
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

    pub fn insert(&mut self, code: u32, service: Service) -> Option<Service> {
        if service.is_exit() {
            self.n_exits += 1;
        }
        self.data.insert(code, service)
    }

    pub fn remove(&mut self, code: u32) -> Option<Service> {
        self.data.remove(&code).inspect(|service| {
            if service.is_exit() {
                self.n_exits -= 1;
            }
        })
    }

    pub fn iter(&self) -> HashMapIter<'_, u32, Service> {
        self.data.iter()
    }
}

struct ServicesVisitor;

impl<'de> Visitor<'de> for ServicesVisitor {
    type Value = Services;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a mapping from service codes to services")
    }

    fn visit_map<A>(self, mut access: A) -> StdResult<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut services = Services::with_capacity(access.size_hint().unwrap_or(0));
        while let Some((code, service)) = access.next_entry::<String, Service>()? {
            let code = code.parse::<u32>().map_err(serde::de::Error::custom)?;
            services.insert(code, service);
        }
        Ok(services)
    }
}
