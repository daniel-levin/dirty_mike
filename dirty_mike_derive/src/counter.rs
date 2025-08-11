use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn derive_counter_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;

    // Check for lifetimes first (more specific than generic parameters)
    if !input.generics.lifetimes().collect::<Vec<_>>().is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "counters should not contain references",
        ));
    }

    // Check for generic parameters
    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "counters must not contain generics",
        ));
    }

    // Check that this is a struct (not enum or union)
    match input.data {
        Data::Struct(_) => {}
        Data::Enum(_) => {
            return Err(Error::new_spanned(
                &input,
                "must not be an enum - counters are only supported on structs",
            ));
        }
        Data::Union(_) => {
            return Err(Error::new_spanned(
                &input,
                "must not be a union - counters are only supported on structs",
            ));
        }
    }

    let expanded = quote! {
        impl ::dirty_mike_core::Counter for #name {
            fn measure<T, F: Fn() -> T>(f: F) -> Result<(T, Self), ::dirty_mike_core::CounterError> {
                todo!();
            }
        }
    };

    Ok(TokenStream::from(expanded))
}
