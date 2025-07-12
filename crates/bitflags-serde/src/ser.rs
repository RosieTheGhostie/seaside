#[macro_export]
macro_rules! impl_serialize {
    ($flag:ty) => {
        impl ::serde::Serialize for $flag {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use ::serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(::core::option::Option::None)?;
                for (name, flag) in Self::all().iter_names() {
                    map.serialize_entry(
                        &format!("{}", ::heck::AsSnakeCase(name)),
                        &self.intersects(flag),
                    )?;
                }
                map.end()
            }
        }
    };
}
