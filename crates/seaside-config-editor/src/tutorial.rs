use cursive::{
    theme::Theme,
    traits::*,
    views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView},
    Cursive, CursiveRunnable,
};

pub fn new_editor() -> CursiveRunnable {
    let mut siv = cursive::default();
    siv.set_theme(Theme::terminal_default());

    // --- Callbacks ---
    //siv.add_global_callback('q', |s| s.quit());

    // --- Elements ---
    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add New", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    // --- Layers ---
    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("Select a Profile"),
    );

    siv
}

fn add_name(siv: &mut Cursive) {
    fn ok(siv: &mut Cursive, name: &str) {
        siv.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        siv.pop_layer();
    }

    siv.add_layer(
        Dialog::around(EditView::new().with_name("name").fixed_width(10))
            .title("Enter a New Name")
            .button("Ok", |s| {
                let name = s
                    .call_on_name("name", |view: &mut EditView| view.get_content())
                    .unwrap();
                ok(s, &name);
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

fn delete_name(siv: &mut Cursive) {
    let mut select = siv.find_name::<SelectView<String>>("select").unwrap();
    if let Some(focus) = select.selected_id() {
        select.remove_item(focus);
    } else {
        siv.add_layer(Dialog::info("No name to remove"));
    }
}

fn on_submit(siv: &mut Cursive, name: &str) {
    siv.pop_layer();
    siv.add_layer(
        Dialog::text(format!("Name: {name}\nAwesome: yes"))
            .title(format!("{name}'s Info"))
            .button("Quit", Cursive::quit),
    );
}
