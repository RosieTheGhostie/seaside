use num_traits::{FromPrimitive, ToPrimitive};
use std::str::FromStr;

pub trait RegisterSet: FromStr + FromPrimitive + ToPrimitive {
    const NUM_REGISTERS: usize;
    const REGISTER_NAMES: &'static [&'static str];
}

macro_rules! make_register_set_visitor {
    ($register_t:ty, $visitor_name:ident) => {
        #[derive(Default)]
        pub struct $visitor_name;

        impl<'de> serde::de::Visitor<'de> for $visitor_name {
            type Value = crate::config::registers::Registers<{ <$register_t>::NUM_REGISTERS }>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a set of register defaults")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
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
    };
}
pub(super) use make_register_set_visitor;

macro_rules! make_register_set_serialize_fn {
    ($register_t:ty, $fn_name:ident) => {
        pub fn $fn_name<S>(
            register_set: &crate::config::registers::Registers<{ <$register_t>::NUM_REGISTERS }>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeMap;

            let mut map = serializer.serialize_map(None)?;
            for (name, value) in std::iter::zip(<$register_t>::REGISTER_NAMES, register_set)
                .filter(|(_, value)| *value != 0)
            {
                map.serialize_entry(name, &value)?;
            }
            map.end()
        }
    };
}
pub(super) use make_register_set_serialize_fn;

macro_rules! make_register_set_deserialize_fn {
    ($register_t:ty, $set_visitor:ty, $fn_name:ident) => {
        pub fn $fn_name<'de, D>(
            deserializer: D,
        ) -> Result<crate::config::registers::Registers<{ <$register_t>::NUM_REGISTERS }>, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            // If this weren't a macro, the call to `default` would be redundant; however, the
            // compiler throws a hissy fit if you leave it out because it's expecting an expression.
            deserializer.deserialize_map(<$set_visitor>::default())
        }
    };
}
pub(super) use make_register_set_deserialize_fn;
