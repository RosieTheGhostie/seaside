use seaside_service_group::{NestedServiceGroup, ServiceGroup};
use strum::{EnumString, IntoStaticStr};

#[derive(Clone, Copy, Debug, Eq, NestedServiceGroup, PartialEq)]
pub enum Spim {
    Print(Print),
    Read(Read),
    File(File),
    System(System),
}

#[derive(Clone, Copy, Debug, EnumString, Eq, IntoStaticStr, PartialEq, ServiceGroup)]
#[strum(serialize_all = "snake_case")]
pub enum Print {
    Int,
    Float,
    Double,
    Char,
    String,
}

#[derive(Clone, Copy, Debug, EnumString, Eq, IntoStaticStr, PartialEq, ServiceGroup)]
#[strum(serialize_all = "snake_case")]
pub enum Read {
    Int,
    Float,
    Double,
    Char,
    String,
}

#[derive(Clone, Copy, Debug, EnumString, Eq, IntoStaticStr, PartialEq, ServiceGroup)]
#[strum(serialize_all = "snake_case")]
pub enum File {
    Open,
    Read,
    Write,
    Close,
}

#[derive(Clone, Copy, Debug, EnumString, Eq, IntoStaticStr, PartialEq, ServiceGroup)]
#[strum(serialize_all = "snake_case")]
pub enum System {
    Sbrk,
    Exit,
    Exit2,
}
