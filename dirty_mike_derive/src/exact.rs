use crate::DesignatedField;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error};

pub fn derive_exact_counter_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = input.ident.clone();

    let cs = DesignatedField::extract_fields(input)?;

    let imports = quote! {
        use ::dirty_mike_core::exact::*;
    };

    let enable_measurements = cs
        .iter()
        .map(|DesignatedField { spec, .. }| {
            let num = spec.unwrap_raw();
            quote! {
                .measure(#num)
            }
        })
        .collect::<Vec<_>>();

    let code = quote! {
        impl #name {
            pub fn measure<T, F: FnOnce() -> T>(f: F) -> Result<T, ::dirty_mike_core::exact::ExactMeasurementsError> {
                #imports

                let eb = ExactMeasurements::builder()
                    #(#enable_measurements)*
                    .build()?;

                let (outcome, reading) = eb.measure(f)?;

                Ok(outcome)
            }
        }
    };

    Ok(code.into())
}
