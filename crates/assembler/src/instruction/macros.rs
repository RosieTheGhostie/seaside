macro_rules! special {
    [{_} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Special(
            ::seaside_core::instruction::special::Fields {
                r#fn: $(::seaside_core::consts::codes::SpecialFn::$variant)|+,
                ..
            }
        )
    };
    [{$fields:ident} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Special(
            $fields @ ::seaside_core::instruction::special::Fields {
                r#fn: $(::seaside_core::consts::codes::SpecialFn::$variant)|+,
                ..
            }
        )
    };
}
pub(crate) use special;

macro_rules! register_immediate {
    [{_} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::RegisterImmediate(
            ::seaside_core::instruction::register_immediate::Fields {
                r#fn: $(::seaside_core::consts::codes::RegisterImmediateFn::$variant)|+,
                ..
            }
        )
    };
    [{$fields:ident} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::RegisterImmediate(
            $fields @ ::seaside_core::instruction::register_immediate::Fields {
                r#fn: $(::seaside_core::consts::codes::RegisterImmediateFn::$variant)|+,
                ..
            }
        )
    };
}
pub(crate) use register_immediate;

macro_rules! immediate {
    [{_} $($variant:ident),+ $(,)?] => {
        $(::seaside_core::instruction::UnpackedInstruction::$variant(_))|+
    };
    [{$fields:ident} $($variant:ident),+ $(,)?] => {
        $(::seaside_core::instruction::UnpackedInstruction::$variant($fields))|+
    };
}
pub(crate) use immediate;
pub(crate) use immediate as jump;

macro_rules! coprocessor_0 {
    [{_} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor0(
            ::seaside_core::instruction::coprocessor_0::Fields {
                r#fn: $(::seaside_core::consts::codes::Coprocessor0Fn::$variant)|+,
                ..
            }
        )
    };
    [{$fields:ident} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor0(
            $fields @ ::seaside_core::instruction::coprocessor_0::Fields {
                r#fn: $(::seaside_core::consts::codes::Coprocessor0Fn::$variant)|+,
                ..
            }
        )
    };
}
pub(crate) use coprocessor_0;

macro_rules! coprocessor_1 {
    [{_} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor1(
            ::seaside_core::instruction::coprocessor_1::Fields::Normal(
                ::seaside_core::instruction::coprocessor_1::NormalFields {
                    r#fn: $(::seaside_core::consts::codes::Coprocessor1Fn::$variant)|+,
                    ..
                },
            ),
        )
    };
    [{$fields:ident} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor1(
            ::seaside_core::instruction::coprocessor_1::Fields::Normal(
                $fields @ ::seaside_core::instruction::coprocessor_1::NormalFields {
                    r#fn: $(::seaside_core::consts::codes::Coprocessor1Fn::$variant)|+,
                    ..
                },
            ),
        )
    };
}
pub(crate) use coprocessor_1;

macro_rules! coprocessor_1_register_immediate {
    [{_} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor1(
            ::seaside_core::instruction::coprocessor_1::RegisterImmediateFields {
                r#fn: $(::seaside_core::consts::codes::Coprocessor1RegisterImmediateFn::$variant)|+,
                ..
            }
        )
    };
    [{$fields:ident} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor1(
            ::seaside_core::instruction::coprocessor_1::Fields::RegisterImmediate(
                $fields @ ::seaside_core::instruction::coprocessor_1::RegisterImmediateFields {
                    r#fn: $(::seaside_core::consts::codes::Coprocessor1RegisterImmediateFn::$variant)|+,
                    ..
                },
            ),
        )
    };
}
pub(crate) use coprocessor_1_register_immediate;

macro_rules! coprocessor_1x {
    [{_} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor1X(
            ::seaside_core::instruction::coprocessor_1x::Fields {
                r#fn: $(::seaside_core::consts::codes::Coprocessor1XFn::$variant)|+,
                ..
            },
        )
    };
    [{$fields:ident} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor1X(
            $fields @ ::seaside_core::instruction::coprocessor_1x::Fields {
                r#fn: $(::seaside_core::consts::codes::Coprocessor1XFn::$variant)|+,
                ..
            },
        )
    };
}
pub(crate) use coprocessor_1x;

macro_rules! coprocessor_2_register_immediate {
    [{_} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor2(
            ::seaside_core::instruction::coprocessor_2::Fields {
                r#fn: $(::seaside_core::consts::codes::Coprocessor2RegisterImmediateFn::$variant)|+,
                ..
            },
        )
    };
    [{$fields:ident} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Coprocessor2(
            $fields @ ::seaside_core::instruction::coprocessor_2::Fields {
                r#fn: $(::seaside_core::consts::codes::Coprocessor2RegisterImmediateFn::$variant)|+,
                ..
            },
        )
    };
}
pub(crate) use coprocessor_2_register_immediate;

macro_rules! special_2 {
    [{_} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Special2(
            ::seaside_core::instruction::special_2::Fields {
                r#fn: $(::seaside_core::consts::codes::Special2Fn::$variant)|+,
                ..
            }
        )
    };
    [{$fields:ident} $($variant:ident),+ $(,)?] => {
        ::seaside_core::instruction::UnpackedInstruction::Special2(
            $fields @ ::seaside_core::instruction::special_2::Fields {
                r#fn: $(::seaside_core::consts::codes::Special2Fn::$variant)|+,
                ..
            }
        )
    };
}
pub(crate) use special_2;
