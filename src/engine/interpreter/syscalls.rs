use std::{
    io::{stdin, Read},
    mem::transmute,
};

use super::{exception::Exception, Interpreter};
use crate::{
    config::memory_map::Address,
    constants::{register, service_codes},
};
use bitflags::bitflags;

bitflags! {
    pub struct Syscalls: u64 {
        const PrintInt = 1 << service_codes::PRINT_INT;
        const PrintFloat = 1 << service_codes::PRINT_FLOAT;
        const PrintDouble = 1 << service_codes::PRINT_DOUBLE;
        const PrintString = 1 << service_codes::PRINT_STRING;
        const ReadInt = 1 << service_codes::READ_INT;
        const ReadFloat = 1 << service_codes::READ_FLOAT;
        const ReadDouble = 1 << service_codes::READ_DOUBLE;
        const ReadString = 1 << service_codes::READ_STRING;
        const Sbrk = 1 << service_codes::SBRK;
        const Exit = 1 << service_codes::EXIT;
        const PrintChar = 1 << service_codes::PRINT_CHAR;
        const ReadChar = 1 << service_codes::READ_CHAR;
        const OpenFile = 1 << service_codes::OPEN_FILE;
        const ReadFile = 1 << service_codes::READ_FILE;
        const WriteFile = 1 << service_codes::WRITE_FILE;
        const CloseFile = 1 << service_codes::CLOSE_FILE;
        const Exit2 = 1 << service_codes::EXIT_2;
    }
}

impl Interpreter {
    fn print_int(&self) -> Result<(), Exception> {
        let x: i32 = self.registers.read_i32_from_cpu(register::A0)?;
        print!("{x}");
        Ok(())
    }

    fn print_float(&self) -> Result<(), Exception> {
        let x: f32 = self.registers.read_f32_from_fpu(12)?;
        print!("{x}");
        Ok(())
    }

    fn print_double(&self) -> Result<(), Exception> {
        let x: f64 = self.registers.read_f64_from_fpu(12)?;
        print!("{x}");
        Ok(())
    }

    fn print_string(&self) -> Result<(), Exception> {
        let _buffer_address: Address = self.registers.read_u32_from_cpu(register::A0)?;
        todo!("fetch the string from memory");
    }

    fn read_int(&mut self) -> Result<(), Exception> {
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .map_err(|_| Exception::SyscallFailure)?;
        let parsed: i32 = buffer
            .trim()
            .parse()
            .map_err(|_| Exception::SyscallFailure)?;
        self.registers.write_i32_to_cpu(register::V0, parsed)
    }

    fn read_float(&mut self) -> Result<(), Exception> {
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .map_err(|_| Exception::SyscallFailure)?;
        let parsed: f32 = buffer
            .trim()
            .parse()
            .map_err(|_| Exception::SyscallFailure)?;
        self.registers.write_f32_to_fpu(0, parsed)
    }

    fn read_double(&mut self) -> Result<(), Exception> {
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .map_err(|_| Exception::SyscallFailure)?;
        let parsed: f64 = buffer
            .trim()
            .parse()
            .map_err(|_| Exception::SyscallFailure)?;
        self.registers.write_f64_to_fpu(0, parsed)
    }

    fn read_string(&mut self) -> Result<(), Exception> {
        let _buffer_address: Address = self.registers.read_u32_from_cpu(register::A0)?;
        let _max_chars: u32 = self.registers.read_u32_from_cpu(register::A1)?;
        todo!("fetch the buffer from memory");
    }

    fn sbrk(&mut self) -> Result<(), Exception> {
        let _bytes_to_allocate: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        todo!("how does sbrk work???");
    }

    fn exit(&self) -> Result<(), Exception> {
        todo!("call a method to shut down the interpreter");
    }

    fn print_char(&self) -> Result<(), Exception> {
        let c = match char::from_u32(self.registers.read_u32_from_cpu(register::A0)?) {
            Some(c) => c,
            None => return Err(Exception::SyscallFailure),
        };
        print!("{c}");
        Ok(())
    }

    fn read_char(&mut self) -> Result<(), Exception> {
        // NOTE: I have absolutely no confidence in this method.
        let mut buffer = [0u8; 4];
        stdin()
            .read_exact(&mut buffer)
            .map_err(|_| Exception::SyscallFailure)?;
        let input = unsafe { transmute::<[u8; 4], u32>(buffer) };
        match char::from_u32(input) {
            Some(_) => self.registers.write_u32_to_cpu(register::V0, input),
            None => Err(Exception::SyscallFailure),
        }
    }

    fn open_file(&mut self) -> Result<(), Exception> {
        let _file_name_address = self.registers.read_u32_from_cpu(register::A0)?;
        let _flags = self.registers.read_u32_from_cpu(register::A1)?;
        let _mode = self.registers.read_u32_from_cpu(register::A2)?;
        todo!("get the file name, open the file, and set $v0 to the fd/error code");
    }

    fn read_file(&mut self) -> Result<(), Exception> {
        let _fd = self.registers.read_u32_from_cpu(register::A0)?;
        let _buffer_address = self.registers.read_u32_from_cpu(register::A1)?;
        let _max_chars = self.registers.read_u32_from_cpu(register::A2)?;
        todo!("read stuff from the file into the buffer and set $v0 accordingly");
    }

    fn write_file(&mut self) -> Result<(), Exception> {
        let _fd = self.registers.read_u32_from_cpu(register::A0)?;
        let _buffer_address = self.registers.read_u32_from_cpu(register::A1)?;
        let _chars_to_write = self.registers.read_u32_from_cpu(register::A2)?;
        todo!("write stuff from the buffer to the file and set $v0 accordingly");
    }

    fn close_file(&mut self) -> Result<(), Exception> {
        let _fd = self.registers.read_u32_from_cpu(register::A0)?;
        todo!("close the file");
    }

    fn exit_2(&mut self) -> Result<(), Exception> {
        let _exit_code = self.registers.read_u32_from_cpu(register::A0)?;
        todo!("call a method to shut down the interpreter (include the exit code)");
    }
}
