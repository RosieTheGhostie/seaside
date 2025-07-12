macro_rules! special {
    ($variant:ident) => {
        $crate::instruction::operator::Operator::Special {
            r#fn: seaside_constants::fn_codes::SpecialFn::$variant,
            condition: false,
        }
    };
    ($variant:ident, $condition:literal) => {
        $crate::instruction::operator::Operator::Special {
            r#fn: seaside_constants::fn_codes::SpecialFn::$variant,
            condition: $condition,
        }
    };
    ($variant:ident, condition: _) => {
        $crate::instruction::operator::Operator::Special {
            r#fn: seaside_constants::fn_codes::SpecialFn::$variant,
            condition: _,
        }
    };
    ($variant:ident, condition: $condition:ident) => {
        $crate::instruction::operator::Operator::Special {
            r#fn: seaside_constants::fn_codes::SpecialFn::$variant,
            condition: $condition,
        }
    };
    [$variant_0:ident, $variant_1:ident $(,)?] => {
        $crate::instruction::macros::special!($variant_0, condition: _)
        | $crate::instruction::macros::special!($variant_1, condition: _)
    };
    [$variant_0:ident, $variant_1:ident, $($variants:ident),+ $(,)?] => {
        $crate::instruction::macros::special!($variant_0, condition: _)
        | $crate::instruction::macros::special!($variant_1, $($variants),+)
    };
}
pub(crate) use special;

macro_rules! register_immediate {
    ($variant:ident) => {
        $crate::instruction::operator::Operator::RegisterImmediate(
            seaside_constants::fn_codes::RegisterImmediateFn::$variant,
        )
    };
    [$variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::instruction::macros::register_immediate!($variant)
        | $crate::instruction::macros::register_immediate!($($variants),+)
    };
}
pub(crate) use register_immediate;

macro_rules! coprocessor_0 {
    ($variant:ident) => {
        $crate::instruction::operator::Operator::Coprocessor0(
            seaside_constants::fn_codes::Coprocessor0Fn::$variant,
        )
    };
    [$variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::instruction::macros::coprocessor_0!($variant)
        | $crate::instruction::macros::coprocessor_0!($($variants),+)
    };
}
pub(crate) use coprocessor_0;

macro_rules! coprocessor_1 {
    (<$fmt:ident> $variant:ident) => {
        $crate::instruction::operator::Operator::Coprocessor1 {
            r#fn: seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            fmt: seaside_constants::number_fmt::NumberFormat::$fmt,
            condition: false,
        }
    };
    ({$fmt:ident} $variant:ident) => {
        $crate::instruction::operator::Operator::Coprocessor1 {
            r#fn: seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            fmt: $fmt,
            condition: false,
        }
    };
    (<$fmt_0:ident | $fmt_1:ident> $variant:ident) => {
        $crate::instruction::macros::coprocessor_1!(<$fmt_0> $variant)
        | $crate::instruction::macros::coprocessor_1!(<$fmt_1> $variant)
    };
    (<$fmt:ident> $variant:ident, $condition:literal) => {
        $crate::instruction::operator::Operator::Coprocessor1 {
            r#fn: seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            fmt: seaside_constants::number_fmt::NumberFormat::$fmt,
            condition: $condition,
        }
    };
    ({$fmt:ident} $variant:ident, $condition:literal) => {
        $crate::instruction::operator::Operator::Coprocessor1 {
            r#fn: seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            fmt: $fmt,
            condition: $condition,
        }
    };
    (<$fmt:ident> $variant:ident, condition: _) => {
        $crate::instruction::operator::Operator::Coprocessor1 {
            r#fn: seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            fmt: seaside_constants::number_fmt::NumberFormat::$fmt,
            condition: _,
        }
    };
    ({$fmt:ident} $variant:ident, condition: _) => {
        $crate::instruction::operator::Operator::Coprocessor1 {
            r#fn: seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            fmt: $fmt,
            condition: _,
        }
    };
    (<$fmt:ident> $variant:ident, condition: $condition:ident) => {
        $crate::instruction::operator::Operator::Coprocessor1 {
            r#fn: seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            fmt: seaside_constants::number_fmt::NumberFormat::$fmt,
            condition: $condition,
        }
    };
    ({$fmt:ident} $variant:ident, condition: $condition:ident) => {
        $crate::instruction::operator::Operator::Coprocessor1 {
            r#fn: seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            fmt: $fmt,
            condition: $condition,
        }
    };
    (<$fmt_0:ident | $fmt_1:ident> $variant:ident, $condition:literal) => {
        $crate::instruction::macros::coprocessor_1!(<$fmt_0> $variant, $condition)
        | $crate::instruction::macros::coprocessor_1!(<$fmt_1> $variant, $condition)
    };
    (<$fmt_0:ident | $fmt_1:ident> $variant:ident, condition: $condition:ident) => {
        $crate::instruction::macros::coprocessor_1!(<$fmt_0> $variant, condition: $condition)
        | $crate::instruction::macros::coprocessor_1!(<$fmt_1> $variant, condition: $condition)
    };
    (<$fmt_0:ident | $fmt_1:ident> $variant:ident, condition: _) => {
        $crate::instruction::macros::coprocessor_1!(<$fmt_0> $variant, condition: _)
        | $crate::instruction::macros::coprocessor_1!(<$fmt_1> $variant, condition: _)
    };
    [<$fmt:ident> $variant_0:ident, $variant_1:ident $(,)?] => {
        $crate::instruction::macros::coprocessor_1!(<$fmt> $variant_0, condition: _)
        | $crate::instruction::macros::coprocessor_1!(<$fmt> $variant_1, condition: _)
    };
    [<$fmt:ident> $variant_0:ident, $variant_1:ident, $($variants:ident),+ $(,)?] => {
        $crate::instruction::macros::coprocessor_1!(<$fmt> $variant_0, condition: _)
        | $crate::instruction::macros::coprocessor_1!(<$fmt> $variant_1, $($variants),+)
    };
    [{$fmt:ident} $variant_0:ident, $variant_1:ident $(,)?] => {
        $crate::instruction::macros::coprocessor_1!({$fmt} $variant_0, condition: _)
        | $crate::instruction::macros::coprocessor_1!({$fmt} $variant_1, condition: _)
    };
    [{$fmt:ident} $variant_0:ident, $variant_1:ident, $($variants:ident),+ $(,)?] => {
        $crate::instruction::macros::coprocessor_1!({$fmt} $variant_0, condition: _)
        | $crate::instruction::macros::coprocessor_1!({$fmt} $variant_1, $($variants),+)
    };
    [<$fmt_0:ident | $fmt_1:ident> $variant_0:ident, $variant_1:ident $(,)?] => {
        $crate::instruction::macros::coprocessor_1!(<$fmt_0 | $fmt_1> $variant_0, condition: _)
        | $crate::instruction::macros::coprocessor_1!(<$fmt_0 | $fmt_1> $variant_1, condition: _)
    };
    [<$fmt_0:ident | $fmt_1:ident> $variant_0:ident, $variant_1:ident, $($variants:ident),+ $(,)?] => {
        $crate::instruction::macros::coprocessor_1!(<$fmt_0 | $fmt_1> $variant_0, condition: _)
        | $crate::instruction::macros::coprocessor_1!(<$fmt_0 | $fmt_1> $variant_1, $($variants),+)
    };
}
pub(crate) use coprocessor_1;

macro_rules! coprocessor_1_register_immediate {
    ($variant:ident) => {
        $crate::instruction::operator::Operator::Coprocessor1RegisterImmediate {
            r#fn: seaside_constants::fn_codes::Coprocessor1RegisterImmediateFn::$variant,
            condition: false,
        }
    };
    ($variant:ident, $condition:literal) => {
        $crate::instruction::operator::Operator::Coprocessor1RegisterImmediate {
            r#fn: seaside_constants::fn_codes::Coprocessor1RegisterImmediateFn::$variant,
            condition: $condition,
        }
    };
    ($variant:ident, condition: _) => {
        $crate::instruction::operator::Operator::Coprocessor1RegisterImmediate {
            r#fn: seaside_constants::fn_codes::Coprocessor1RegisterImmediateFn::$variant,
            condition: _,
        }
    };
    ($variant:ident, condition: $condition:ident) => {
        $crate::instruction::operator::Operator::Coprocessor1RegisterImmediate {
            r#fn: seaside_constants::fn_codes::Coprocessor1RegisterImmediateFn::$variant,
            condition: $condition,
        }
    };
    [$variant_0:ident, $variant_1:ident $(,)?] => {
        $crate::instruction::macros::coprocessor_1_register_immediate!($variant_0, condition: _)
        | $crate::instruction::macros::coprocessor_1_register_immediate!($variant_1, condition: _)
    };
    [$variant_0:ident, $variant_1:ident, $($variants:ident),+ $(,)?] => {
        $crate::instruction::macros::coprocessor_1_register_immediate!($variant_0, condition: _)
        | $crate::instruction::macros::coprocessor_1_register_immediate!($variant_1, $($variants),+)
    };
}
pub(crate) use coprocessor_1_register_immediate;

macro_rules! special_2 {
    ($variant:ident) => {
        $crate::instruction::operator::Operator::Special2(
            seaside_constants::fn_codes::Special2Fn::$variant,
        )
    };
    [$variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::instruction::macros::special_2!($variant)
        | $crate::instruction::macros::special_2!($($variants),+)
    };
}
pub(crate) use special_2;
