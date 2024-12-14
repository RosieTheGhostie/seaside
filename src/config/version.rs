use semver::Version;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(version: &Version, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let repr = version.to_string();
    serializer.serialize_str(&repr)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    let repr = String::deserialize(deserializer)?;
    let version = Version::parse(&repr).map_err(serde::de::Error::custom)?;
    Ok(version)
}
