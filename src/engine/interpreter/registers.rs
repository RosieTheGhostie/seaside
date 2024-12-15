use crate::engine::{Error, ErrorKind};
use std::mem::transmute;

#[derive(Default)]
pub struct Registers {
    cpu: [u32; 32],
    pub hi: u32,
    pub lo: u32,
    fpu: [f32; 32],
    fpu_flags: u8,
    pub vaddr: u32,
    pub status: u32,
    pub cause: u32,
    pub epc: u32,
}

impl Registers {
    pub fn read_from_cpu(&self, index: u8) -> Result<u32, Error> {
        if index < 32 {
            Ok(self.cpu[index as usize])
        } else {
            Err(Error::from(ErrorKind::InternalLogicIssue))
        }
    }

    pub fn read_f32_from_fpu(&self, index: u8) -> Result<f32, Error> {
        if index < 32 {
            Ok(self.fpu[index as usize])
        } else {
            Err(Error::from(ErrorKind::InternalLogicIssue))
        }
    }

    pub fn read_f64_from_fpu(&self, index: u8) -> Result<f64, Error> {
        if index % 2 == 0 && index < 32 {
            let registers = [self.fpu[index as usize], self.fpu[index as usize + 1]];
            Ok(unsafe { transmute::<[f32; 2], f64>(registers) })
        } else {
            Err(Error::from(ErrorKind::InternalLogicIssue))
        }
    }

    pub fn read_flag_from_fpu(&self, index: u8) -> Result<bool, Error> {
        if index < 8 {
            Ok((self.fpu_flags >> index) & 1 == 1)
        } else {
            Err(Error::from(ErrorKind::InternalLogicIssue))
        }
    }

    pub fn write_to_cpu(&mut self, index: u8, value: u32) -> Result<(), Error> {
        if index < 32 {
            self.cpu[index as usize] = value;
            Ok(())
        } else {
            Err(Error::from(ErrorKind::InternalLogicIssue))
        }
    }

    pub fn write_f32_to_fpu(&mut self, index: u8, value: f32) -> Result<(), Error> {
        if index < 32 {
            self.fpu[index as usize] = value;
            Ok(())
        } else {
            Err(Error::from(ErrorKind::InternalLogicIssue))
        }
    }

    pub fn write_f64_to_fpu(&mut self, index: u8, value: f64) -> Result<(), Error> {
        if index % 2 == 0 && index < 32 {
            let index = index as usize;
            let halves: [f32; 2] = unsafe { transmute::<f64, [f32; 2]>(value) };
            self.fpu[index] = halves[0];
            self.fpu[index + 1] = halves[1];
            Ok(())
        } else {
            Err(Error::from(ErrorKind::InternalLogicIssue))
        }
    }

    pub fn write_flag_to_fpu(&mut self, index: u8, value: bool) -> Result<(), Error> {
        if index < 8 {
            let mask: u8 = 1u8 << index;
            let value = if value { mask } else { 0 };
            self.fpu_flags &= !(1u8 << index);
            self.fpu_flags |= value;
            Ok(())
        } else {
            Err(Error::from(ErrorKind::InternalLogicIssue))
        }
    }
}
