use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{
    io::{Error, ErrorKind},
    marker::PhantomData,
    str::FromStr,
};

pub trait HasPresets: Sized {
    type Presets: FromStr;

    fn get_preset(preset: Self::Presets) -> Self;
}

pub trait HasBasicPresets: HasPresets<Presets = BasicPresets> {
    fn everything() -> Self {
        Self::get_preset(BasicPresets::Everything)
    }

    fn nothing() -> Self {
        Self::get_preset(BasicPresets::Nothing)
    }

    fn recommended() -> Self {
        Self::get_preset(BasicPresets::Recommended)
    }
}

pub enum BasicPresets {
    Everything,
    Nothing,
    Recommended,
}

impl FromStr for BasicPresets {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "everything" | "all" | "full" => Ok(Self::Everything),
            "nothing" | "none" | "empty" => Ok(Self::Nothing),
            "recommended" | "default" => Ok(Self::Recommended),
            _ => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }
}

pub fn maybe_using_preset<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + HasPresets,
    D: Deserializer<'de>,
{
    struct MaybeUsingPreset<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for MaybeUsingPreset<T>
    where
        T: Deserialize<'de> + HasPresets,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("the name of a preset or an explicit mapping from flags to bools")
        }

        fn visit_str<E>(self, v: &str) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            if let Ok(preset) = <T as HasPresets>::Presets::from_str(v) {
                Ok(T::get_preset(preset))
            } else {
                Err(serde::de::Error::custom("not a valid preset"))
            }
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(MaybeUsingPreset(PhantomData))
}

macro_rules! impl_bitflags_has_basic_presets {
    ($flag:ty, $recommended:expr) => {
        impl crate::config::presets::HasPresets for $flag {
            type Presets = crate::config::presets::BasicPresets;

            fn get_preset(preset: Self::Presets) -> Self {
                match preset {
                    Self::Presets::Everything => Self::all(),
                    Self::Presets::Nothing => Self::empty(),
                    Self::Presets::Recommended => $recommended,
                }
            }
        }
        impl crate::config::presets::HasBasicPresets for $flag {}
    };
}

pub(super) use impl_bitflags_has_basic_presets;
