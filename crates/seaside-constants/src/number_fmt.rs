use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum NumberFormat {
    SingleNoPrefix = 0,
    DoubleNoPrefix = 1,
    WordNoPrefix = 4,
    // LongNoPrefix = 5,
    Single = 16,
    Double = 17,
    Word = 20,
    // Long = 21,
}

impl Display for NumberFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use NumberFormat::*;
        f.write_str(match *self {
            Single | SingleNoPrefix => ".s",
            Double | DoubleNoPrefix => ".d",
            Word | WordNoPrefix => ".w",
        })
    }
}
