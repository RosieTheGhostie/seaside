use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Token, Variant, punctuated::Punctuated, spanned::Spanned};

/**
 * Service::Spim(Spim::Print(Print::String)) => "spim.print.string"
 */

pub fn generate_code(ident: Ident, variants: Punctuated<Variant, Token![,]>) -> TokenStream2 {
    let mut branches: Punctuated<TokenStream2, Token![else]> = Punctuated::new();
    let mut match_arms = Vec::new();
    for variant in variants {
        let span = variant.span();
        let variant_ident = variant.ident;
        let prefix = format!("{}.", heck::AsSnakeCase(format!("{variant_ident}")));
        let fmt_string = format!("{}{{x}}", prefix);
        let prefix_str = syn::LitStr::new(&prefix, variant_ident.span());
        let fmt_str = syn::LitStr::new(&fmt_string, span);
        branches.push(quote! {
            if let ::core::option::Option::Some(suffix) = s.strip_prefix(#prefix_str) {
                ::core::result::Result::Ok(Self::#variant_ident(<_ as ::core::str::FromStr>::from_str(suffix)?))
            }
        });
        match_arms.push(quote! {
            Self::#variant_ident(x) => write!(f, #fmt_str),
        });
    }
    branches.push(quote! {
        {
            ::core::result::Result::Err(::strum::ParseError::VariantNotFound)
        }
    });
    quote! {
        impl ::core::str::FromStr for #ident {
            type Err = ::strum::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #branches
            }
        }

        impl ::core::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #(#match_arms)*
                }
            }
        }
    }
}
