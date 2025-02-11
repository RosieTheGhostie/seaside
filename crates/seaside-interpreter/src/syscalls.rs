use bitflags::bitflags;
use seaside_config::features::syscalls;
use seaside_constants::service_codes;

bitflags! {
    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
