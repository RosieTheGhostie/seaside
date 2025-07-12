use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

pub fn generate_code(ident: Ident) -> TokenStream2 {
    quote! {
        impl ::core::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", <&'static str>::from(self))
            }
        }

        impl ::core::convert::From<#ident> for ::std::string::String {
            fn from(value: #ident) -> Self {
                value.to_string()
            }
        }
    }
}
