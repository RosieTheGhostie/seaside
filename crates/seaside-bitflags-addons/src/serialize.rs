use super::BitFlagVisitor;
use bitflags::Flags;
use convert_case::{Case, Casing};
use serde::de::{self, Visitor};
use std::fmt::{Formatter, Result as FmtResult};

impl<'de, F> Visitor<'de> for BitFlagVisitor<F>
where
    F: Flags,
{
    type Value = F;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
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

#[macro_export]
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
