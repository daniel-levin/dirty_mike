use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Ident, Meta, Type,
    TypePath,
};

struct CounterField {
    name: Ident,
    counter_ident: Ident,
    _ty: TypePath,
    hardware_attr: Option<String>,
}

fn parse_hardware_attribute(attrs: &[Attribute]) -> Result<Option<String>, Error> {
    for attr in attrs {
        if attr.path().is_ident("hardware") {
            match &attr.meta {
                Meta::List(list) => {
                    let tokens = &list.tokens;
                    let token_str = tokens.to_string();
                    return Ok(Some(token_str));
                }
                _ => {
                    return Err(Error::new_spanned(
                        attr,
                        "hardware attribute must have a value like #[hardware(CPU_CYCLES)]",
                    ));
                }
            }
        }
    }
    Ok(None)
}

fn obtain_counter_fields(fields: FieldsNamed) -> Result<Vec<CounterField>, Error> {
    let mut counters = vec![];

    for Field {
        attrs, ident, ty, ..
    } in fields.named
    {
        let Type::Path(tp) = ty else {
            return Err(Error::new_spanned(ty, "counters do not support this type"));
        };

        let name = ident.unwrap();
        let counter_ident = quote::format_ident!("{}_counter", &name);
        let hardware_attr = parse_hardware_attribute(&attrs)?;

        counters.push(CounterField {
            name,
            _ty: tp,
            counter_ident,
            hardware_attr,
        });
    }

    Ok(counters)
}

fn all_fields_set(counter_fields: &[CounterField]) -> proc_macro2::TokenStream {
    let mut assignments = vec![];

    for CounterField {
        name,
        counter_ident,
        ..
    } in counter_fields
    {
        assignments.push(quote! {
            #name: *&counts[& #counter_ident]
        });
    }

    quote! {
        #(#assignments),*
    }
}

fn add_counters_to_group(counter_fields: &[CounterField]) -> proc_macro2::TokenStream {
    let mut add_counter = vec![];

    for CounterField {
        counter_ident,
        hardware_attr,
        ..
    } in counter_fields
    {
        let hardware_event = if let Some(attr_value) = hardware_attr {
            let tokens: proc_macro2::TokenStream = attr_value.parse().unwrap();
            quote! { Hardware::#tokens }
        } else {
            quote! { Hardware::CPU_CYCLES }
        };

        add_counter.push(quote! {
            let #counter_ident = group.add(&Builder::new(#hardware_event)).unwrap();
        });
    }

    quote! {
        #(#add_counter);*
    }
}

pub fn derive_counter_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;

    if !input.generics.lifetimes().collect::<Vec<_>>().is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "counters should not contain references",
        ));
    }

    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "counters must not contain generics",
        ));
    }

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

    let counters = obtain_counter_fields(fields)?;
    let set_fields = all_fields_set(&counters);

    let mk_struct = quote! {
        let counter = #name {
            #set_fields
        };
    };

    let add_counters = add_counters_to_group(&counters);

    let expanded = quote! {
        impl ::dirty_mike_core::Counter for #name {
            fn measure<T, F: FnOnce() -> T>(mut f: F) -> Result<(T, Self), ::dirty_mike_core::CounterError> {
                #imports

                let mut gb = Group::builder();
                gb.read_format(ReadFormat::all());
                let mut group = gb.build_group().unwrap();

                #add_counters

                group.enable().unwrap();
                let result = f();
                group.disable().unwrap();

                let counts = group.read().unwrap();

                #mk_struct

                Ok((result, counter))
            }
        }
    };

    Ok(TokenStream::from(expanded))
}
