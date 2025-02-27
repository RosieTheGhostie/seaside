macro_rules! properties_internal {
    ([$id:literal]) => {
        pub const ID: u8 = $id;
    };
    ($(@prefixed)? [$id:literal, level: 0]) => {
        $crate::properties::macros::properties_internal!([$id]);
        pub const PREFIX: u32 = (ID as u32) << 24;
    };
    ($(@prefixed)? [$id:literal, level: $level:literal]) => {
        $crate::properties::macros::properties_internal!([$id]);
        pub const PREFIX: u32 = super::PREFIX | ((ID as u32) << (24 - ($level << 3)));
    };

    ($(@prefixed)?) => {};

    ([$id:literal] $property:ident; $($rest:tt)*) => {
        pub const $property: u8 = $id;
        $crate::properties::macros::properties_internal!($($rest)*);
    };
    ([$id:literal, level: 0] $name:ident { $($group:tt)* } $($rest:tt)*) => {
        pub mod $name {
            $crate::properties::macros::properties_internal!([$id, level: 0]);
            $crate::properties::macros::properties_internal!($($group)*);
        }
        $crate::properties::macros::properties_internal!($($rest)*);
    };
    ([$id:literal, level: $level:literal] $name:ident { $($group:tt)* } $($rest:tt)*) => {
        pub mod $name {
            $crate::properties::macros::properties_internal!([$id, level: $level]);
            $crate::properties::macros::properties_internal!($($group)*);
        }
        $crate::properties::macros::properties_internal!($($rest)*);
    };

    (@prefixed [$id:literal] $property:ident; $($rest:tt)*) => {
        pub const $property: u32 = PREFIX | $id;
        $crate::properties::macros::properties_internal!(@prefixed $($rest)*);
    };
    (@prefixed [$id:literal, level: 0] $name:ident { $($group:tt)* } $($rest:tt)*) => {
        pub mod $name {
            $crate::properties::macros::properties_internal!([$id, level: 0]);
            $crate::properties::macros::properties_internal!(@prefixed $($group)*);
        }
        $crate::properties::macros::properties_internal!(@prefixed $($rest)*);
    };
    (@prefixed [$id:literal, level: $level:literal] $name:ident { $($group:tt)* } $($rest:tt)*) => {
        pub mod $name {
            $crate::properties::macros::properties_internal!([$id, level: $level]);
            $crate::properties::macros::properties_internal!(@prefixed $($group)*);
        }
        $crate::properties::macros::properties_internal!(@prefixed $($rest)*);
    };
}
pub(super) use properties_internal;

macro_rules! properties {
    ([$id:literal, level: 0] { $($group:tt)* }) => {
        $crate::properties::macros::properties_internal!([$id]);
        $crate::properties::macros::properties_internal!($($group)*);

        pub mod prefixed {
            $crate::properties::macros::properties_internal!([$id, level: 0]);
            $crate::properties::macros::properties_internal!(@prefixed $($group)*);
        }
    };
    ([$id:literal, level: $level:literal] { $($group:tt)* }) => {
        $crate::properties::macros::properties_internal!([$id]);
        $crate::properties::macros::properties_internal!($($group)*);

        pub mod prefixed {
            $crate::properties::macros::properties_internal!([$id, level: $level]);
            $crate::properties::macros::properties_internal!(@prefixed $($group)*);
        }
    };

    ([$id:literal, level: 0] $name:ident { $($group:tt)* }) => {
        pub mod $name {
            $crate::properties::macros::properties!([$id, level: 0] { $($group)* });
        }
    };
    ([$id:literal, level: $level:literal] $name:ident { $($group:tt)* }) => {
        pub mod $name {
            $crate::properties::macros::properties!([$id, level: $level] { $($group)* });
        }
    };
}
pub(super) use properties;

#[macro_export]
macro_rules! syscall_id {
    ($group:ident[$property:ident]) => {
        (($crate::properties::features::syscalls::$group::ID as u16) << 8)
            | $crate::properties::features::syscalls::$group::$property as u16
    };
}
