use crate::fields::DesignatedField;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error};

pub fn derive_observations_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = input.ident.clone();

    let cs = DesignatedField::extract_fields(input)?;

    let field_assignments = cs
        .iter()
        .enumerate()
        .map(|(idx, DesignatedField { name, .. })| {
            quote! {
                #name: observations[#idx]
            }
        })
        .collect::<Vec<_>>();

    let trait_impl = quote! {
        impl ::dirty_mike_core::Observations for #name {
            fn new(observations: &[u64]) -> Self {
                Self {
                    #(#field_assignments),*
                }
            }
        }
    };

    Ok(trait_impl.into())
}
