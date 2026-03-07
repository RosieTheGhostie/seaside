use std::collections::HashMap;

use anyhow::Result;
use seaside_core::{
    ServiceCode, Services,
    consts::services::{
        Service,
        mars::{self, Mars},
        spim::{self, Spim},
    },
};

use crate::{Exception, Interpreter, InterpreterState};

pub type ServiceFn = for<'a> fn(&'a mut InterpreterState) -> Result<(), Exception>;

impl Interpreter {
    pub(crate) fn init_services(
        services: &Services,
        flags: seaside_executable::Flags,
    ) -> HashMap<ServiceCode, ServiceFn> {
        services
            .iter()
            .map(|(&code, &service)| (code, InterpreterState::get_service_fn(service, flags)))
            .collect()
    }
}

impl InterpreterState {
    pub fn get_service_fn(
        service: Service,
        flags: seaside_executable::Flags,
    ) -> fn(&mut InterpreterState) -> Result<(), Exception> {
        match service {
            Service::Spim(Spim::Print(spim::Print::Int)) => InterpreterState::print_int,
            Service::Mars(Mars::Print(mars::Print::Uint)) => InterpreterState::print_uint,
            Service::Mars(Mars::Print(mars::Print::Bin)) => InterpreterState::print_bin,
            Service::Mars(Mars::Print(mars::Print::Hex)) => InterpreterState::print_hex,
            Service::Spim(Spim::Print(spim::Print::Float)) => InterpreterState::print_float,
            Service::Spim(Spim::Print(spim::Print::Double)) => InterpreterState::print_double,
            Service::Spim(Spim::Print(spim::Print::Char)) => InterpreterState::print_char,
            Service::Spim(Spim::Print(spim::Print::String)) => InterpreterState::print_string,
            Service::Spim(Spim::Read(spim::Read::Int)) => InterpreterState::read_int,
            Service::Spim(Spim::Read(spim::Read::Float)) => InterpreterState::read_float,
            Service::Spim(Spim::Read(spim::Read::Double)) => InterpreterState::read_double,
            Service::Spim(Spim::Read(spim::Read::Char)) => InterpreterState::read_char,
            Service::Spim(Spim::Read(spim::Read::String)) => InterpreterState::read_string,
            Service::Spim(Spim::File(spim::File::Open)) => InterpreterState::open_file,
            Service::Spim(Spim::File(spim::File::Read)) => InterpreterState::read_file,
            Service::Spim(Spim::File(spim::File::Write)) => InterpreterState::write_file,
            Service::Spim(Spim::File(spim::File::Close)) => InterpreterState::close_file,
            Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Confirm))) => {
                InterpreterState::confirm_dialog
            }
            Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Int))) => {
                InterpreterState::input_dialog_int
            }
            Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Float))) => {
                InterpreterState::input_dialog_float
            }
            Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::Double))) => {
                InterpreterState::input_dialog_double
            }
            Service::Mars(Mars::Dialog(mars::Dialog::Input(mars::InputDialog::String))) => {
                InterpreterState::input_dialog_string
            }
            Service::Mars(Mars::Dialog(mars::Dialog::Message(mars::MessageDialog::General))) => {
                InterpreterState::message_dialog
            }
            Service::Mars(Mars::Dialog(mars::Dialog::Message(mars::MessageDialog::Int))) => {
                InterpreterState::message_dialog_int
            }
            Service::Mars(Mars::Dialog(mars::Dialog::Message(mars::MessageDialog::Float))) => {
                InterpreterState::message_dialog_float
            }
            Service::Mars(Mars::Dialog(mars::Dialog::Message(mars::MessageDialog::Double))) => {
                InterpreterState::message_dialog_double
            }
            Service::Mars(Mars::Dialog(mars::Dialog::Message(mars::MessageDialog::String))) => {
                InterpreterState::message_dialog_string
            }
            Service::Spim(Spim::System(spim::System::Sbrk)) => {
                if flags.freeable_heap_allocations() {
                    |state: &mut InterpreterState| state.sbrk(true)
                } else {
                    |state: &mut InterpreterState| state.sbrk(false)
                }
            }
            Service::Spim(Spim::System(spim::System::Exit)) => InterpreterState::exit,
            Service::Spim(Spim::System(spim::System::Exit2)) => InterpreterState::exit_2,
            Service::Mars(Mars::System(mars::System::Time)) => InterpreterState::time,
            Service::Mars(Mars::System(mars::System::Sleep)) => InterpreterState::sleep,
            Service::Mars(Mars::System(mars::System::MidiOut)) => InterpreterState::midi_out,
            Service::Mars(Mars::System(mars::System::MidiOutSync)) => {
                InterpreterState::midi_out_sync
            }
            Service::Mars(Mars::Random(mars::Random::SetSeed)) => InterpreterState::set_seed,
            Service::Mars(Mars::Random(mars::Random::RandInt)) => InterpreterState::rand_int,
            Service::Mars(Mars::Random(mars::Random::RandIntRange)) => {
                InterpreterState::rand_int_range
            }
            Service::Mars(Mars::Random(mars::Random::RandFloat)) => InterpreterState::rand_float,
            Service::Mars(Mars::Random(mars::Random::RandDouble)) => InterpreterState::rand_double,
        }
    }
}
