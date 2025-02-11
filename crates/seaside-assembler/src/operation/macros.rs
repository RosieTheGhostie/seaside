macro_rules! special {
    ($variant:ident) => {
        BasicOperator::Special(seaside_constants::fn_codes::SpecialFn::$variant, None)
    };
    ($variant:ident, $condition:literal) => {
        BasicOperator::Special(
            seaside_constants::fn_codes::SpecialFn::$variant,
            Some($condition),
        )
    };
    ($variant:ident, $condition0:literal | $condition1:literal) => {
        $crate::operation::macros::special!($variant, $condition0)
        | $crate::operation::macros::special!($variant, $condition1)
    };
    ($variant:ident, Some($condition:ident)) => {
        BasicOperator::Special(
            seaside_constants::fn_codes::SpecialFn::$variant,
            Some($condition),
        )
    };
    [$variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::operation::macros::special!($variant)
        | $crate::operation::macros::special!($($variants),+)
    };
}
pub(crate) use special;

macro_rules! register_immediate {
    ($variant:ident) => {
        BasicOperator::RegisterImmediate(seaside_constants::fn_codes::RegisterImmediateFn::$variant)
    };
    [$variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::operation::macros::register_immediate!($variant)
        | $crate::operation::macros::register_immediate!($($variants),+)
    };
}
pub(crate) use register_immediate;

macro_rules! coprocessor_0 {
    ($variant:ident) => {
        BasicOperator::Coprocessor0(seaside_constants::fn_codes::Coprocessor0Fn::$variant)
    };
    [$variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::operation::macros::coprocessor_0!($variant)
        | $crate::operation::macros::coprocessor_0!($($variants),+)
    };
}
pub(crate) use coprocessor_0;

macro_rules! coprocessor_1 {
    (<$fmt:ident> $variant:ident) => {
        BasicOperator::Coprocessor1(
            seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            seaside_constants::number_fmt::NumberFormat::$fmt,
            None,
        )
    };
    ({$fmt:ident} $variant:ident) => {
        BasicOperator::Coprocessor1(
            seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            $fmt,
            None,
        )
    };
    (<$fmt0:ident | $fmt1:ident> $variant:ident) => {
        $crate::operation::macros::coprocessor_1!(<$fmt0> $variant)
        | $crate::operation::macros::coprocessor_1!(<$fmt1> $variant)
    };
    (<$fmt:ident> $variant:ident, $condition:literal) => {
        BasicOperator::Coprocessor1(
            seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            seaside_constants::number_fmt::NumberFormat::$fmt,
            Some($condition),
        )
    };
    ({$fmt:ident} $variant:ident, $condition:literal) => {
        BasicOperator::Coprocessor1(
            seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            $fmt,
            Some($condition),
        )
    };
    (<$fmt:ident> $variant:ident, Some($condition:ident)) => {
        BasicOperator::Coprocessor1(
            seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            seaside_constants::number_fmt::NumberFormat::$fmt,
            Some($condition),
        )
    };
    ({$fmt:ident} $variant:ident, Some($condition:ident)) => {
        BasicOperator::Coprocessor1(
            seaside_constants::fn_codes::Coprocessor1Fn::$variant,
            $fmt,
            Some($condition),
        )
    };
    (<$fmt0:ident | $fmt1:ident> $variant:ident, $condition:literal) => {
        $crate::operation::macros::coprocessor_1!(<$fmt0> $variant, $condition)
        | $crate::operation::macros::coprocessor_1!(<$fmt1> $variant, $condition)
    };
    (<$fmt0:ident | $fmt1:ident> $variant:ident, Some($condition:ident)) => {
        $crate::operation::macros::coprocessor_1!(<$fmt0> $variant, Some($condition))
        | $crate::operation::macros::coprocessor_1!(<$fmt1> $variant, Some($condition))
    };
    (<$fmt0:ident | $fmt1:ident> $variant:ident, $condition0:literal | $condition1:literal) => {
        $crate::operation::macros::coprocessor_1!(<$fmt0 | $fmt1> $variant, $condition0)
        | $crate::operation::macros::coprocessor_1!(<$fmt0 | $fmt1> $variant, $condition1)
    };
    [<$fmt:ident> $variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::operation::macros::coprocessor_1!(<$fmt> $variant)
        | $crate::operation::macros::coprocessor_1!(<$fmt> $($variants),+)
    };
    [{$fmt:ident} $variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::operation::macros::coprocessor_1!({$fmt} $variant)
        | $crate::operation::macros::coprocessor_1!({$fmt} $($variants),+)
    };
    [<$fmt0:ident | $fmt1:ident> $variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::operation::macros::coprocessor_1!(<$fmt0 | $fmt1> $variant)
        | $crate::operation::macros::coprocessor_1!(<$fmt0 | $fmt1> $($variants),+)
    };
}
pub(crate) use coprocessor_1;

macro_rules! special_2 {
    ($variant:ident) => {
        BasicOperator::Special2(seaside_constants::fn_codes::Special2Fn::$variant)
    };
    [$variant:ident, $($variants:ident),+ $(,)?] => {
        $crate::operation::macros::special_2!($variant)
        | $crate::operation::macros::special_2!($($variants),+)
    };
}
pub(crate) use special_2;
