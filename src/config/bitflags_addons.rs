use convert_case::{Case, Casing};
use serde::de::{self, Visitor};
use std::marker::PhantomData;

pub struct BitFlagVisitor<F: bitflags::Flags> {
    marker: PhantomData<fn() -> F>,
}

impl<F: bitflags::Flags> BitFlagVisitor<F> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, F> Visitor<'de> for BitFlagVisitor<F>
where
    F: bitflags::Flags,
{
    type Value = F;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a set of bit flags (represented as a map from strings to bools)")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut flags = F::empty();
        while let Some((key, true)) = access.next_entry::<String, bool>()? {
            let key = key.to_case(Case::Pascal); // Assume flags are supposed to be in PascalCase.
            if let Some(flag) = F::from_name(&key) {
                flags.insert(flag);
            }
            // NOTE: We can't raise an error if the key was invalid because M::Error doesn't provide
            //       any methods to construct it.
        }
        Ok(flags)
    }
}

macro_rules! impl_serialize {
    ($flag:ty) => {
        impl serde::Serialize for $flag {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use convert_case::{Case::Snake, Casing};
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(None)?;
                for (name, flag) in Self::all().iter_names() {
                    map.serialize_entry(&name.to_case(Snake), &self.intersects(flag))?;
                }
                map.end()
            }
        }
    };
}

macro_rules! impl_deserialize {
    ($flag:ty) => {
        impl<'de> serde::Deserialize<'de> for $flag {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                use crate::config::bitflags_addons::BitFlagVisitor;
                deserializer.deserialize_map(BitFlagVisitor::new())
            }
        }
    };
}

pub(super) use impl_deserialize;
pub(super) use impl_serialize;
