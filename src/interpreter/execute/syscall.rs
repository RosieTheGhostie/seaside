use super::{
    super::{memory::regions::Region, syscalls::Syscalls},
    Exception, Interpreter,
};
use crate::{
    constants::{register, service_codes},
    type_aliases::address::Address,
};
use std::{
    ffi::CStr,
    fs::OpenOptions,
    io::{stdin, Read, Write},
    mem::transmute,
    thread::sleep,
    time::{Duration, SystemTime},
};

impl Interpreter {
    pub fn syscall(&mut self) -> Result<(), Exception> {
        use service_codes::*;
        let service_code = self.registers.read_u32_from_cpu(register::V0)? as u8;
        let syscall_flag = Syscalls::from_bits_truncate(1 << service_code);
        if !self.syscalls.intersects(syscall_flag) {
            return Err(Exception::SyscallFailure);
        }
        match service_code {
            PRINT_INT => self.print_int(),
            PRINT_FLOAT => self.print_float(),
            PRINT_DOUBLE => self.print_double(),
            PRINT_STRING => self.print_string(),
            READ_INT => self.read_int(),
            READ_FLOAT => self.read_float(),
            READ_DOUBLE => self.read_double(),
            READ_STRING => self.read_string(),
            SBRK => self.sbrk(),
            EXIT => self.exit(),
            PRINT_CHAR => self.print_char(),
            READ_CHAR => self.read_char(),
            OPEN_FILE => self.open_file(),
            READ_FILE => self.read_file(),
            WRITE_FILE => self.write_file(),
            CLOSE_FILE => self.close_file(),
            EXIT_2 => self.exit_2(),
            TIME => self.time(),
            MIDI_OUT => self.midi_out(),
            SLEEP => self.sleep(),
            MIDI_OUT_SYNC => self.midi_out_sync(),
            PRINT_HEX => self.print_hex(),
            PRINT_BIN => self.print_bin(),
            PRINT_UINT => self.print_uint(),
            SET_SEED => self.set_seed(),
            RAND_INT => self.rand_int(),
            RAND_INT_RANGE => self.rand_int_range(),
            RAND_FLOAT => self.rand_float(),
            RAND_DOUBLE => self.rand_double(),
            _ => Err(Exception::SyscallFailure),
        }
    }
}

impl Interpreter {
    fn print_int(&mut self) -> Result<(), Exception> {
        let x: i32 = self.registers.read_i32_from_cpu(register::A0)?;
        print!("{x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    fn print_float(&mut self) -> Result<(), Exception> {
        let x: f32 = self.registers.read_f32_from_fpu(12)?;
        print!("{x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    fn print_double(&mut self) -> Result<(), Exception> {
        let x: f64 = self.registers.read_f64_from_fpu(12)?;
        print!("{x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    fn print_string(&mut self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read_u32_from_cpu(register::A0)?;
        let string = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure)?
            .to_str()
            .map_err(|_| Exception::SyscallFailure)?;
        print!("{string}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    fn read_int(&mut self) -> Result<(), Exception> {
        let mut buffer = String::new();
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure)?;
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
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure)?;
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
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure)?;
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
        // To appease the borrow checker, we must flush up here instead of directly before the read
        // from stdin like the other read services.
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure)?;
        let buffer_address: Address = self.registers.read_u32_from_cpu(register::A0)?;
        let buffer = self.memory.get_slice_mut(buffer_address)?;
        let max_chars = usize::min(
            self.registers.read_u32_from_cpu(register::A1)? as usize,
            buffer.len(),
        );
        if max_chars == 0 {
            return Ok(());
        }
        let mut buffer = &mut buffer[..max_chars];
        let mut temp = String::new();
        stdin()
            .read_line(&mut temp)
            .map_err(|_| Exception::SyscallFailure)?;
        let slice = match temp.strip_suffix("\r\n") {
            Some(stripped) => stripped,
            None => &temp,
        };
        let bytes: Vec<u8> = {
            let mut bytes = slice.as_bytes().to_vec();
            bytes.push(b'\n');
            bytes.truncate(max_chars);
            if bytes.len() == max_chars {
                bytes[max_chars - 1] = b'\0';
            } else {
                bytes.push(b'\0');
            }
            bytes
        };
        buffer
            .write_all(&bytes)
            .map_err(|_| Exception::SyscallFailure)
    }

    fn sbrk(&mut self) -> Result<(), Exception> {
        let _bytes_to_allocate: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        todo!("how does sbrk work???");
    }

    fn exit(&mut self) -> Result<(), Exception> {
        self.exit_code = Some(0);
        Ok(())
    }

    fn print_char(&mut self) -> Result<(), Exception> {
        let c = match char::from_u32(self.registers.read_u32_from_cpu(register::A0)?) {
            Some(c) => c,
            None => return Err(Exception::SyscallFailure),
        };
        print!("{c}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    fn read_char(&mut self) -> Result<(), Exception> {
        // NOTE: I have absolutely no confidence in this method.
        let mut buffer = [0u8; 4];
        self.flush_stdout_if_necessary()
            .map_err(|_| Exception::SyscallFailure)?;
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
        let file_name_address = self.registers.read_u32_from_cpu(register::A0)?;
        let file_name = CStr::from_bytes_until_nul(self.memory.get_slice(file_name_address)?)
            .map_err(|_| Exception::SyscallFailure)?
            .to_str()
            .map_err(|_| Exception::SyscallFailure)?;
        let flags = self.registers.read_u32_from_cpu(register::A1)?;
        // The `mode` parameter is currently ignored by both MARS and seaside.
        let _mode = self.registers.read_u32_from_cpu(register::A2)?;
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
        self.registers.write_u32_to_cpu(register::V0, fd)
    }

    fn read_file(&mut self) -> Result<(), Exception> {
        let fd = self.registers.read_u32_from_cpu(register::A0)?;
        let buffer_address = self.registers.read_u32_from_cpu(register::A1)?;
        let buffer = self.memory.get_slice_mut(buffer_address)?;
        let max_chars = usize::min(
            self.registers.read_u32_from_cpu(register::A2)? as usize,
            buffer.len(),
        );
        if max_chars == 0 {
            return Ok(());
        }
        let buffer = &mut buffer[..max_chars];
        let bytes_read = match self.files.get_mut(&fd) {
            Some(handle) => handle.read(buffer).map_or(u32::MAX, |n| n as u32),
            None => u32::MAX,
        };
        self.registers.write_u32_to_cpu(register::V0, bytes_read)
    }

    fn write_file(&mut self) -> Result<(), Exception> {
        let fd = self.registers.read_u32_from_cpu(register::A0)?;
        let buffer_address = self.registers.read_u32_from_cpu(register::A1)?;
        let buffer = self.memory.get_slice(buffer_address)?;
        let max_chars = usize::min(
            self.registers.read_u32_from_cpu(register::A2)? as usize,
            buffer.len(),
        );
        let buffer = &buffer[..max_chars];
        let bytes_written = match self.files.get_mut(&fd) {
            Some(handle) => handle.write(buffer).map_or(u32::MAX, |n| n as u32),
            None => u32::MAX,
        };
        self.registers.write_u32_to_cpu(register::V0, bytes_written)
    }

    fn close_file(&mut self) -> Result<(), Exception> {
        let fd = self.registers.read_u32_from_cpu(register::A0)?;
        // For whatever reason, MARS doesn't complain if you try to close any of the special files
        // (stdin, stdout, and stderr). I disagree with that, but to maintain compatibility with it,
        // I'll ignore the result.
        let _succeeded = self.close_file_handle(fd);
        Ok(())
    }

    fn exit_2(&mut self) -> Result<(), Exception> {
        let exit_code = self.registers.read_u32_from_cpu(register::A0)?;
        self.exit_code = Some((exit_code & 0xFF) as u8);
        Ok(())
    }

    fn time(&mut self) -> Result<(), Exception> {
        // NOTE: Byte order shenanigans will probably mess things up.
        let system_time: u64 = match SystemTime::UNIX_EPOCH.elapsed() {
            Ok(duration) => duration.as_millis() as u64,
            Err(_) => return Err(Exception::SyscallFailure),
        };
        let halves: [u32; 2] = unsafe { transmute::<u64, [u32; 2]>(system_time) };
        self.registers.write_u32_to_cpu(register::A0, halves[0])?;
        self.registers.write_u32_to_cpu(register::A1, halves[1])
    }

    fn midi_out(&self) -> Result<(), Exception> {
        let _pitch = self.registers.read_u32_from_cpu(register::A0)?;
        let _millis = self.registers.read_u32_from_cpu(register::A1)?;
        let _instrument = self.registers.read_u32_from_cpu(register::A2)?;
        let _volume = self.registers.read_u32_from_cpu(register::A3)?;
        todo!("generate a sound");
    }

    fn sleep(&self) -> Result<(), Exception> {
        let millis: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        sleep(Duration::from_millis(millis as u64));
        Ok(())
    }

    fn midi_out_sync(&self) -> Result<(), Exception> {
        let _pitch = self.registers.read_u32_from_cpu(register::A0)?;
        let _millis = self.registers.read_u32_from_cpu(register::A1)?;
        let _instrument = self.registers.read_u32_from_cpu(register::A2)?;
        let _volume = self.registers.read_u32_from_cpu(register::A3)?;
        todo!("generate a sound");
    }

    fn print_hex(&mut self) -> Result<(), Exception> {
        let x: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        print!("0x{x:08x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    fn print_bin(&mut self) -> Result<(), Exception> {
        let x: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        print!("0b{x:032b}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    fn print_uint(&mut self) -> Result<(), Exception> {
        let x: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        print!("{x}");
        self.stdout_pending_flush = true;
        Ok(())
    }

    fn set_seed(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        let seed: u64 = self.registers.read_u32_from_cpu(register::A1)? as u64;
        self.set_rng_seed(id, seed);
        Ok(())
    }

    fn rand_int(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        let rng = match self.rngs.get_mut(&id) {
            Some(rng) => rng,
            None => self.make_rng(id),
        };
        let x: u32 = rng.next_u32();
        self.registers.write_u32_to_cpu(register::A0, x)
    }

    fn rand_int_range(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        let upper_bound: u64 = self.registers.read_u32_from_cpu(register::A1)? as u64;
        let rng = match self.rngs.get_mut(&id) {
            Some(rng) => rng,
            None => self.make_rng(id),
        };
        match rng.next_u32_from_range(upper_bound) {
            Some(x) => self.registers.write_u32_to_cpu(register::A0, x),
            None => Err(Exception::SyscallFailure),
        }
    }

    fn rand_float(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        let rng = match self.rngs.get_mut(&id) {
            Some(rng) => rng,
            None => self.make_rng(id),
        };
        let x: f32 = rng.next_f32();
        self.registers.write_f32_to_fpu(0, x)
    }

    fn rand_double(&mut self) -> Result<(), Exception> {
        let id: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        let rng = match self.rngs.get_mut(&id) {
            Some(rng) => rng,
            None => self.make_rng(id),
        };
        let x: f64 = rng.next_f64();
        self.registers.write_f64_to_fpu(0, x)
    }
}
