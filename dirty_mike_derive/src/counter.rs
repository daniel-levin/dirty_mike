use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Ident, Meta, Type};

#[derive(Debug)]
enum EventSpec {
    Hardware(String),

    Raw(u64),
}

#[derive(Debug)]
struct CounterField {
    name: Ident,
    spec: EventSpec,
}

impl CounterField {
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
            if attr.path().is_ident("hardware") {
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

                        return Ok(Self {
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

                        return Ok(Self {
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
struct IntendedFields {
    counter_fields: Vec<CounterField>,
}

impl IntendedFields {
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

    let cs = IntendedFields::from_named_fields(fields)?;

    let mut field_assignments = vec![];
    let mut field_creations = vec![];

    for (i, field) in cs.counter_fields.iter().enumerate() {
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

    //panic!("{}", expanded.to_string());

    Ok(expanded.into())
}
