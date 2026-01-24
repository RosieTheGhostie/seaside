pub mod specifier;

pub use specifier::ServiceSpecifier;

use std::collections::HashMap;

use seaside_address_range::sized::SizedAddressRange;
use seaside_type_aliases::ServiceCode;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Services {
    pub providers: Vec<SizedAddressRange>,
    pub groups: Vec<SizedAddressRange>,
    pub subgroups: Vec<SizedAddressRange>,
    pub members: Vec<SizedAddressRange>,
    pub mapping: HashMap<ServiceCode, ServiceSpecifier>,
}
