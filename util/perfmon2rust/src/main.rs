use serde::{Deserialize, Serialize};

fn dehex<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
    let s = <&str>::deserialize(d)?;
    Ok(u8::from_str_radix(&s[2..=3], 16).unwrap())
}

fn bool_from_str<'de, D: serde::Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    let s = <&str>::deserialize(d)?;
    Ok(s == "1")
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Header {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Event {
    #[serde(deserialize_with = "dehex")]
    event_code: u8,

    #[serde(rename = "UMask", deserialize_with = "dehex")]
    umask: u8,

    event_name: String,

    brief_description: String,

    public_description: String,

    #[serde(deserialize_with = "bool_from_str")]
    any_thread: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Events {
    pub header: Header,
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EventAlias {
    name: String,
    alias: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Metric {
    metric_name: String,
    legacy_name: String,
    level: usize,
    brief_description: String,
    events: Vec<EventAlias>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Metrics {
    pub header: Header,
    pub metrics: Vec<Metric>,
}

fn main() {
    let x = include_str!("../../../extern/perfmon/SKL/events/skylake_core.json");
    let e: Events = serde_json::from_str(x).unwrap();

    let y = include_str!("../../../extern/perfmon/SKL/metrics/skylake_metrics.json");
    let m: Metrics = serde_json::from_str(y).unwrap();

    dbg!(m);
}
