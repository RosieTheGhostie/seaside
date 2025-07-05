use crate::{
    Exception, Interpreter, InterpreterState, SyscallFailureKind,
    memory::Region,
    register_file::{IndexByRegister, TryIndexByRegister},
};
use console::Term;
use seaside_constants::register::{CpuRegister, FpuRegister};
use seaside_type_aliases::Address;
use std::{
    ffi::CStr,
    fs::OpenOptions,
    io::{Read, Write, stdin},
    thread::sleep,
    time::{Duration, SystemTime},
};

impl Interpreter {
    pub fn syscall(&mut self) -> Result<(), Exception> {
        let service_code: u32 = self.state.registers.read(CpuRegister::Val0);
        let service = self.services.get(&service_code).ok_or_else(|| {
            Exception::SyscallFailure(SyscallFailureKind::UnknownServiceCode(service_code))
        })?;
        service(&mut self.state)
    }
}

impl InterpreterState {
    pub fn print_int(&mut self) -> Result<(), Exception> {
        let x: i32 = self.registers.read(CpuRegister::Arg0);
        print!("{x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    pub fn print_float(&mut self) -> Result<(), Exception> {
        let x: f32 = self.registers.read(FpuRegister::F12);
        print!("{x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    pub fn print_double(&mut self) -> Result<(), Exception> {
        let x: f64 = self.registers.try_read(FpuRegister::F12)?;
        print!("{x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    pub fn print_string(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let string = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        print!("{string}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    pub fn read_int(&mut self) -> Result<(), Exception> {
        let mut buffer = String::new();
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdoutFlushFailed))?;
        stdin()
            .read_line(&mut buffer)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdinReadFailed))?;
        let parsed: i32 = buffer
            .trim()
            .parse()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::ParseError))?;
        self.registers.write(CpuRegister::Val0, parsed);
        Ok(())
    }

    pub fn read_float(&mut self) -> Result<(), Exception> {
        let mut buffer = String::new();
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdoutFlushFailed))?;
        stdin()
            .read_line(&mut buffer)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdinReadFailed))?;
        let parsed: f32 = buffer
            .trim()
            .parse()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::ParseError))?;
        self.registers.write(FpuRegister::F0, parsed);
        Ok(())
    }

    pub fn read_double(&mut self) -> Result<(), Exception> {
        let mut buffer = String::new();
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdoutFlushFailed))?;
        stdin()
            .read_line(&mut buffer)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdinReadFailed))?;
        let parsed: f64 = buffer
            .trim()
            .parse()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::ParseError))?;
        self.registers.try_write(FpuRegister::F0, parsed)
    }

    pub fn read_string(&mut self) -> Result<(), Exception> {
        // To appease the borrow checker, we must flush up here instead of directly before the read
        // from stdin like the other read services.
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdoutFlushFailed))?;
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let buffer = self.memory.get_slice_mut(buffer_address)?;
        let max_bytes = usize::min(
            <_ as IndexByRegister<_, u32>>::read(&self.registers, CpuRegister::Arg1) as usize,
            buffer.len(),
        );
        if max_bytes == 0 {
            return Ok(());
        }
        let mut buffer = &mut buffer[..max_bytes];
        let mut temp = String::new();
        stdin()
            .read_line(&mut temp)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdinReadFailed))?;
        let slice = temp
            .strip_suffix('\n')
            .unwrap_or(&temp)
            .strip_suffix('\r')
            .unwrap_or(&temp);
        let bytes: Vec<u8> = {
            let mut bytes = slice.as_bytes().to_vec();
            bytes.push(b'\n');
            bytes.truncate(max_bytes);
            if bytes.len() == max_bytes {
                bytes[max_bytes - 1] = b'\0';
            } else {
                bytes.push(b'\0');
            }
            bytes
        };
        buffer
            .write_all(&bytes)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::WriteFailed))
    }

    pub fn sbrk(&mut self, freeable_heap_allocations: bool) -> Result<(), Exception> {
        let n_bytes: i32 = self.registers.read(CpuRegister::Arg0);
        // Adjust value of `n_bytes` to be a multiple of four.
        let should_allocate: bool = n_bytes > 0;
        let mut n_bytes: u32 = n_bytes.unsigned_abs();
        if n_bytes & 0b11 != 0 {
            n_bytes = ((n_bytes >> 2) + 1) << 2;
        }

        let address = if should_allocate {
            let free_heap_space = self.memory.free_heap_space_mut();
            if let Some(new_free_space) = free_heap_space.checked_sub(n_bytes) {
                *free_heap_space = new_free_space;
                let next_available = self.memory.next_heap_address_mut();
                let address = *next_available;
                *next_available += n_bytes;
                address
            } else {
                0
            }
        } else if freeable_heap_allocations {
            n_bytes = n_bytes.min(self.memory.used_heap_space());
            *self.memory.free_heap_space_mut() += n_bytes;
            *self.memory.next_heap_address_mut() -= n_bytes;
            0
        } else {
            return Err(Exception::SyscallFailure(
                SyscallFailureKind::HeapFreeDisabled,
            ));
        };

        self.registers.write(CpuRegister::Val0, address);
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), Exception> {
        self.exit_code = Some(0);
        Ok(())
    }

    pub fn print_char(&mut self) -> Result<(), Exception> {
        let c = char::from_u32(self.registers.read(CpuRegister::Arg0))
            .ok_or(Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        print!("{c}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    pub fn read_char(&mut self) -> Result<(), Exception> {
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdoutFlushFailed))?;
        // No idea why we're supposedly reading from stdout, but this works.
        let input = Term::buffered_stdout()
            .read_char()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::StdinReadFailed))?;
        // `Term::read_char` hides what it reads, so we have to show it ourselves.
        print!("{input}");
        self.stdout_pending_flush = true;
        self.registers.write(CpuRegister::Val0, input as u32);
        Ok(())
    }

    pub fn open_file(&mut self) -> Result<(), Exception> {
        let file_name_address: Address = self.registers.read(CpuRegister::Arg0);
        let file_name = CStr::from_bytes_until_nul(self.memory.get_slice(file_name_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        let flags: u32 = self.registers.read(CpuRegister::Arg1);
        // The `mode` parameter is currently ignored by both MARS and seaside.
        let _mode: u32 = self.registers.read(CpuRegister::Arg2);
        let fd: u32 = match OpenOptions::new()
            .read(flags == 0)
            .write(flags & 1 != 0)
            .create(flags & 1 != 0)
            .append(flags & 8 != 0)
            .open(file_name)
        {
            Ok(file) => {
                let fd = self.next_fd;
                self.make_file_handle(file);
                fd
            }
            Err(_) => u32::MAX,
        };
        self.registers.write(CpuRegister::Val0, fd);
        Ok(())
    }

    pub fn read_file(&mut self) -> Result<(), Exception> {
        let fd: u32 = self.registers.read(CpuRegister::Arg0);
        let buffer_address: Address = self.registers.read(CpuRegister::Arg1);
        let buffer = self.memory.get_slice_mut(buffer_address)?;
        let max_bytes = usize::min(
            <_ as IndexByRegister<_, u32>>::read(&self.registers, CpuRegister::Arg2) as usize,
            buffer.len(),
        );
        if max_bytes == 0 {
            return Ok(());
        }
        let buffer = &mut buffer[..max_bytes];
        let bytes_read = match self.files.get_mut(&fd) {
            Some(handle) => handle.read(buffer).map_or(u32::MAX, |n| n as u32),
            None => u32::MAX,
        };
        self.registers.write(CpuRegister::Val0, bytes_read);
        Ok(())
    }

    pub fn write_file(&mut self) -> Result<(), Exception> {
        let fd: u32 = self.registers.read(CpuRegister::Arg0);
        let buffer_address: Address = self.registers.read(CpuRegister::Arg1);
        let buffer = self.memory.get_slice(buffer_address)?;
        let max_bytes = usize::min(
            <_ as IndexByRegister<_, u32>>::read(&self.registers, CpuRegister::Arg2) as usize,
            buffer.len(),
        );
        let buffer = &buffer[..max_bytes];
        let bytes_written = match self.files.get_mut(&fd) {
            Some(handle) => handle.write(buffer).map_or(u32::MAX, |n| n as u32),
            None => u32::MAX,
        };
        self.registers.write(CpuRegister::Val0, bytes_written);
        Ok(())
    }

    pub fn close_file(&mut self) -> Result<(), Exception> {
        let fd: u32 = self.registers.read(CpuRegister::Arg0);
        // For whatever reason, MARS doesn't complain if you try to close any of the special files
        // (stdin, stdout, and stderr). I disagree with that, but to maintain compatibility with it,
        // I'll ignore the result.
        let _succeeded = self.close_file_handle(fd);
        Ok(())
    }

    pub fn exit_2(&mut self) -> Result<(), Exception> {
        let exit_code: u32 = self.registers.read(CpuRegister::Arg0);
        self.exit_code = Some((exit_code & 0xff) as u8);
        Ok(())
    }

    pub fn time(&mut self) -> Result<(), Exception> {
        let system_time: u64 = match SystemTime::UNIX_EPOCH.elapsed() {
            Ok(duration) => duration.as_millis() as u64,
            Err(_) => {
                return Err(Exception::SyscallFailure(
                    SyscallFailureKind::BeforeUnixEpoch,
                ));
            }
        };
        let upper_half = (system_time >> 32) as u32;
        let lower_half = (system_time & u32::MAX as u64) as u32;
        self.registers.write(CpuRegister::Arg0, lower_half);
        self.registers.write(CpuRegister::Arg1, upper_half);
        Ok(())
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn midi_out(&mut self) -> Result<(), Exception> {
        let _pitch: u32 = self.registers.read(CpuRegister::Arg0);
        let _millis: u32 = self.registers.read(CpuRegister::Arg1);
        let _instrument: u32 = self.registers.read(CpuRegister::Arg2);
        let _volume: u32 = self.registers.read(CpuRegister::Arg3);
        todo!("generate a sound");
    }

    pub fn sleep(&mut self) -> Result<(), Exception> {
        let millis: u32 = self.registers.read(CpuRegister::Arg0);
        sleep(Duration::from_millis(millis as u64));
        Ok(())
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn midi_out_sync(&mut self) -> Result<(), Exception> {
        let _pitch: u32 = self.registers.read(CpuRegister::Arg0);
        let _millis: u32 = self.registers.read(CpuRegister::Arg1);
        let _instrument: u32 = self.registers.read(CpuRegister::Arg2);
        let _volume: u32 = self.registers.read(CpuRegister::Arg3);
        todo!("generate a sound");
    }

    pub fn print_hex(&mut self) -> Result<(), Exception> {
        let x: u32 = self.registers.read(CpuRegister::Arg0);
        print!("0x{x:08x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    pub fn print_bin(&mut self) -> Result<(), Exception> {
        let x: u32 = self.registers.read(CpuRegister::Arg0);
        print!("0b{x:032b}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    pub fn print_uint(&mut self) -> Result<(), Exception> {
        let x: u32 = self.registers.read(CpuRegister::Arg0);
        print!("{x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    pub fn set_seed(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read(CpuRegister::Arg0);
        let seed = <_ as IndexByRegister<_, u32>>::read(&self.registers, CpuRegister::Arg1) as u64;
        self.set_rng_seed(id, seed);
        Ok(())
    }

    pub fn rand_int(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read(CpuRegister::Arg0);
        let rng = match self.rngs.get_mut(&id) {
            Some(rng) => rng,
            None => self.make_rng(id),
        };
        let x: u32 = rng.next_u32();
        self.registers.write(CpuRegister::Arg0, x);
        Ok(())
    }

    pub fn rand_int_range(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read(CpuRegister::Arg0);
        let upper_bound =
            <_ as IndexByRegister<_, u32>>::read(&self.registers, CpuRegister::Arg1) as u64;
        let rng = match self.rngs.get_mut(&id) {
            Some(rng) => rng,
            None => self.make_rng(id),
        };
        match rng.next_u32_from_range(upper_bound) {
            Some(x) => {
                self.registers.write(CpuRegister::Arg0, x);
                Ok(())
            }
            None => Err(Exception::SyscallFailure(
                SyscallFailureKind::NoPossibleOutput,
            )),
        }
    }

    pub fn rand_float(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read(CpuRegister::Arg0);
        let rng = match self.rngs.get_mut(&id) {
            Some(rng) => rng,
            None => self.make_rng(id),
        };
        let x: f32 = rng.next_f32();
        self.registers.write(FpuRegister::F0, x);
        Ok(())
    }

    pub fn rand_double(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read(CpuRegister::Arg0);
        let rng = match self.rngs.get_mut(&id) {
            Some(rng) => rng,
            None => self.make_rng(id),
        };
        let x: f64 = rng.next_f64();
        self.registers.try_write(FpuRegister::F0, x)
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn confirm_dialog(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let _message = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        todo!("show the dialog and return the result");
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn input_dialog_int(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let _message = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        todo!("show the dialog and return the result");
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn input_dialog_float(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let _message = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        todo!("show the dialog and return the result");
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn input_dialog_double(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let _message = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        todo!("show the dialog and return the result");
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn input_dialog_string(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let _message = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        todo!("show the dialog and return the result");
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn message_dialog(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let _message = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        let _message_type: u32 = self.registers.read(CpuRegister::Arg1);
        todo!("show the dialog");
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn message_dialog_int(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let _message = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        let _x: u32 = self.registers.read(CpuRegister::Arg1);
        todo!("show the dialog");
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn message_dialog_float(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let _message = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        let _x: f32 = self.registers.read(FpuRegister::F12);
        todo!("show the dialog");
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn message_dialog_double(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read(CpuRegister::Arg0);
        let _message = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        let _x: f64 = self.registers.try_read(FpuRegister::F12)?;
        todo!("show the dialog");
    }

    #[allow(dead_code, reason = "not yet implemented")]
    pub fn message_dialog_string(&mut self) -> Result<(), Exception> {
        let buffer_address_0: Address = self.registers.read(CpuRegister::Arg0);
        let _message_0 = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address_0)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        let buffer_address_1: Address = self.registers.read(CpuRegister::Arg1);
        let _message_1 = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address_1)?)
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::NulNotFound))?
            .to_str()
            .map_err(|_| Exception::SyscallFailure(SyscallFailureKind::InvalidUtf8))?;
        todo!("show the dialog");
    }
}
