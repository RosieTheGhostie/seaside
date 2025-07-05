use core::marker::PhantomData;
use heck::AsPascalCase;
use serde::de::{self, Visitor};

pub struct BitFlagVisitor<F: bitflags::Flags> {
    _marker: PhantomData<fn() -> F>,
}

impl<F: bitflags::Flags> BitFlagVisitor<F> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
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
            // Assume flags are supposed to be in PascalCase.
            let key = format!("{}", AsPascalCase(key));
            if let Some(flag) = F::from_name(&key) {
                flags.insert(flag);
            } else {
                return Err(serde::de::Error::custom("unknown flag"));
            }
        }
        Ok(flags)
    }
}

#[macro_export]
macro_rules! impl_deserialize {
    ($flag:ty) => {
        impl<'de> ::serde::Deserialize<'de> for $flag {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_map(::seaside_bitflags_serde::de::BitFlagVisitor::new())
            }
        }
    };
}
