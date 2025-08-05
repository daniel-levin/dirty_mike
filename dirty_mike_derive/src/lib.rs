use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Scope)]
pub fn derive_scope(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ::dirty_mike_core::Scope for #name {
            fn measure<T, F: Fn() -> T>(f: F) -> Result<(T, Self), ::dirty_mike_core::ScopeError> {
                todo!();
            }
        }
    };

    TokenStream::from(expanded)
}
