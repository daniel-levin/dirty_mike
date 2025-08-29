use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod counter;

#[proc_macro_derive(Counter, attributes(hardware, raw))]
pub fn derive_counter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match counter::derive_counter_inner(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}
