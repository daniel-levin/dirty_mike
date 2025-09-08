use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

pub(crate) mod fields;
mod obs;

#[proc_macro_derive(Observation, attributes(hardware, raw))]
pub fn derive_observation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match obs::derive_observation_inner(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}
