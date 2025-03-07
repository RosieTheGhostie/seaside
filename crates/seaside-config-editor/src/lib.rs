mod editable;
mod macros;
mod menus;
mod seaside_version;

pub(crate) use editable::{Config, Editable};
pub(crate) use seaside_version::SeasideVersion;

use anyhow::Result;
use cursive::{event::Key, menu::Tree, theme::Theme, Cursive, CursiveRunnable};
use menus::{home, save, save_as};
use seaside_config::{
    features::Syscalls, register_defaults::Registers, Config as SeasideConfig, Features, MemoryMap,
};

pub fn new_editor(config: Option<SeasideConfig>) -> Result<CursiveRunnable> {
    let mut siv = cursive::default();
    siv.set_theme(Theme::terminal_default());
    let editable_config: Config = match config {
        Some(config) => config.try_into()?,
        None => Config::default(),
    };
    siv.set_user_data(editable_config);

    // --- Callbacks ---
    siv.add_global_callback(Key::Esc, Cursive::select_menubar);

    // --- Elements ---

    // --- Menu Bar ---
    siv.menubar()
        .add_subtree(
            "File",
            Tree::new()
                .leaf("Save", save)
                .leaf("Save as", save_as)
                .delimiter()
                .leaf("Quit", Cursive::quit),
        )
        .add_leaf("General", Config::menu)
        .add_subtree(
            "Features",
            Tree::new()
                .leaf("General", Features::menu)
                .leaf("Syscalls", Syscalls::menu),
        )
        .add_leaf("Memory Map", MemoryMap::menu)
        .add_subtree(
            "Register Defaults",
            Tree::new()
                .leaf("General Purpose", <Registers<32> as Editable<'g'>>::menu)
                .leaf("Cop. 0", <Registers<4> as Editable<'0'>>::menu)
                .leaf("Cop. 1", <Registers<32> as Editable<'1'>>::menu),
        );
    siv.set_autohide_menu(false);

    // --- Layers ---
    home(&mut siv);

    Ok(siv)
}
