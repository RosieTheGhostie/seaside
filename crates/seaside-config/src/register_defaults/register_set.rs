use num_traits::{FromPrimitive, ToPrimitive};
use std::str::FromStr;

/// A set of related registers.
pub trait RegisterSet: FromStr + FromPrimitive + ToPrimitive {
    /// The number of registers in this set.
    const NUM_REGISTERS: usize;
    /// The names of each register. Should be in the same order as the register array.
    const REGISTER_NAMES: &'static [&'static str];
}

/// Implements [`serde`] serialization and deserialization for a type implementing [`RegisterSet`].
macro_rules! make_registers_format {
    ($register_t:ty) => {
        pub mod registers_format {
            use super::*;
            use serde::{
                de::{Deserializer, MapAccess, Visitor},
                ser::SerializeMap,
                Serializer,
            };
            use std::{
                fmt::{Formatter, Result as FmtResult},
                iter::zip,
            };
            use $crate::register_defaults::Registers;

            pub struct RegisterSetVisitor;

            impl<'de> Visitor<'de> for RegisterSetVisitor {
                type Value = Registers<{ <$register_t>::NUM_REGISTERS }>;

                fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                    formatter.write_str("a set of register defaults")
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    let mut register_set = <Self::Value>::default();
                    while let Some((key, value)) = access.next_entry::<String, u32>()? {
                        if let Ok(register) = <$register_t>::from_str(&key) {
                            register_set[register.to_usize().unwrap()] = value;
                        }
                        // NOTE: We can't raise an error if the key was invalid because M::Error
                        //       is too generic to provide any semblance of a constructor.
                    }
                    Ok(register_set)
                }
            }

            pub fn serialize<S>(
                register_set: &Registers<{ <$register_t>::NUM_REGISTERS }>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut map = serializer.serialize_map(None)?;
                for (name, value) in zip(<$register_t>::REGISTER_NAMES, register_set)
                    .filter(|(_, value)| *value != 0)
                {
                    map.serialize_entry(name, &value)?;
                }
                map.end()
            }

            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> Result<Registers<{ <$register_t>::NUM_REGISTERS }>, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_map(RegisterSetVisitor)
            }
        }
    };
}
pub(super) use make_registers_format;
