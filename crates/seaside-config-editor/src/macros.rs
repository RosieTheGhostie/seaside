#[macro_export]
macro_rules! checklist {
    {#![object($object:ident), groups($groups:ident)]} => {
        cursive::views::ListView::new()
    };
    {
        #![object($object:ident), groups($groups:ident)]
        [$index:literal] $field:ident as $label:literal;
        $($rest:tt)*
    } => {{
        let mut list = cursive::views::ListView::new();
        $crate::checklist! {
            list <- #![object($object), groups($groups)]
            [$index] $field as $label;
            $($rest)*
        };
        list
    }};

    {$list:ident <- #![object($object:ident), groups($groups:ident)]} => {};
    {
        $list:ident <- #![object($object:ident), groups($groups:ident)]
        [$index:literal] $field:ident as $label:literal;
        $($rest:tt)*
    } => {
        $list.add_child(
            $label,
            <_ as cursive::With>::with_if(
                cursive::views::LinearLayout::horizontal()
                    .child(<_ as cursive::view::Resizable>::fixed_width(
                        $groups[$index].button(true, "Yes"),
                        10,
                    ))
                    .child(
                        <_ as cursive::view::Resizable>::fixed_width(
                            <_ as cursive::With>::with_if(
                                $groups[$index].button(false, "No"),
                                !$object.$field,
                                |button| {
                                    button.select();
                                },
                            ),
                            10
                        ),
                    ),
                $object.$field,
                |layout| {
                    layout.set_focus_index(0).unwrap();
                },
            ),
        );
        $crate::checklist! {
            $list <- #![object($object), groups($groups)]
            $($rest)*
        };
    };
}
