use semver::Version;
use std::cmp::Ordering;

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
