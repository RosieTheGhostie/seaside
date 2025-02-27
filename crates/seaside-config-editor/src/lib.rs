mod editable_config;
mod menus;
mod seaside_version;

pub(crate) use seaside_version::SeasideVersion;

use cursive::{event::Key, menu::Tree, theme::Theme, Cursive, CursiveRunnable};
use editable_config::EditableConfig;

pub fn new_editor() -> CursiveRunnable {
    let mut siv = cursive::default();
    siv.set_theme(Theme::terminal_default());
    siv.set_user_data(EditableConfig::default());

    // --- Callbacks ---
    siv.add_global_callback(Key::Esc, Cursive::select_menubar);

    // --- Elements ---

    // --- Menu Bar ---
    siv.menubar()
        .add_subtree(
            "File",
            Tree::new()
                .leaf("Save", move |_| {})
                .leaf("Save as", move |_| {})
                .delimiter()
                .leaf("Quit", Cursive::quit),
        )
        .add_leaf("General", menus::general)
        .add_subtree(
            "Features",
            Tree::new()
                .leaf("General", menus::features::general)
                .leaf("Assembler", menus::features::assembler)
                .leaf("Syscalls", menus::features::syscalls),
        )
        .add_leaf("Memory Map", menus::memory_map)
        .add_subtree(
            "Register Defaults",
            Tree::new()
                .leaf("CPU", menus::register_defaults::cpu)
                .leaf("FPU", menus::register_defaults::fpu)
                .leaf("Cop. 0", menus::register_defaults::coprocessor_0),
        );
    siv.set_autohide_menu(false);

    // --- Layers ---
    menus::home(&mut siv);

    siv
}
