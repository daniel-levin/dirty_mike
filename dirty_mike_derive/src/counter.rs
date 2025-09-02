use crate::DesignatedField;
use crate::EventSpec;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error};

pub fn derive_counter_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = input.ident.clone();

    let cs = DesignatedField::extract_fields(input)?;

    let mut field_assignments = vec![];
    let mut field_creations = vec![];

    for (i, field) in cs.iter().enumerate() {
        let name = &field.name;

        field_creations.push(match &field.spec {
            EventSpec::Hardware(s) => {
                let as_ident = quote::format_ident!("{}", s);
                quote! {
                    ctrs.add(Hardware :: #as_ident)?;
                }
            }
            EventSpec::Raw(id) => {
                quote! {
                    ctrs.add(Raw :: new(#id))?;
                }
            }
        });

        field_assignments.push(quote! {
            #name: observations[#i]
        });
    }

    let expanded = quote! {
        impl #name {
            pub fn new() -> std::io::Result<::dirty_mike_core::Counters<Self>> {
                use ::dirty_mike_core::pe2::events::{Raw, Hardware};
                let mut ctrs = ::dirty_mike_core::Counters::new();

                #(#field_creations)*

                Ok(ctrs)
            }

            pub fn measure<T, F: FnOnce() -> T>(f: F) -> std::io::Result<(T, Self)> {
                let mut collection = Self::new()?;
                collection.enable()?;
                let result = f();
                collection.disable()?;
                Ok((result, collection.read()?))
            }
        }

        impl ::dirty_mike_core::Measurements for #name {
            fn from_observations(observations: &[u64]) -> Self {
                Self {
                    #(#field_assignments),*
                }
            }
        }
    };

    Ok(expanded.into())
}
