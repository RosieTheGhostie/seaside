use num_derive::FromPrimitive;

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
