use core::fmt::{self, Formatter};

use serde::{Deserialize, Deserializer, de};

use super::{Service, Services};
use crate::ServiceCode;

impl<'de> Deserialize<'de> for Services {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ServicesVisitor)
    }
}

pub struct ServicesVisitor;

impl<'de> de::Visitor<'de> for ServicesVisitor {
    type Value = Services;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a mapping from service codes to services")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut services = Services::with_capacity(access.size_hint().unwrap_or(0));
        while let Some((ServiceCodeWrapper(code), service)) = access.next_entry::<_, Service>()? {
            services.insert(code, service);
        }

        Ok(services)
    }
}

struct ServiceCodeWrapper(ServiceCode);

impl<'de> Deserialize<'de> for ServiceCodeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ServiceCodeVisitor).map(Self)
    }
}

struct ServiceCodeVisitor;

impl<'de> de::Visitor<'de> for ServiceCodeVisitor {
    type Value = ServiceCode;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a service code or its string representation")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.try_into().map_err(de::Error::custom)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.try_into().map_err(de::Error::custom)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.try_into().map_err(de::Error::custom)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.try_into().map_err(de::Error::custom)
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.try_into().map_err(de::Error::custom)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v as _)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v as _)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v as _)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.try_into().map_err(de::Error::custom)
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.try_into().map_err(de::Error::custom)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse().map_err(de::Error::custom)
    }
}
