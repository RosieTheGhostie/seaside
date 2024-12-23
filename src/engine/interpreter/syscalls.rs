use super::{Exception, Interpreter};
use crate::{
    config::features::syscalls,
    constants::{register, service_codes},
    engine::interpreter::memory::regions::Region,
    type_aliases::address::Address,
};
use bitflags::bitflags;
use std::io::stdout;
use std::{
    ffi::CStr,
    io::{stdin, Read, Write},
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

impl From<&syscalls::Syscalls> for Syscalls {
    fn from(value: &syscalls::Syscalls) -> Self {
        let mut output = Self::empty();
        output |= Self::from(&value.print);
        output |= Self::from(&value.read);
        output |= Self::from(&value.file);
        output |= Self::from(&value.system);
        output |= Self::from(&value.random);
        output |= Self::from(&value.dialog.input);
        output |= Self::from(&value.dialog.message);
        output
    }
}

impl From<&syscalls::Print> for Syscalls {
    fn from(value: &syscalls::Print) -> Self {
        use syscalls::Print;
        let mut output = Self::empty();
        if value.intersects(Print::Int) {
            output |= Self::PrintInt;
        }
        if value.intersects(Print::Float) {
            output |= Self::PrintFloat;
        }
        if value.intersects(Print::Double) {
            output |= Self::PrintDouble;
        }
        if value.intersects(Print::String) {
            output |= Self::PrintString;
        }
        if value.intersects(Print::Char) {
            output |= Self::PrintChar;
        }
        if value.intersects(Print::Hex) {
            output |= Self::PrintHex;
        }
        if value.intersects(Print::Bin) {
            output |= Self::PrintBin;
        }
        if value.intersects(Print::Uint) {
            output |= Self::PrintUint;
        }
        output
    }
}

impl From<&syscalls::Read> for Syscalls {
    fn from(value: &syscalls::Read) -> Self {
        use syscalls::Read;
        let mut output = Self::empty();
        if value.intersects(Read::Int) {
            output |= Self::ReadInt;
        }
        if value.intersects(Read::Float) {
            output |= Self::ReadFloat;
        }
        if value.intersects(Read::Double) {
            output |= Self::ReadDouble;
        }
        if value.intersects(Read::String) {
            output |= Self::ReadString;
        }
        if value.intersects(Read::Char) {
            output |= Self::ReadChar;
        }
        output
    }
}

impl From<&syscalls::File> for Syscalls {
    fn from(value: &syscalls::File) -> Self {
        use syscalls::File;
        let mut output = Self::empty();
        if value.intersects(File::Open) {
            output |= Self::OpenFile;
        }
        if value.intersects(File::Read) {
            output |= Self::ReadFile;
        }
        if value.intersects(File::Write) {
            output |= Self::WriteFile;
        }
        if value.intersects(File::Close) {
            output |= Self::CloseFile;
        }
        output
    }
}

impl From<&syscalls::System> for Syscalls {
    fn from(value: &syscalls::System) -> Self {
        use syscalls::System;
        let mut output = Self::empty();
        if value.intersects(System::Sbrk) {
            output |= Self::Sbrk;
        }
        if value.intersects(System::Exit) {
            output |= Self::Exit;
        }
        if value.intersects(System::Exit2) {
            output |= Self::Exit2;
        }
        if value.intersects(System::Time) {
            output |= Self::Time;
        }
        if value.intersects(System::Midi) {
            output |= Self::MidiOut;
        }
        if value.intersects(System::Sleep) {
            output |= Self::Sleep;
        }
        if value.intersects(System::MidiSync) {
            output |= Self::MidiOutSync;
        }
        output
    }
}

impl From<&syscalls::Random> for Syscalls {
    fn from(value: &syscalls::Random) -> Self {
        use syscalls::Random;
        let mut output = Self::empty();
        if value.intersects(Random::SetSeed) {
            output |= Self::SetSeed;
        }
        if value.intersects(Random::RandInt) {
            output |= Self::RandInt;
        }
        if value.intersects(Random::RandIntRange) {
            output |= Self::RandIntRange;
        }
        if value.intersects(Random::RandFloat) {
            output |= Self::RandFloat;
        }
        if value.intersects(Random::RandDouble) {
            output |= Self::RandDouble;
        }
        output
    }
}

impl From<&syscalls::Input> for Syscalls {
    fn from(value: &syscalls::Input) -> Self {
        use syscalls::Input;
        let mut output = Self::empty();
        if value.intersects(Input::Confirm) {
            output |= Self::ConfirmDialog;
        }
        if value.intersects(Input::Int) {
            output |= Self::InputDialogInt;
        }
        if value.intersects(Input::Float) {
            output |= Self::InputDialogFloat;
        }
        if value.intersects(Input::Double) {
            output |= Self::InputDialogDouble;
        }
        if value.intersects(Input::String) {
            output |= Self::InputDialogString;
        }
        output
    }
}

impl From<&syscalls::Message> for Syscalls {
    fn from(value: &syscalls::Message) -> Self {
        use syscalls::Message;
        let mut output = Self::empty();
        if value.intersects(Message::General) {
            output |= Self::MessageDialog;
        }
        if value.intersects(Message::Int) {
            output |= Self::MessageDialogInt;
        }
        if value.intersects(Message::Float) {
            output |= Self::MessageDialogFloat;
        }
        if value.intersects(Message::Double) {
            output |= Self::MessageDialogDouble;
        }
        if value.intersects(Message::String) {
            output |= Self::MessageDialogString;
        }
        output
    }
}

impl Interpreter {
    pub fn syscall(&mut self) -> Result<(), Exception> {
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
            _ => Err(Exception::SyscallFailure),
        }
    }
}

impl Interpreter {
    fn print_int(&self) -> Result<(), Exception> {
        let x: i32 = self.registers.read_i32_from_cpu(register::A0)?;
        print!("{x}");
        // We need to flush stdout because the automatic flushing is insufficient in most scenarios.
        stdout().flush().map_err(|_| Exception::SyscallFailure)
    }

    fn print_float(&self) -> Result<(), Exception> {
        let x: f32 = self.registers.read_f32_from_fpu(12)?;
        print!("{x}");
        // We need to flush stdout because the automatic flushing is insufficient in most scenarios.
        stdout().flush().map_err(|_| Exception::SyscallFailure)
    }

    fn print_double(&self) -> Result<(), Exception> {
        let x: f64 = self.registers.read_f64_from_fpu(12)?;
        print!("{x}");
        // We need to flush stdout because the automatic flushing is insufficient in most scenarios.
        stdout().flush().map_err(|_| Exception::SyscallFailure)
    }

    fn print_string(&self) -> Result<(), Exception> {
        let buffer_address: Address = self.registers.read_u32_from_cpu(register::A0)?;
        let string = CStr::from_bytes_until_nul(self.memory.get_slice(buffer_address)?)
            .map_err(|_| Exception::SyscallFailure)?
            .to_str()
            .map_err(|_| Exception::SyscallFailure)?;
        print!("{string}");
        // We need to flush stdout because the automatic flushing is insufficient in most scenarios.
        stdout().flush().map_err(|_| Exception::SyscallFailure)
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
        let temp: Vec<u8> = {
            let mut temp = temp.as_bytes().to_vec();
            temp.truncate(max_chars);
            if temp.len() == max_chars {
                temp[max_chars - 1] = 0;
            } else {
                temp.push(0);
            }
            temp
        };
        buffer
            .write_all(&temp)
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

    fn print_char(&self) -> Result<(), Exception> {
        let c = match char::from_u32(self.registers.read_u32_from_cpu(register::A0)?) {
            Some(c) => c,
            None => return Err(Exception::SyscallFailure),
        };
        print!("{c}");
        // We need to flush stdout because the automatic flushing is insufficient in most scenarios.
        stdout().flush().map_err(|_| Exception::SyscallFailure)
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
        let exit_code = self.registers.read_u32_from_cpu(register::A0)?;
        self.exit_code = Some((exit_code & 0xFF) as u8);
        Ok(())
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
        // We need to flush stdout because the automatic flushing is insufficient in most scenarios.
        stdout().flush().map_err(|_| Exception::SyscallFailure)
    }

    fn print_bin(&self) -> Result<(), Exception> {
        let x: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        print!("0b{x:032b}");
        // We need to flush stdout because the automatic flushing is insufficient in most scenarios.
        stdout().flush().map_err(|_| Exception::SyscallFailure)
    }

    fn print_uint(&self) -> Result<(), Exception> {
        let x: u32 = self.registers.read_u32_from_cpu(register::A0)?;
        print!("{x}");
        // We need to flush stdout because the automatic flushing is insufficient in most scenarios.
        stdout().flush().map_err(|_| Exception::SyscallFailure)
    }
}
