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

    let field_reads = cs
        .iter()
        .map(|DesignatedField { name, .. }| {
            quote! {
                self . #name
            }
        })
        .collect::<Vec<_>>();

    let expected_count = cs.len();
    let n = expected_count;

    let trait_impl = quote! {
        impl ::dirty_mike_core::Observations <#n> for #name {
            fn new(observations: &[u64]) -> Result<Self, ::dirty_mike_core::ObservationsOutOfBounds> {
                let received: usize = observations.len();
                let expected: usize = #expected_count;

                if received != expected {
                    Err(::dirty_mike_core::ObservationsOutOfBounds { received, expected })
                } else {
                    Ok(Self {
                        #(#field_assignments),*
                    })
                }
            }

            fn measurements(&self) -> [u64; #n] {
                [
                    #(#field_reads),*
                ]
            }
        }
    };

    Ok(trait_impl.into())
}
