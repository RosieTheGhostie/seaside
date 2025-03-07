use crate::Editable;
use cursive::Cursive;
use seaside_config::register_defaults::Registers;

impl Editable<'g'> for Registers<32> {
    fn menu(siv: &mut Cursive) {}
}

impl Editable<'0'> for Registers<4> {
    fn menu(siv: &mut Cursive) {}
}

impl Editable<'1'> for Registers<32> {
    fn menu(siv: &mut Cursive) {}
}
