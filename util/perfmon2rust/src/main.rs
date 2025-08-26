use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Header {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Event {
    #[serde(deserialize_with = "hex_deser")]
    event_code: u8,

    #[serde(rename = "UMask", deserialize_with = "hex_deser")]
    umask: u8,

    event_name: String,

    brief_description: String,

    public_description: String,
}

fn hex_deser<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
    let s = <&str>::deserialize(d)?;
    Ok(u8::from_str_radix(&s[2..=3], 16).unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Events {
    pub header: Header,
    pub events: Vec<Event>,
}

fn main() {
    let x = include_str!("../../../extern/perfmon/SKL/events/skylake_core.json");
    let e: Events = serde_json::from_str(x).unwrap();

    dbg!(e);
}
