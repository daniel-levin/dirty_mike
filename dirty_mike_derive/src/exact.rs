use crate::DesignatedField;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error};

pub fn derive_exact_counter_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let _name = input.ident.clone();

    let _cs = DesignatedField::extract_fields(input)?;

    Ok(quote! {}.into())
}
