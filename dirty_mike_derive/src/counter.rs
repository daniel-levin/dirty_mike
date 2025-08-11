use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed};

pub fn derive_counter_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;

    // Check for lifetimes first (more specific than generic parameters)
    if !input.generics.lifetimes().collect::<Vec<_>>().is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "counters should not contain references",
        ));
    }

    // Check for generic parameters
    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "counters must not contain generics",
        ));
    }

    // Check that this is a struct (not enum or union)
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named_fields),
            ..
        }) => named_fields,
        Data::Enum(_) => {
            return Err(Error::new_spanned(
                &input,
                "must not be an enum - counters are only supported on structs",
            ));
        }
        Data::Union(_) => {
            return Err(Error::new_spanned(
                &input,
                "must not be a union - counters are only supported on structs",
            ));
        }
        _ => {
            return Err(Error::new_spanned(
                &input,
                "all fields must be named in counters",
            ));
        }
    };

    let imports = quote! {
        use perf_event::ReadFormat;
        use perf_event::events::Hardware;
        use perf_event::{Builder, Group};
    };

    let expanded = quote! {
        impl ::dirty_mike_core::Counter for #name {
            fn measure<T, F: Fn() -> T>(f: F) -> Result<(T, Self), ::dirty_mike_core::CounterError> {
                #imports

                let mut gb = Group::builder();
                gb.read_format(ReadFormat::all());
                let mut group = gb.build_group().unwrap();

                group.enable().unwrap();
                let result = f();
                group.disable().unwrap();

                let counts = group.read().unwrap();

                dbg!(counts);



                todo!();
            }
        }
    };

    Ok(TokenStream::from(expanded))
}
