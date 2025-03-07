macro_rules! properties {
    {} => {};
    {
        #[$_root_id:literal: _] {
            $(
                $(#[doc $($property_doc:tt)*])*
                #[name = $property_name:literal]
                [$property_id:literal] $property:ident,
            )*
        }

        $(
            $(#[doc $($group_doc:tt)*])*
            $(#[extra_fns($($fn:ident),*)])?
            #[$group_id:literal: $group:ident $(, name = $group_name:literal)?] {
                $($contents:tt)*
            }
        )*

        $(
            ---
            $(
                $(#[doc $($item_doc:tt)*])*
                $item:item
            )*
        )?
    } => {
        $(
            $(#[doc $($property_doc)*])*
            pub const $property: u8 = $property_id;
        )*

        #[inline]
        pub const fn all_ids() -> &'static [u8] {
            &[$($property_id),*]
        }

        #[inline]
        pub const fn all_names() -> &'static [&'static str] {
            &[$($property_name),*]
        }

        #[inline]
        pub const fn all_names_and_ids() -> &'static [(&'static str, u8)] {
            &[$(($property_name, $property_id),)*]
        }

        pub const fn name_of(property: u8) -> &'static str {
            match property {
                $($property => $property_name,)*
                _ => panic!("unknown property"),
            }
        }

        $(
            $(#[doc $($group_doc)*])*
            pub mod $group {
                const LEVEL: u8 = 0;
                $(pub const NAME: &str = $group_name;)?
                pub const ID: u8 = $group_id;
                pub const PREFIX: u32 = (ID as u32) << 24;

                $($crate::properties::macros::_extra_fns!($($fn),*);)?

                $crate::properties::macros::properties! {
                    @internal
                    $($contents)*
                }
            }
        )*

        $($(
            $(#[doc $($item_doc)*])*
            $item
        )*)?
    };
    {
        @internal
        #[$_root_id:literal: _] {
            $(
                $(#[doc $($property_doc:tt)*])*
                #[name = $property_name:literal]
                [$property_id:literal] $property:ident,
            )*
        }

        $(
            $(#[doc $($group_doc:tt)*])*
            $(#[extra_fns($($fn:ident),*)])?
            #[$group_id:literal: $group:ident $(, name = $group_name:literal)?] {
                $($contents:tt)*
            }
        )*

        $(
            ---
            $(
                $(#[doc $($item_doc:tt)*])*
                $item:item
            )*
        )?
    } => {
        $(
            $(#[doc $($property_doc)*])*
            pub const $property: u8 = $property_id;
        )*

        #[inline]
        pub const fn all_ids() -> &'static [u8] {
            &[$($property_id),*]
        }

        #[inline]
        pub const fn all_names() -> &'static [&'static str] {
            &[$($property_name),*]
        }

        #[inline]
        pub const fn all_names_and_ids() -> &'static [(&'static str, u8)] {
            &[$(($property_name, $property_id),)*]
        }

        pub const fn name_of(property: u8) -> &'static str {
            match property {
                $($property => $property_name,)*
                _ => panic!("unknown property"),
            }
        }

        $(
            $(#[doc $($group_doc)*])*
            pub mod $group {
                const LEVEL: u8 = super::LEVEL + 1;
                $(pub const NAME: &str = $group_name;)?
                pub const ID: u8 = $group_id;
                pub const PREFIX: u32 = super::PREFIX | (ID as u32) << (24 - (LEVEL << 3));

                $($crate::properties::macros::_extra_fns!($($fn),*);)?

                $crate::properties::macros::properties! {
                    @internal
                    $($contents)*
                }
            }
        )*

        $($(
            $(#[doc $($item_doc)*])*
            $item
        )*)?
    };
    {
        @internal
        $(
            $(#[doc $($property_doc:tt)*])*
            #[name = $property_name:literal]
            [$property_id:literal] $property:ident,
        )*

        $(
            ---
            $(
                $(#[doc $($item_doc:tt)*])*
                $item:item
            )*
        )?
    } => {
        $(
            $(#[doc $($property_doc)*])*
            pub const $property: u8 = $property_id;
        )*

        #[inline]
        pub const fn all_ids() -> &'static [u8] {
            &[$($property_id),*]
        }

        #[inline]
        pub const fn all_names() -> &'static [&'static str] {
            &[$($property_name),*]
        }

        #[inline]
        pub const fn all_names_and_ids() -> &'static [(&'static str, u8)] {
            &[$(($property_name, $property_id),)*]
        }

        pub const fn name_of(property: u8) -> &'static str {
            match property {
                $($property => $property_name,)*
                _ => panic!("unknown property"),
            }
        }

        $($(
            $(#[doc $($item_doc)*])*
            $item
        )*)?
    };
}
pub(super) use properties;

macro_rules! _extra_fns {
    () => {};
    (service_id $(, $fn:ident)*) => {
        pub const fn service_id(property: u8) -> u16 {
            ((super::ID as u16) << 8) | property as u16
        }

        $crate::properties::macros::_extra_fns!($($fn),*);
    };
    (full_name_of $(, $fn:ident)*) => {
        pub fn full_name_of(property: u8) -> String {
            format!("{}/{}", NAME, name_of(property))
        }

        $crate::properties::macros::_extra_fns!($($fn),*);
    };
    (all_full_names_and_service_ids $(, $fn:ident)*) => {
        pub fn all_full_names_and_service_ids() -> impl Iterator<Item = (String, u16)> {
            all_ids().iter().map(|id| (full_name_of(*id), service_id(*id)))
        }

        $crate::properties::macros::_extra_fns!($($fn),*);
    };
}
pub(super) use _extra_fns;

#[macro_export]
macro_rules! service_id {
    ($group:ident[$property:ident]) => {
        (($crate::properties::features::syscalls::$group::ID as u16) << 8)
            | $crate::properties::features::syscalls::$group::$property as u16
    };
}

#[macro_export]
macro_rules! service_name {
    ($group:ident[$property:ident]) => {
        format!(
            "{}/{}",
            stringify!($group),
            stringify!($property).to_lowercase()
        )
    };
}

#[macro_export]
macro_rules! property_name_id {
    (_[$property:ident]) => {{
        let property: u8 = $crate::properties::$property;
        (
            $crate::properties::name_of(property),
            property,
        )
    }};
    ($a:ident $(:: $others:ident)* [$property:ident]) => {{
        let property: u8 = $crate::properties::$a $(:: $others)*::$property;
        (
            $crate::properties::$a $(:: $others)*::name_of(property),
            property,
        )
    }};
}

#[macro_export]
macro_rules! service_name_id {
    ($group:ident[$property:ident]) => {
        (
            $crate::service_name!($group[$property]),
            $crate::service_id!($group[$property]),
        )
    };
}

#[macro_export]
macro_rules! prefix {
    ($a:ident $(:: $others:ident)*) => {
        $crate::properties::$a $(:: $others)*::PREFIX
    };
}

#[macro_export]
macro_rules! prefixed {
    (_[$property:ident]) => {
        $crate::properties::$property as u32
    };
    ($a:ident $(:: $others:ident)* [$property:ident]) => {
        $crate::properties::$a $(:: $others)*::PREFIX
        | $crate::properties::$a $(:: $others)*::$property as u32
    };
}
