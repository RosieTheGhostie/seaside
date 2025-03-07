use crate::Editable;
use cursive::Cursive;
use seaside_config::MemoryMap;

impl Editable<' '> for MemoryMap {
    fn menu(siv: &mut Cursive) {}
}
