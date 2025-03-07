use anyhow::{anyhow, Error};
use num_derive::{FromPrimitive as FromPrimitiveMacro, ToPrimitive as ToPrimitiveMacro};
use num_traits::FromPrimitive;
use semver::Version;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, AddAssign, Sub, SubAssign},
};
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    EnumCountMacro,
    EnumIter,
    Eq,
    FromPrimitiveMacro,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    ToPrimitiveMacro,
)]
pub enum SeasideVersion {
    #[default]
    V1_2_0 = 5,
    V1_1_0 = 4,
    V1_0_3 = 3,
    V1_0_2 = 2,
    V1_0_1 = 1,
    V1_0_0 = 0,
}

impl From<SeasideVersion> for Version {
    fn from(value: SeasideVersion) -> Self {
        use SeasideVersion::*;
        match value {
            V1_2_0 => Version::new(1, 2, 0),
            V1_1_0 => Version::new(1, 1, 0),
            V1_0_3 => Version::new(1, 0, 3),
            V1_0_2 => Version::new(1, 0, 2),
            V1_0_1 => Version::new(1, 0, 1),
            V1_0_0 => Version::new(1, 0, 0),
        }
    }
}

impl Display for SeasideVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", Version::from(*self))
    }
}

impl Add<isize> for SeasideVersion {
    type Output = SeasideVersion;

    fn add(self, rhs: isize) -> Self::Output {
        Self::from_isize((self as isize + rhs) % Self::COUNT as isize).unwrap()
    }
}

impl AddAssign<isize> for SeasideVersion {
    fn add_assign(&mut self, rhs: isize) {
        *self = *self + rhs;
    }
}

impl Sub<isize> for SeasideVersion {
    type Output = SeasideVersion;

    fn sub(self, rhs: isize) -> Self::Output {
        Self::from_isize((self as isize - rhs) % Self::COUNT as isize).unwrap()
    }
}

impl SubAssign<isize> for SeasideVersion {
    fn sub_assign(&mut self, rhs: isize) {
        *self = *self - rhs;
    }
}

impl TryFrom<Version> for SeasideVersion {
    type Error = Error;

    fn try_from(value: Version) -> Result<Self, Self::Error> {
        use SeasideVersion::*;

        Ok(match (value.major, value.minor, value.patch) {
            (1, 2, 0) => V1_2_0,
            (1, 1, 0) => V1_1_0,
            (1, 0, 3) => V1_0_3,
            (1, 0, 2) => V1_0_2,
            (1, 0, 1) => V1_0_1,
            (1, 0, 0) => V1_0_0,
            _ => return Err(anyhow!("{value} is not a known version of seaside")),
        })
    }
}
