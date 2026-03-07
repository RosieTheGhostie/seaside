macro_rules! template {
    ($((I))? $opcode:ident) => {
        $crate::instruction::UnpackedInstruction::$opcode(
            $crate::instruction::immediate::Fields::new_template()
        )
    };
    ((J) $opcode:ident) => {
        $crate::instruction::UnpackedInstruction::$opcode(
            $crate::instruction::jump::Fields::new_template()
        )
    };
    ($((R))? $group:ident$(::<$fmt:ident>)?::$fn:ident $(+ $field:ident: $field_value:expr)?) => {
        $crate::instruction::UnpackedInstruction::$group(
            $crate::instruction::macros::template_fields!(
                $group$(::<$fmt>)?::$fn $(+ $field: $field_value)?
            ),
        )
    };
}
pub(crate) use template;

macro_rules! template_fields {
    (Special::$fn:ident) => {
        $crate::instruction::special::Fields::new_template($crate::consts::codes::SpecialFn::$fn)
    };
    (Special::$fn:ident + rt: $rt:expr) => {{
        let mut fields = $crate::instruction::macros::template_fields!(Special::$fn);
        fields.rt = $rt;
        fields
    }};
    (RegisterImmediate::$fn:ident) => {
        $crate::instruction::register_immediate::Fields::new_template(
            $crate::consts::codes::RegisterImmediateFn::$fn,
        )
    };
    (Coprocessor0::$fn:ident) => {
        $crate::instruction::coprocessor_0::Fields::new_template(
            $crate::consts::codes::Coprocessor0Fn::$fn,
        )
    };
    (@internal Coprocessor1::$fn:ident) => {
        $crate::instruction::coprocessor_1::RegisterImmediateFields::new_template(
            $crate::consts::codes::Coprocessor1RegisterImmediateFn::$fn,
        )
    };
    (Coprocessor1::$fn:ident) => {
        $crate::instruction::coprocessor_1::Fields::RegisterImmediate(
            $crate::instruction::macros::template_fields!(@internal Coprocessor1::$fn),
        )
    };
    (Coprocessor1::$fn:ident + rt: $rt:expr) => {{
        let mut fields = $crate::instruction::macros::template_fields!(@internal Coprocessor1::$fn);
        fields.rt = $rt;
        $crate::instruction::coprocessor_1::Fields::RegisterImmediate(fields)
    }};
    (@internal Coprocessor1::<$fmt:ident>::$fn:ident) => {
        $crate::instruction::coprocessor_1::NormalFields::new_template(
            $crate::consts::codes::Coprocessor1Fn::$fn,
            $crate::consts::formats::NumberFormat::$fmt,
        )
    };
    (Coprocessor1::<$fmt:ident>::$fn:ident) => {
        $crate::instruction::coprocessor_1::Fields::Normal(
            $crate::instruction::macros::template_fields!(@internal Coprocessor1::<$fmt>::$fn),
        )
    };
    (Coprocessor1::<$fmt:ident>::$fn:ident + ft: $ft:expr) => {{
        let mut fields = $crate::instruction::macros::template_fields!(
            @internal Coprocessor1::<$fmt>::$fn
        );
        fields.ft = $ft;
        $crate::instruction::coprocessor_1::Fields::Normal(fields)
    }};
    (Coprocessor2::$fn:ident) => {
        $crate::instruction::coprocessor_2::Fields::new_template(
            $crate::consts::codes::Coprocessor2RegisterImmediateFn::$fn,
        )
    };
    (Coprocessor1X::$fn:ident) => {
        $crate::instruction::coprocessor_1x::Fields::new_template(
            $crate::consts::codes::Coprocessor1XFn::$fn,
        )
    };
    (Special2::$fn:ident) => {
        $crate::instruction::special_2::Fields::new_template($crate::consts::codes::Special2Fn::$fn)
    };
}
pub(crate) use template_fields;
