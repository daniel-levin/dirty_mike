use proc_macro::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Error, Field, Fields, Meta, Type, parse_macro_input};

pub(crate) mod fields;
mod obs;

#[proc_macro_derive(Observations, attributes(hardware, raw))]
pub fn derive_observations(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match obs::derive_observations_inner(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}
