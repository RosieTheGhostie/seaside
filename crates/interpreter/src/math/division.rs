#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Division<T> {
    pub quotient: T,
    pub remainder: T,
}

macro_rules! divmod {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        $crate::math::Division {
            quotient: a / b,
            remainder: a % b,
        }
    }};
}
pub(super) use divmod;
