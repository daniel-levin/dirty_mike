use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Ident, Meta, Type};

#[derive(Debug)]
enum EventSpec {
    Hardware(String),

    Raw(u64),
}

#[derive(Debug)]
enum CounterField {
    Counter { name: Ident, spec: EventSpec },

    TimeEnabled,

    TimeRunning,
}

impl CounterField {
    pub fn extraction(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Counter { name, .. } => {
                let counter_name = quote::format_ident!("{}_counter", name);
                quote! {
                    #name: counts[& #counter_name]
                }
            }
            _ => todo!(),
        }
    }

    pub fn enablement(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Counter {
                name,
                spec: EventSpec::Hardware(s),
            } => {
                let counter_name = quote::format_ident!("{}_counter", name);
                let as_ident = quote::format_ident!("{}", s);
                Some(quote! {
                    let #counter_name = group.add(&Builder::new(Hardware:: #as_ident)).unwrap();
                })
            }
            Self::Counter {
                name,
                spec: EventSpec::Raw(id),
            } => {
                let counter_name = quote::format_ident!("{}_counter", name);
                Some(quote! {
                    let #counter_name = group.add(&Builder::new(Raw::new( #id))).unwrap();
                })
            }
            _ => None,
        }
    }

    pub fn extract_from_field(
        Field {
            attrs, ident, ty, ..
        }: Field,
    ) -> Result<Self, Error> {
        let ident = ident.unwrap();

        let Type::Path(_tp) = &ty else {
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
                                format!(
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
            } else if attr.path().is_ident("raw") {
                match &attr.meta {
                    Meta::List(list) => {
                        let token_str = list.tokens.to_string().to_lowercase();

                        let radix = token_str.strip_prefix("0x").unwrap_or(&token_str);

                        let id = u64::from_str_radix(radix, 16).unwrap();

                        return Ok(Self::Counter {
                            name: ident,
                            spec: EventSpec::Raw(id),
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

    let imports = quote! {
        use perf_event::ReadFormat;
        use perf_event::events::Hardware;
        use perf_event::events::Raw;
        use perf_event::{Builder, Group};
    };

    let mut counter_enablements = vec![];

    for spec in cs.counter_fields.iter() {
        if let Some(enablement) = spec.enablement() {
            counter_enablements.push(enablement);
        }
    }

    let counter_extractions = cs
        .counter_fields
        .iter()
        .map(|f| f.extraction())
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl ::dirty_mike_core::Counter for #name {
            fn measure<T, F: FnOnce() -> T>(mut f: F) -> Result<(T, Self), ::dirty_mike_core::CounterError> {
                #imports

                let mut gb = Group::builder();
                gb.read_format(ReadFormat::all());
                let mut group = gb.build_group().unwrap();

                #(#counter_enablements);* ;

                group.enable().unwrap();
                let result = f();
                group.disable().unwrap();

                let counts = group.read().unwrap();

                let counter = #name {
                    #(#counter_extractions),*
                };

                Ok((result, counter))
            }
        }
    };

    //panic!("{}", expanded.to_string());

    Ok(expanded.into())
}

#[cfg(test)]
mod inner_tests {
    use super::*;

    static S1: &str = r#"
    {
        #[hardware(CPU_CYCLES)]
        a: u64,

        #[raw(0xFF89)]
        br_misp_exec_all_branches: u64,
    }
    "#;

    #[test]
    fn extract_counter_spec() {
        let s1: FieldsNamed = syn::parse_str(S1).unwrap();

        let cs = CounterSpec::from_named_fields(s1).unwrap();

        assert_eq!(cs.counter_fields.len(), 2);

        if let CounterField::Counter {
            spec: EventSpec::Raw(id),
            ..
        } = &cs.counter_fields[1]
        {
            assert_eq!(*id, 0xFF89);
        } else {
            panic!("Expected Intel counter field");
        }
    }

    #[test]
    fn test_different_hardware_events() {
        let tests = vec![
            ("CPU_CYCLES", "CPU_CYCLES"),
            ("INSTRUCTIONS", "INSTRUCTIONS"),
            ("CACHE_REFERENCES", "CACHE_REFERENCES"),
            ("CACHE_MISSES", "CACHE_MISSES"),
            ("BRANCH_INSTRUCTIONS", "BRANCH_INSTRUCTIONS"),
            ("BRANCH_MISSES", "BRANCH_MISSES"),
            ("BUS_CYCLES", "BUS_CYCLES"),
            ("STALLED_CYCLES_FRONTEND", "STALLED_CYCLES_FRONTEND"),
            ("STALLED_CYCLES_BACKEND", "STALLED_CYCLES_BACKEND"),
            ("REF_CPU_CYCLES", "REF_CPU_CYCLES"),
        ];

        for (event, expected) in tests {
            let test_str = format!(
                r#"
                {{
                    #[hardware({})]
                    counter: u64,
                }}
                "#,
                event
            );

            let fields: FieldsNamed = syn::parse_str(&test_str).unwrap();
            let cs = CounterSpec::from_named_fields(fields).unwrap();

            assert_eq!(cs.counter_fields.len(), 1);
            if let CounterField::Counter {
                spec: EventSpec::Hardware(hw_event),
                ..
            } = &cs.counter_fields[0]
            {
                assert_eq!(hw_event, expected);
            } else {
                panic!("Expected hardware counter field for {}", event);
            }
        }
    }

    #[test]
    fn test_time_fields() {
        let test_str = r#"
        {
            #[time_enabled]
            enabled_time: Duration,

            #[time_running]
            running_time: Duration,
        }
        "#;

        let fields: FieldsNamed = syn::parse_str(test_str).unwrap();
        let cs = CounterSpec::from_named_fields(fields).unwrap();

        assert_eq!(cs.counter_fields.len(), 2);

        assert!(matches!(cs.counter_fields[0], CounterField::TimeEnabled));
        assert!(matches!(cs.counter_fields[1], CounterField::TimeRunning));
    }
}
