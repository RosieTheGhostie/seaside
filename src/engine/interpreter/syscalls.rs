use super::{exception::Exception, Interpreter};
use crate::{
    config::memory_map::Address,
    constants::{register, service_codes},
};
use bitflags::bitflags;
use std::{
    io::{stdin, Read},
    mem::transmute,
    thread::sleep,
    time::{Duration, SystemTime},
};

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

        const Time = 1 << service_codes::TIME;
        const MidiOut = 1 << service_codes::MIDI_OUT;
        const Sleep = 1 << service_codes::SLEEP;
        const MidiOutSync = 1 << service_codes::MIDI_OUT_SYNC;
        const PrintHex = 1 << service_codes::PRINT_HEX;
        const PrintBin = 1 << service_codes::PRINT_BIN;
        const PrintUint = 1 << service_codes::PRINT_UINT;

        const SetSeed = 1 << service_codes::SET_SEED;
        const RandInt = 1 << service_codes::RAND_INT;
        const RandIntRange = 1 << service_codes::RAND_INT_RANGE;
        const RandFloat = 1 << service_codes::RAND_FLOAT;
        const RandDouble = 1 << service_codes::RAND_DOUBLE;

        const ConfirmDialog = 1 << service_codes::CONFIRM_DIALOG;
        const InputDialogInt = 1 << service_codes::INPUT_DIALOG_INT;
        const InputDialogFloat = 1 << service_codes::INPUT_DIALOG_FLOAT;
        const InputDialogDouble = 1 << service_codes::INPUT_DIALOG_DOUBLE;
        const InputDialogString = 1 << service_codes::INPUT_DIALOG_STRING;
        const MessageDialog = 1 << service_codes::MESSAGE_DIALOG;
        const MessageDialogInt = 1 << service_codes::MESSAGE_DIALOG_INT;
        const MessageDialogFloat = 1 << service_codes::MESSAGE_DIALOG_FLOAT;
        const MessageDialogDouble = 1 << service_codes::MESSAGE_DIALOG_DOUBLE;
        const MessageDialogString = 1 << service_codes::MESSAGE_DIALOG_STRING;
    }
}

pub trait SyscallExecutor {
    fn syscall(&mut self) -> Result<(), Exception>;
}

impl SyscallExecutor for Interpreter {
    fn syscall(&mut self) -> Result<(), Exception> {
        use service_codes::*;
        let service_code = self.registers.read_u32_from_cpu(register::V0)? as u8;
        match service_code {
            PRINT_INT => self.print_int(),
            PRINT_FLOAT => self.print_float(),
            PRINT_DOUBLE => self.print_double(),
            PRINT_STRING => self.print_string(),
            READ_INT => self.read_int(),
            READ_FLOAT => self.read_float(),
            READ_DOUBLE => self.read_double(),
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
            _ => Err(Exception::SyscallFailure),
        }
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

    fn time(&mut self) -> Result<(), Exception> {
        // NOTE: Byte order shenanigans will probably mess things up.
        let system_time: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
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

    fn print_hex(&self) -> Result<(), Exception> {
        let x: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        print!("0x{x:08x}");
        Ok(())
    }

    fn print_bin(&self) -> Result<(), Exception> {
        let x: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        print!("0b{x:032b}");
        Ok(())
    }

    fn print_uint(&self) -> Result<(), Exception> {
        let x: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        print!("{x}");
        Ok(())
    }
}
