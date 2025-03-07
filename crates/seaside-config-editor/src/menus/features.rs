use crate::{checklist, Config, Editable};
use cursive::{
    align::HAlign,
    view::{Nameable, Resizable, Scrollable},
    views::{
        Button, Dialog, DummyView, EditView, LinearLayout, ListView, RadioGroup, SelectView,
        TextView,
    },
    Cursive,
};
use seaside_config::{features::Syscalls, properties, Features};

impl Editable<' '> for Features {
    fn menu(siv: &mut Cursive) {
        let mut groups: [RadioGroup<bool>; 6] = core::array::from_fn(|_| RadioGroup::new());
        let data = siv
            .with_user_data(|user_data: &mut Config| user_data.features.clone())
            .unwrap();
        siv.add_layer(
            Dialog::new()
                .title("Features (General)")
                .content(checklist! {
                    #![object(data), groups(groups)]
                    [0] kernel_space_accessible as "kernel space accessible: ";
                    [1] self_modifying_code as "self-modifying code: ";
                    [2] delay_slot as "delay slot: ";
                    [3] freeable_heap_allocations as "freeable heap allocations: ";
                    [4] show_crash_handler as "show crash handler: ";
                    [5] pseudo_instructions as "pseudo-instructions: ";
                })
                .button("Done", move |s| {
                    s.with_user_data(|user_data: &mut Config| {
                        let features = &mut user_data.features;
                        features.kernel_space_accessible = *groups[0].selection();
                        features.self_modifying_code = *groups[1].selection();
                        features.delay_slot = *groups[2].selection();
                        features.freeable_heap_allocations = *groups[3].selection();
                        features.show_crash_handler = *groups[4].selection();
                        features.pseudo_instructions = *groups[5].selection();
                    })
                    .unwrap();
                    s.pop_layer();
                }),
        );
    }
}

impl Editable<' '> for Syscalls {
    fn menu(siv: &mut Cursive) {
        // let data = siv
        //     .with_user_data(|user_data: &mut Config| user_data.features.syscalls.clone())
        //     .unwrap();

        let mapped = Dialog::around(
            ListView::new()
                .with_name("mapped")
                .scrollable()
                .fixed_width(48),
        )
        .title("Mapped");

        let available = Dialog::around(
            SelectView::<u16>::new()
                .with_all(properties::features::syscalls::all_full_names_and_service_ids())
                .with_name("available_services")
                .scrollable()
                .fixed_width(32),
        )
        .title("Available");

        let buttons = LinearLayout::vertical()
            .child(Button::new("Add", add_service))
            .child(Button::new("Edit", edit_service_mapping))
            .child(Button::new("Remove", remove_service))
            .child(DummyView)
            .child(Button::new("Cancel", move |s| {
                s.pop_layer();
            }))
            .child(Button::new("Done", move |s| {
                // todo!("write the changes");
                s.pop_layer();
            }))
            .fixed_width(10);

        siv.add_layer(
            Dialog::around(
                LinearLayout::horizontal()
                    .child(mapped)
                    .child(available)
                    .child(buttons),
            )
            .title("Features (Syscalls)"),
        );
    }
}

fn add_service(siv: &mut Cursive) {
    fn on_success(siv: &mut Cursive, service: (&str, u16), code: &str) {
        let mut available_services = siv
            .find_name::<SelectView<u16>>("available_services")
            .unwrap();
        let available_focus = available_services.selected_id().unwrap();
        available_services.remove_item(available_focus);

        siv.find_name::<ListView>("mapped")
            .unwrap()
            .add_child(service.0, TextView::new(code).h_align(HAlign::Right));
        siv.pop_layer();
    }

    let available_services = siv
        .find_name::<SelectView<u16>>("available_services")
        .unwrap();
    if let Some(focus) = available_services.selected_id() {
        let (name, &id) = available_services.get_item(focus).unwrap();
        let name = name.to_owned();
        siv.add_layer(
            Dialog::around(EditView::new().with_name("service_code").fixed_width(8))
                .title(format!("Set Service Code for {name:#}"))
                .button("Add", move |s| {
                    let code = s
                        .call_on_name("service_code", |view: &mut EditView| view.get_content())
                        .unwrap();
                    if code.parse::<u32>().is_ok() {
                        on_success(s, (&name, id), &code);
                    } else {
                        s.add_layer(Dialog::info("invalid service code :("));
                    }
                })
                .button("Cancel", |s| {
                    s.pop_layer();
                }),
        );
    } else {
        siv.add_layer(Dialog::info("no service to add"));
    }
}

fn edit_service_mapping(siv: &mut Cursive) {}

fn remove_service(siv: &mut Cursive) {
    let mut mapped = siv.find_name::<ListView>("mapped").unwrap();
    if let Some(focus) = (!mapped.is_empty()).then(|| mapped.focus()) {
        if let cursive::views::ListChild::Row(label, _) = mapped.get_row(focus) {
            // todo!("recover the service id");
            siv.find_name::<SelectView<u16>>("available_services")
                .unwrap()
                .add_item(label, 0);
        }
        mapped.remove_child(focus);
    } else {
        siv.add_layer(Dialog::info("no service to remove"));
    }
}
