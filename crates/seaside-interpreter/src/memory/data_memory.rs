use super::{
    super::Exception,
    regions::{DataRegion, Region},
};
use seaside_type_aliases::Address;

pub struct DataMemory {
    r#extern: DataRegion,
    data: DataRegion,
    heap: DataRegion,
    stack: DataRegion,
    kdata: DataRegion,
    mmio: DataRegion,
    pub next_heap_address: Address,
    pub free_heap_space: u32,
}

impl Region for DataMemory {
    fn contains(&self, address: Address) -> bool {
        self.region_containing(address).is_some()
    }

    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        match self.region_containing(address) {
            Some(region) => region.read_u8(address),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        match self.region_containing(address) {
            Some(region) => region.read_u16(address, assert_aligned),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        match self.region_containing(address) {
            Some(region) => region.read_u32(address, assert_aligned),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u64(&self, address: Address, assert_aligned: bool) -> Result<u64, Exception> {
        match self.region_containing(address) {
            Some(region) => region.read_u64(address, assert_aligned),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn get_slice(&self, address: Address) -> Result<&[u8], Exception> {
        match self.region_containing(address) {
            Some(region) => region.get_slice(address),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn get_slice_mut(&mut self, address: Address) -> Result<&mut [u8], Exception> {
        match self.region_containing_mut(address) {
            Some(region) => region.get_slice_mut(address),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        match self.region_containing_mut(address) {
            Some(region) => region.write_u8(address, value),
            None => Err(Exception::InvalidStore(address)),
        }
    }

    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        match self.region_containing_mut(address) {
            Some(region) => region.write_u16(address, value, assert_aligned),
            None => Err(Exception::InvalidStore(address)),
        }
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        match self.region_containing_mut(address) {
            Some(region) => region.write_u32(address, value, assert_aligned),
            None => Err(Exception::InvalidStore(address)),
        }
    }

    fn write_u64(
        &mut self,
        address: Address,
        value: u64,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        match self.region_containing_mut(address) {
            Some(region) => region.write_u64(address, value, assert_aligned),
            None => Err(Exception::InvalidStore(address)),
        }
    }
}

impl DataMemory {
    pub fn new(
        r#extern: DataRegion,
        data: DataRegion,
        heap: DataRegion,
        stack: DataRegion,
        kdata: DataRegion,
        mmio: DataRegion,
    ) -> Self {
        let next_heap_address: Address = heap.addresses.start;
        let free_heap_space: u32 = heap.addresses.len() as u32;
        Self {
            r#extern,
            data,
            heap,
            stack,
            kdata,
            mmio,
            next_heap_address,
            free_heap_space,
        }
    }

    pub fn used_heap_space(&self) -> u32 {
        self.heap.addresses.len() as u32 - self.free_heap_space
    }

    fn region_containing(&self, address: Address) -> Option<&DataRegion> {
        if self.r#extern.contains(address) {
            Some(&self.r#extern)
        } else if self.data.contains(address) {
            Some(&self.data)
        } else if self.heap.contains(address) {
            Some(&self.heap)
        } else if self.stack.contains(address) {
            Some(&self.stack)
        } else if self.kdata.contains(address) {
            Some(&self.kdata)
        } else if self.mmio.contains(address) {
            Some(&self.mmio)
        } else {
            None
        }
    }

    fn region_containing_mut(&mut self, address: Address) -> Option<&mut DataRegion> {
        if self.r#extern.contains(address) {
            Some(&mut self.r#extern)
        } else if self.data.contains(address) {
            Some(&mut self.data)
        } else if self.heap.contains(address) {
            Some(&mut self.heap)
        } else if self.stack.contains(address) {
            Some(&mut self.stack)
        } else if self.kdata.contains(address) {
            Some(&mut self.kdata)
        } else if self.mmio.contains(address) {
            Some(&mut self.mmio)
        } else {
            None
        }
    }
}
