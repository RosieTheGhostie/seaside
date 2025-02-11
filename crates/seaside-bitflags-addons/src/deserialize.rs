#[macro_export]
macro_rules! impl_deserialize {
    ($flag:ty) => {
        impl<'de> serde::Deserialize<'de> for $flag {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_map($crate::BitFlagVisitor::new())
            }
        }
    };
}
