use crate::fields::{DesignatedField, ObservableEventSpec};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error};

pub fn derive_observation_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = input.ident.clone();

    let cs = DesignatedField::extract_fields(input)?;

    let field_assignments = cs
        .iter()
        .enumerate()
        .map(|(idx, DesignatedField { name, .. })| {
            quote! {
                #name: measurements[#idx]
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
        impl ::dirty_mike_core::Observation <#n> for #name {
            fn new(measurements: [u64; #n]) -> Self {
                Self {
                    #(#field_assignments),*
                }
            }

            fn fields() -> &'static [dirty_mike_core::MeasurementDefinition; #n] {
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
        .map(|DesignatedField { name, spec }| {
            let raw_code = match spec {
                ObservableEventSpec::Raw(r) => quote! {
                    ::dirty_mike_core::EventCode::Raw(#r)
                },
                ObservableEventSpec::Hardware(hw) => {
                    let ident = quote::format_ident!("{}", hw.to_uppercase());
                    quote! {
                        ::dirty_mike_core::EventCode::Hardware(
                        ::dirty_mike_core::pe2::events::Hardware::#ident.0)
                    }
                }
            };

            let name_as_s = name.to_string();
            quote! {
                ::dirty_mike_core::MeasurementDefinition {
                    name: #name_as_s,
                    code: #raw_code,
                }
            }
        })
        .collect::<Vec<_>>();

    let static_field_defn = quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
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
