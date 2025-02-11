use semver::Version;
use serde::{Deserialize, Deserializer, Serializer};
use std::cmp::Ordering;

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

pub enum VersionComparison {
    Compatible,
    AIsAheadOfB,
    BIsAheadOfA,
}

pub fn major_and_minor(version: &Version) -> (u64, u64) {
    (version.major, version.minor)
}

impl VersionComparison {
    pub fn compare(a: &Version, b: &Version) -> Self {
        let a_major_minor = major_and_minor(a);
        let b_major_minor = major_and_minor(b);
        match a_major_minor.cmp(&b_major_minor) {
            Ordering::Equal => Self::Compatible,
            Ordering::Less => Self::BIsAheadOfA,
            Ordering::Greater => Self::AIsAheadOfB,
        }
    }
}
