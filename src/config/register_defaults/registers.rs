use std::ops::{Index, IndexMut};

/// The values of `N` 32-bit registers stored contiguously in memory.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Registers<const N: usize> {
    registers: [u32; N],
}

impl<const N: usize> Default for Registers<N> {
    fn default() -> Self {
        Self {
            registers: [0u32; N],
        }
    }
}

impl<const N: usize> Registers<N> {
    /// Returns true if this object is in the default state.
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl<const N: usize> Index<usize> for Registers<N> {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.registers[index]
    }
}

impl<const N: usize> IndexMut<usize> for Registers<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.registers[index]
    }
}

impl<const N: usize> IntoIterator for Registers<N> {
    type IntoIter = <[u32; N] as IntoIterator>::IntoIter;
    type Item = <[u32; N] as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.registers.into_iter()
    }
}

impl<const N: usize> IntoIterator for &Registers<N> {
    type IntoIter = <[u32; N] as IntoIterator>::IntoIter;
    type Item = <[u32; N] as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.registers.into_iter()
    }
}
