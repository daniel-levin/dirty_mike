use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Ident, Meta, Type};

#[derive(Debug)]
struct RawField {
    name: Ident,
    spec: u64,
}

pub fn derive_counter_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;

    Ok(quote! {}.into())
}

#[cfg(test)]
mod tests {}
