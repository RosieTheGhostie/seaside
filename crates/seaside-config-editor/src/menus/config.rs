use crate::{Config, Editable, SeasideVersion};
use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, LinearLayout, ListView, RadioGroup, SelectView},
    Cursive, With,
};
use seaside_int_utils::Endian;
use strum::{EnumCount, IntoEnumIterator};

impl Editable<' '> for Config {
    fn menu(siv: &mut Cursive) {
        let mut endian_group: RadioGroup<Endian> = RadioGroup::new();
        let mut cwd_group: RadioGroup<bool> = RadioGroup::new();
        let data = siv
            .with_user_data(|user_data: &mut Config| user_data.clone())
            .unwrap();
        siv.add_layer(
            Dialog::new()
                .title("General")
                .content(
                    ListView::new()
                        .child("version: ", {
                            let mut view = SelectView::new();
                            for version in SeasideVersion::iter() {
                                view.add_item(version.to_string(), version);
                            }
                            view.selected(SeasideVersion::COUNT - data.version as usize - 1)
                                .popup()
                                .decorators("", "")
                                .with_name("version")
                                .fixed_width(8)
                        })
                        .child(
                            "endian: ",
                            LinearLayout::horizontal()
                                .child(
                                    endian_group
                                        .button(Endian::Little, "Little")
                                        .fixed_width(10),
                                )
                                .child(
                                    endian_group
                                        .button(Endian::Big, "Big")
                                        .with_if(data.endian == Endian::Big, |button| {
                                            button.select();
                                        })
                                        .fixed_width(10),
                                )
                                .with(|layout| {
                                    if data.endian == Endian::Big {
                                        layout.set_focus_index(1).unwrap();
                                    }
                                }),
                        )
                        .child(
                            "project directory is CWD: ",
                            LinearLayout::horizontal()
                                .child(cwd_group.button(true, "Yes").fixed_width(10))
                                .child(
                                    cwd_group
                                        .button(false, "No")
                                        .with_if(!(data.project_directory_is_cwd), |button| {
                                            button.select();
                                        })
                                        .fixed_width(10),
                                )
                                .with(|layout| {
                                    if data.project_directory_is_cwd {
                                        layout.set_focus_index(0).unwrap();
                                    }
                                }),
                        ),
                )
                .button("Done", move |s| {
                    let version = *s
                        .call_on_name("version", |view: &mut SelectView<SeasideVersion>| {
                            view.selection().unwrap()
                        })
                        .unwrap();
                    s.with_user_data(|user_data: &mut Config| {
                        user_data.version = version;
                        user_data.endian = *endian_group.selection();
                        user_data.project_directory_is_cwd = *cwd_group.selection();
                    })
                    .unwrap();
                    s.pop_layer();
                }),
        );
    }
}
