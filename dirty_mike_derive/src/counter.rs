use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Ident, Meta, Type,
    TypePath,
};

#[derive(Debug)]
enum EventSpec {
    Hardware(String),

    Intel(u8, u8),
}

#[derive(Debug)]
enum CounterField {
    Counter { name: Ident, spec: EventSpec },

    TimeEnabled,

    TimeRunning,
}

impl CounterField {
    pub fn extract_from_field(
        Field {
            attrs, ident, ty, ..
        }: Field,
    ) -> Result<Self, Error> {
        let ident = ident.unwrap();

        let Type::Path(tp) = &ty else {
            return Err(Error::new_spanned(ty, "counters do not support this type"));
        };

        if attrs.is_empty() {
            return Err(Error::new_spanned(
                ty,
                "every field must have an attribute with a usage",
            ));
        }

        for attr in attrs {
            if attr.path().is_ident("time_enabled") {
                return Ok(Self::TimeEnabled);
            } else if attr.path().is_ident("time_running") {
                return Ok(Self::TimeRunning);
            } else if attr.path().is_ident("hardware") {
                match &attr.meta {
                    Meta::List(list) => {
                        let tokens = &list.tokens;
                        let token_str = tokens.to_string();

                        if !is_valid_hardware_event(&token_str) {
                            return Err(Error::new_spanned(
                                attr,
                                &format!(
                                    "invalid hardware event '{}'. Valid events are: CPU_CYCLES, INSTRUCTIONS, CACHE_REFERENCES, CACHE_MISSES, BRANCH_INSTRUCTIONS, BRANCH_MISSES, BUS_CYCLES, STALLED_CYCLES_FRONTEND, STALLED_CYCLES_BACKEND, REF_CPU_CYCLES",
                                    token_str
                                ),
                            ));
                        }

                        return Ok(Self::Counter {
                            name: ident,
                            spec: EventSpec::Hardware(token_str),
                        });
                    }
                    _ => {
                        return Err(Error::new_spanned(
                            attr,
                            "hardware attribute must have a value like #[hardware(CPU_CYCLES)]",
                        ));
                    }
                }
            } else if attr.path().is_ident("intel") {
                match &attr.meta {
                    Meta::List(list) => {
                        let tokens = &list.tokens;
                        let token_str = tokens.to_string();

                        let parts: Vec<&str> = token_str.split(',').map(|s| s.trim()).collect();
                        if parts.len() != 2 {
                            return Err(Error::new_spanned(
                                attr,
                                "intel attribute must have exactly two values like #[intel(0x89, 0xFF)]",
                            ));
                        }

                        let event_selector = parse_hex(parts[0]).map_err(|_| {
                            Error::new_spanned(
                                &attr,
                                "first intel parameter must be a valid u8 hex value (0x..)",
                            )
                        })?;

                        let mask = parse_hex(parts[1]).map_err(|_| {
                            Error::new_spanned(
                                &attr,
                                "second intel parameter must be a valid u8 hex value (0x..)",
                            )
                        })?;

                        return Ok(Self::Counter {
                            name: ident,
                            spec: EventSpec::Intel(event_selector, mask),
                        });
                    }
                    _ => {
                        return Err(Error::new_spanned(
                            attr,
                            "intel attribute must have values like #[intel(0x89, 0xFF)]",
                        ));
                    }
                }
            }
        }

        Err(Error::new_spanned(
            ident,
            "no known attributes designating a purpose for this field",
        ))
    }
}

#[derive(Debug, Default)]
struct CounterSpec {
    counter_fields: Vec<CounterField>,
}

impl CounterSpec {
    fn from_named_fields(fields: FieldsNamed) -> Result<Self, Error> {
        let mut cs = Self::default();

        for f in fields.named {
            cs.counter_fields.push(CounterField::extract_from_field(f)?);
        }

        Ok(cs)
    }
}

fn is_valid_hardware_event(event: &str) -> bool {
    matches!(
        event,
        "CPU_CYCLES"
            | "INSTRUCTIONS"
            | "CACHE_REFERENCES"
            | "CACHE_MISSES"
            | "BRANCH_INSTRUCTIONS"
            | "BRANCH_MISSES"
            | "BUS_CYCLES"
            | "STALLED_CYCLES_FRONTEND"
            | "STALLED_CYCLES_BACKEND"
            | "REF_CPU_CYCLES"
    )
}

fn parse_hex(s: &str) -> Result<u8, Box<dyn std::error::Error>> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u8::from_str_radix(&s[2..], 16).map_err(Into::into)
    } else {
        Err("hex values must start with 0x or 0X".into())
    }
}

#[cfg(test)]
mod inner_tests {
    use super::*;

    static S1: &str = r#"
    {
        #[hardware(CPU_CYCLES)]
        a: u64,

        #[intel(0x89, 0xFF)]
        br_misp_exec_all_branches: u64,
    }
    "#;

    #[test]
    fn extract_counter_spec() {
        let s1: FieldsNamed = syn::parse_str(S1).unwrap();

        let cs = CounterSpec::from_named_fields(s1).unwrap();

        assert_eq!(cs.counter_fields.len(), 2);

        if let CounterField::Counter {
            spec: EventSpec::Intel(selector, mask),
            ..
        } = &cs.counter_fields[1]
        {
            assert_eq!(*selector, 0x89);
            assert_eq!(*mask, 0xFF);
        } else {
            panic!("Expected Intel counter field");
        }
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

    let cs = CounterSpec::from_named_fields(fields)?;

    Ok(quote! {}.into())
    /*
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
    */
}
