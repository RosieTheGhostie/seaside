extern crate proc_macro;
extern crate proc_macro2;

mod nested;
mod unnested;

use proc_macro::TokenStream;
use proc_macro_error2::{abort, proc_macro_error};
use syn::{
    Data, DataEnum, DataStruct, DataUnion, DeriveInput, parse_macro_input, spanned::Spanned,
};

#[proc_macro_error]
#[proc_macro_derive(ServiceGroup)]
pub fn service_group_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    match data {
        Data::Enum(DataEnum { variants: _, .. }) => {}
        Data::Struct(DataStruct { struct_token, .. }) => {
            abort!(
                struct_token.span(), "unexpected keyword `struct` (expected `enum`)";
                note = "`NestedServiceGroup` can only be derived for enums, not structs or unions";
            );
        }
        Data::Union(DataUnion { union_token, .. }) => {
            abort!(
                union_token.span(), "unexpected keyword `union` (expected `enum`)";
                note = "`NestedServiceGroup` can only be derived for enums, not structs or unions";
            );
        }
    };
    unnested::generate_code(ident).into()
}

#[proc_macro_error]
#[proc_macro_derive(NestedServiceGroup)]
pub fn nested_service_group_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let variants = match data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        Data::Struct(DataStruct { struct_token, .. }) => {
            abort!(
                struct_token.span(), "unexpected keyword `struct` (expected `enum`)";
                note = "`NestedServiceGroup` can only be derived for enums, not structs or unions";
            );
        }
        Data::Union(DataUnion { union_token, .. }) => {
            abort!(
                union_token.span(), "unexpected keyword `union` (expected `enum`)";
                note = "`NestedServiceGroup` can only be derived for enums, not structs or unions";
            );
        }
    };
    nested::generate_code(ident, variants).into()
}
