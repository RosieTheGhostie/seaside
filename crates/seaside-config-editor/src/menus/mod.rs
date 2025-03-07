pub mod config;
pub mod features;
pub mod memory_map;
pub mod register_defaults;

use cursive::{views::TextView, Cursive};

const HOME_TEXT: &str = r"Configuration Editor
---
Press <Esc> to select the menubar
Use the arrow keys to navigate
Interact with <Enter>";

pub fn home(siv: &mut Cursive) {
    siv.pop_layer();
    siv.add_layer(TextView::new(HOME_TEXT).center());
}

pub fn save(siv: &mut Cursive) {}

pub fn save_as(siv: &mut Cursive) {}
