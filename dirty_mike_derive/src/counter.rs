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
    pub fn enablement(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Counter {
                name,
                spec: EventSpec::Hardware(s),
            } => {
                let counter_name = quote::format_ident!("{}_counter", name);
                let as_ident = quote::format_ident!("{}", s);
                Some(quote! {
                    let #counter_name = Builder::new(Hardware:: #as_ident)
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

                        let event_selector = hex8(parts[0]).map_err(|_| {
                            Error::new_spanned(
                                &attr,
                                "first intel parameter must be a valid u8 hex value (0x..)",
                            )
                        })?;

                        let mask = hex8(parts[1]).map_err(|_| {
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

fn hex8(s: &str) -> Result<u8, Box<dyn std::error::Error>> {
    if s.starts_with("0x") || s.starts_with("0X") {
        let hex_str = &s[2..];
        let bytes = hex::decode(hex_str)?;
        if bytes.len() != 1 {
            return Err(format!("hex value '{}' must represent exactly one byte", s).into());
        }
        Ok(bytes[0])
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
    fn test_different_intel_hex_values() {
        let tests = vec![
            ("0x00, 0x01", 0x00, 0x01),
            ("0x89, 0xFF", 0x89, 0xFF),
            ("0xAB, 0xCD", 0xAB, 0xCD),
            ("0x12, 0x34", 0x12, 0x34),
            ("0XFF, 0X00", 0xFF, 0x00), // Test uppercase 0X
        ];

        for (hex_str, expected_selector, expected_mask) in tests {
            let test_str = format!(
                r#"
                {{
                    #[intel({})]
                    counter: u64,
                }}
                "#,
                hex_str
            );

            let fields: FieldsNamed = syn::parse_str(&test_str).unwrap();
            let cs = CounterSpec::from_named_fields(fields).unwrap();

            assert_eq!(cs.counter_fields.len(), 1);
            if let CounterField::Counter {
                spec: EventSpec::Intel(selector, mask),
                ..
            } = &cs.counter_fields[0]
            {
                assert_eq!(
                    *selector, expected_selector,
                    "Selector mismatch for {}",
                    hex_str
                );
                assert_eq!(*mask, expected_mask, "Mask mismatch for {}", hex_str);
            } else {
                panic!("Expected Intel counter field for {}", hex_str);
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

    #[test]
    fn test_mixed_field_types() {
        let test_str = r#"
        {
            #[hardware(CPU_CYCLES)]
            cpu_cycles: u64,

            #[intel(0x89, 0xFF)]
            branch_misses: u64,

            #[time_enabled]
            time_enabled: Duration,

            #[time_running]
            time_running: Duration,

            #[hardware(INSTRUCTIONS)]
            instructions: u64,
        }
        "#;

        let fields: FieldsNamed = syn::parse_str(test_str).unwrap();
        let cs = CounterSpec::from_named_fields(fields).unwrap();

        assert_eq!(cs.counter_fields.len(), 5);

        // Check CPU_CYCLES
        if let CounterField::Counter {
            spec: EventSpec::Hardware(event),
            ..
        } = &cs.counter_fields[0]
        {
            assert_eq!(event, "CPU_CYCLES");
        } else {
            panic!("Expected hardware counter field for CPU_CYCLES");
        }

        // Check Intel counter
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

        // Check time fields
        assert!(matches!(cs.counter_fields[2], CounterField::TimeEnabled));
        assert!(matches!(cs.counter_fields[3], CounterField::TimeRunning));

        // Check INSTRUCTIONS
        if let CounterField::Counter {
            spec: EventSpec::Hardware(event),
            ..
        } = &cs.counter_fields[4]
        {
            assert_eq!(event, "INSTRUCTIONS");
        } else {
            panic!("Expected hardware counter field for INSTRUCTIONS");
        }
    }

    #[test]
    fn test_hex8_function() {
        // Test valid hex values
        assert_eq!(hex8("0x00").unwrap(), 0x00);
        assert_eq!(hex8("0xFF").unwrap(), 0xFF);
        assert_eq!(hex8("0x89").unwrap(), 0x89);
        assert_eq!(hex8("0XAB").unwrap(), 0xAB); // uppercase 0X

        // Test invalid formats
        assert!(hex8("123").is_err()); // no 0x prefix
        assert!(hex8("0x").is_err()); // empty hex
        assert!(hex8("0xGG").is_err()); // invalid hex chars
        assert!(hex8("0x1234").is_err()); // too many bytes (2 bytes instead of 1)
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

    let imports = quote! {
        use perf_event::ReadFormat;
        use perf_event::events::Hardware;
        use perf_event::{Builder, Group};
    };

    let mut counter_enablements = vec![];

    for spec in cs.counter_fields.iter() {
        if let Some(enablement) = spec.enablement() {
            counter_enablements.push(enablement);
        }
    }

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

                Ok((result, todo!()))
            }
        }
    };

    //panic!("{}", expanded.to_string());

    Ok(expanded.into())
}
