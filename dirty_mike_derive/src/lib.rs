use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::{Data, DataStruct, DeriveInput, Error, Field, Fields, Meta, Type};

mod counter;
mod exact;

#[proc_macro_derive(Counter, attributes(hardware, raw))]
pub fn derive_counter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match counter::derive_counter_inner(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(ExactCounter, attributes(raw))]
pub fn derive_exact_counter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match exact::derive_exact_counter_inner(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}

#[derive(Debug)]
pub(crate) enum EventSpec {
    Hardware(String),
    Raw(u64),
}

impl EventSpec {
    pub fn unwrap_raw(&self) -> u64 {
        let Self::Raw(x) = &self else {
            panic!("");
        };

        *x
    }
}

#[derive(Debug)]
pub(crate) struct DesignatedField {
    pub name: syn::Ident,
    pub spec: EventSpec,
}

impl DesignatedField {
    pub fn extract_fields(input: DeriveInput) -> Result<Vec<DesignatedField>, syn::Error> {
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

        let mut extracted_fields = vec![];

        for nf in fields.named {
            extracted_fields.push(Self::extract_single_field(nf)?);
        }

        Ok(extracted_fields)
    }

    pub fn extract_single_field(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_extractions() {
        let i = syn::parse_str(
            r#"
        #[derive(Debug, ExactCounter)]
        pub struct A {
        }
        "#,
        )
        .unwrap();

        assert!(DesignatedField::extract_fields(i).unwrap().is_empty());
    }

    #[test]
    fn test_mixed_extractions() {
        let i = syn::parse_str(
            r#"
        #[derive(Debug, ExactCounter)]
        pub struct A {
            #[raw(0xab)]
            a: u64,

            #[raw(0xcd)]
            pub b: u64
        }
        "#,
        )
        .unwrap();

        let fields = DesignatedField::extract_fields(i).unwrap();

        assert!(matches!(
            &fields[0],
            DesignatedField {
                spec: EventSpec::Raw(0xab),
                ..
            }
        ));

        assert!(matches!(
            &fields[1],
            DesignatedField {
                spec: EventSpec::Raw(0xcd),
                ..
            }
        ));
    }
}
