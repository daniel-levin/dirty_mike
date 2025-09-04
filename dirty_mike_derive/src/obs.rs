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

    let static_field_name = quote::format_ident!("MEASUREMENT_DEFNS_{}", &name);

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

            fn fields() -> &'static [dirty_mike_core::MeasurementDefinition] {
                & #static_field_name
            }

            fn measurements(&self) -> [u64; #n] {
                [
                    #(#field_reads),*
                ]
            }
        }
    };

    let static_fields = cs
        .iter()
        .map(|DesignatedField { name, .. }| {
            let name_as_s = name.to_string();
            quote! {
                ::dirty_mike_core::MeasurementDefinition {
                    name: #name_as_s
                }
            }
        })
        .collect::<Vec<_>>();

    let static_field_defn = quote! {
        #[doc(hidden)]
        static #static_field_name: [::dirty_mike_core::MeasurementDefinition; #n] = [
            #(#static_fields),*
        ];
    };

    let items = quote! {
        #static_field_defn

        #trait_impl
    };

    Ok(items.into())
}
