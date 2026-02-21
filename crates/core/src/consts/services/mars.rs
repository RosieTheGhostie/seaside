use seaside_service_group::{NestedServiceGroup, ServiceGroup};
use strum::{EnumString, IntoStaticStr};

#[derive(Clone, Copy, Debug, Eq, NestedServiceGroup, PartialEq)]
pub enum Mars {
    Print(Print),
    System(System),
    Random(Random),
    Dialog(Dialog),
}

#[derive(Clone, Copy, Debug, EnumString, Eq, IntoStaticStr, PartialEq, ServiceGroup)]
#[strum(serialize_all = "snake_case")]
pub enum Print {
    Uint,
    Bin,
    Hex,
}

#[derive(Clone, Copy, Debug, EnumString, Eq, IntoStaticStr, PartialEq, ServiceGroup)]
#[strum(serialize_all = "snake_case")]
pub enum System {
    Time,
    Sleep,
    MidiOut,
    MidiOutSync,
}

#[derive(Clone, Copy, Debug, EnumString, Eq, IntoStaticStr, PartialEq, ServiceGroup)]
#[strum(serialize_all = "snake_case")]
pub enum Random {
    SetSeed,
    RandInt,
    RandIntRange,
    RandFloat,
    RandDouble,
}

#[derive(Clone, Copy, Debug, Eq, NestedServiceGroup, PartialEq)]
pub enum Dialog {
    Input(InputDialog),
    Message(MessageDialog),
}

#[derive(Clone, Copy, Debug, EnumString, Eq, IntoStaticStr, PartialEq, ServiceGroup)]
#[strum(serialize_all = "snake_case")]
pub enum InputDialog {
    Confirm,
    Int,
    Float,
    Double,
    String,
}

#[derive(Clone, Copy, Debug, EnumString, Eq, IntoStaticStr, PartialEq, ServiceGroup)]
#[strum(serialize_all = "snake_case")]
pub enum MessageDialog {
    General,
    Int,
    Float,
    Double,
    String,
}
