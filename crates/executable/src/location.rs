use seaside_core::types::{Size, UnsignedOffset};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Location {
    pub offset: UnsignedOffset,
    pub size: Size,
}

impl Location {
    pub const NONE: Self = Self::new(0, 0);

    pub const fn new(offset: UnsignedOffset, size: Size) -> Self {
        Self { offset, size }
    }

    /// Creates a new [`Location`] that doesn't have a meaningful [`offset`](Self::offset).
    ///
    /// This makes it possible to construct a skeleton of the executable without knowing all the
    /// offsets.
    pub const fn plan(size: Size) -> Self {
        Self { offset: 0, size }
    }
}
