use core::ops::{Deref, Index, IndexMut};
use seaside_constants::register::{CpuRegister, FpuRegister};

/// The values of `N` 32-bit registers stored contiguously in memory.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Registers<const N: usize> {
    registers: [u32; N],
}

pub struct RegisterIter<'a, const N: usize> {
    registers: &'a Registers<N>,
    i: usize,
}

impl<const N: usize> Deref for Registers<N> {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.registers
    }
}

impl<const N: usize> Default for Registers<N> {
    fn default() -> Self {
        Self {
            registers: [0u32; N],
        }
    }
}

impl<const N: usize> From<[u32; N]> for Registers<N> {
    fn from(registers: [u32; N]) -> Self {
        Self { registers }
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

impl Index<CpuRegister> for Registers<32> {
    type Output = u32;

    fn index(&self, index: CpuRegister) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<CpuRegister> for Registers<32> {
    fn index_mut(&mut self, index: CpuRegister) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl Index<FpuRegister> for Registers<32> {
    type Output = u32;

    fn index(&self, index: FpuRegister) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<FpuRegister> for Registers<32> {
    fn index_mut(&mut self, index: FpuRegister) -> &mut Self::Output {
        &mut self.registers[index as usize]
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

impl<const N: usize> Registers<N> {
    /// Returns true if this object is in the default state.
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl<'a, const N: usize> Iterator for RegisterIter<'a, N> {
    type Item = &'a u32;

    fn next(&mut self) -> Option<Self::Item> {
        let register = self.registers.get(self.i)?;
        self.i += 1;
        Some(register)
    }
}
