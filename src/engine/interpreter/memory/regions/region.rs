use super::super::super::exception::Exception;
use crate::config::memory_map::Address;

pub trait Region {
    fn contains(&self, address: Address) -> bool;

    #[allow(unused_variables)]
    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        Err(Exception::InvalidLoad)
    }

    #[allow(unused_variables)]
    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        Err(Exception::InvalidLoad)
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception>;

    #[allow(unused_variables)]
    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        Err(Exception::InvalidStore)
    }

    #[allow(unused_variables)]
    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        Err(Exception::InvalidStore)
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception>;
}
