use quote::quote;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

fn dehex<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
    let s = <&str>::deserialize(d)?;
    let clip = std::cmp::min(3, s.len() - 1);
    Ok(u8::from_str_radix(&s[2..=clip], 16).unwrap())
}

fn dedecimal<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
    let s = <&str>::deserialize(d)?;
    Ok(s.parse::<u8>().unwrap())
}

fn bool_from_str<'de, D: serde::Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    let s = <&str>::deserialize(d)?;
    Ok(s == "1")
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Header {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "TMA")]
    Tma,
    #[serde(rename = "BW")]
    Bandwidth,
    #[serde(rename = "BW, IO")]
    BandwidthIo,
    #[serde(rename = "BW,IO")]
    BandwidthIoNoSpace,
    #[serde(rename = "BW, MC")]
    BandwidthMc,
    #[serde(rename = "CPI")]
    Cpi,
    #[serde(rename = "D-side")]
    DataSide,
    #[serde(rename = "Freq")]
    Frequency,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IO, BW")]
    IoBandwidth,
    #[serde(rename = "I-side")]
    InstructionSide,
    #[serde(rename = "Latency")]
    Latency,
    #[serde(rename = "MPI")]
    Mpi,
    #[serde(rename = "MPI, D-side")]
    MpiDataSide,
    #[serde(rename = "MPI, I-side")]
    MpiInstructionSide,
    #[serde(rename = "NUMA")]
    Numa,
    #[serde(rename = "Power")]
    Power,
    #[serde(rename = "Util")]
    Utilization,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParentCategory {
    #[serde(rename = "ALU_Op_Utilization")]
    AluOpUtilization,
    #[serde(rename = "Assists")]
    Assists,
    #[serde(rename = "Backend_Bound")]
    BackendBound,
    #[serde(rename = "Bad_Speculation")]
    BadSpeculation,
    #[serde(rename = "Branch_Mispredicts")]
    BranchMispredicts,
    #[serde(rename = "Branch_Resteers")]
    BranchResteers,
    #[serde(rename = "Code_STLB_Miss")]
    CodeStlbMiss,
    #[serde(rename = "Core_Bound")]
    CoreBound,
    #[serde(rename = "Divider")]
    Divider,
    #[serde(rename = "DRAM_Bound")]
    DramBound,
    #[serde(rename = "DTLB_Load")]
    DtlbLoad,
    #[serde(rename = "DTLB_Store")]
    DtlbStore,
    #[serde(rename = "Fetch_Bandwidth")]
    FetchBandwidth,
    #[serde(rename = "Fetch_Latency")]
    FetchLatency,
    #[serde(rename = "FP_Arith")]
    FpArith,
    #[serde(rename = "FP_Vector")]
    FpVector,
    #[serde(rename = "Frontend_Bound")]
    FrontendBound,
    #[serde(rename = "Heavy_Operations")]
    HeavyOperations,
    #[serde(rename = "ICache_Misses")]
    ICacheMisses,
    #[serde(rename = "IFetch_Bandwidth")]
    IFetchBandwidth,
    #[serde(rename = "IFetch_Latency")]
    IFetchLatency,
    #[serde(rename = "Int_Operations")]
    IntOperations,
    #[serde(rename = "ITLB_Misses")]
    ItlbMisses,
    #[serde(rename = "L1_Bound")]
    L1Bound,
    #[serde(rename = "L2_Bound")]
    L2Bound,
    #[serde(rename = "L3_Bound")]
    L3Bound,
    #[serde(rename = "Light_Operations")]
    LightOperations,
    #[serde(rename = "Load_Op_Utilization")]
    LoadOpUtilization,
    #[serde(rename = "Load_STLB_Miss")]
    LoadStlbMiss,
    #[serde(rename = "Machine_Clears")]
    MachineClears,
    #[serde(rename = "MEM_Bandwidth")]
    MemBandwidth,
    #[serde(rename = "MEM_Latency")]
    MemLatency,
    #[serde(rename = "Memory_Bound")]
    MemoryBound,
    #[serde(rename = "Microcode_Sequencer")]
    MicrocodeSequencer,
    #[serde(rename = "MITE")]
    Mite,
    #[serde(rename = "Other_Light_Ops")]
    OtherLightOps,
    #[serde(rename = "Ports_Utilization")]
    PortsUtilization,
    #[serde(rename = "Ports_Utilized_0")]
    PortsUtilized0,
    #[serde(rename = "Ports_Utilized_3m")]
    PortsUtilized3m,
    #[serde(rename = "Resource_Bound")]
    ResourceBound,
    #[serde(rename = "Retiring")]
    Retiring,
    #[serde(rename = "Serializing_Operation")]
    SerializingOperation,
    #[serde(rename = "Store_Bound")]
    StoreBound,
    #[serde(rename = "Store_Op_Utilization")]
    StoreOpUtilization,
    #[serde(rename = "Store_STLB_Miss")]
    StoreStlbMiss,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    #[serde(deserialize_with = "dehex")]
    pub event_code: u8,

    #[serde(rename = "UMask", deserialize_with = "dehex")]
    pub umask: u8,

    pub event_name: String,

    pub brief_description: String,

    pub public_description: String,

    #[serde(deserialize_with = "bool_from_str", default)]
    pub any_thread: bool,

    #[serde(deserialize_with = "bool_from_str")]
    pub invert: bool,

    #[serde(deserialize_with = "dedecimal")]
    pub counter_mask: u8,
}

impl Event {
    /// Vol. 3B 21-9
    /// Layout of IA32_PERFEVTSELx MSRs
    pub fn raw(&self) -> u64 {
        ((self.counter_mask as u64) << 24)
            | ((self.invert as u64) << 23)
            | ((self.any_thread as u64) << 21)
            | ((self.umask as u64) << 8)
            | (self.event_code as u64)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Events {
    pub header: Header,
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventAlias {
    pub name: String,
    pub alias: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Constant {
    pub name: String,
    pub alias: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metric {
    pub metric_name: String,
    pub legacy_name: String,
    pub level: usize,
    pub brief_description: String,
    pub events: Vec<EventAlias>,
    pub constants: Vec<Constant>,
    pub category: Category,
    pub parent_category: Option<ParentCategory>,
    pub base_formula: String,
    pub formula: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metrics {
    pub header: Header,
    pub metrics: Vec<Metric>,
}

#[derive(Debug, Default)]
pub struct EventDefinitions {
    events: HashMap<String, Event>,
}

impl EventDefinitions {
    pub fn slurp<P: AsRef<Path>>(p: P) -> anyhow::Result<Self> {
        let mut events = HashMap::new();

        for entry in p.as_ref().canonicalize()?.read_dir()? {
            let entry = entry?;

            if let Some(Some("json")) = entry.path().extension().map(|s| s.to_str()) {
                let contents = std::fs::read(entry.path())?;
                if let Ok(e) = serde_json::from_slice::<Events>(&contents) {
                    for event in e.events {
                        events.insert(event.event_name.clone(), event);
                    }
                }
            }
        }

        Ok(Self { events })
    }

    pub fn lookup(&self, ea: &EventAlias) -> Option<&Event> {
        let search_for = ea.name.split(":").next().unwrap();
        self.events.get(search_for)
    }
}

#[derive(Debug)]
pub struct CounterSpec {
    name: proc_macro2::Ident,
    fields: std::collections::HashSet<(proc_macro2::Ident, u64)>,
}

impl CounterSpec {
    pub fn new(metric: &Metric, event_defns: &EventDefinitions) -> anyhow::Result<Self> {
        let mut fields = HashSet::new();

        for ea in &metric.events {
            let Some(defn) = event_defns.lookup(ea) else {
                anyhow::bail!("cannot find event definition for {}", ea.name);
            };

            let name_to_use = ea.name.replace(".", "__").replace(":", "_");

            let name = if name_to_use.chars().nth(0).unwrap().is_ascii_digit() {
                quote::format_ident!("_{}", name_to_use)
            } else {
                quote::format_ident!("{}", name_to_use)
            };

            fields.insert((name, defn.raw()));
        }

        let name = if metric.metric_name.chars().nth(0).unwrap().is_ascii_digit() {
            quote::format_ident!("_{}", metric.metric_name)
        } else {
            quote::format_ident!("{}", metric.metric_name)
        };

        Ok(Self { name, fields })
    }

    pub fn as_rust_code(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let mut fields = vec![];

        for (name, raw) in self.fields.iter() {
            let value = proc_macro2::Literal::from_str(&format!("0x{raw:x}")).unwrap();
            fields.push(quote! {
                #[raw(#value)]
                #name: u64
            });
        }

        quote! {
            #[derive(Debug, Counter)]
            pub struct #name {
                #(#fields),*
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SKL_METRICS: &str =
        include_str!("../../../extern/perfmon/SKL/metrics/skylake_metrics.json");
    static SKL_CORE_EVENTS: &str =
        include_str!("../../../extern/perfmon/SKL/events/skylake_core.json");

    #[test]
    fn translate_to_counter_spec() -> anyhow::Result<()> {
        let ed = EventDefinitions::slurp("../../extern/perfmon/SKL/events/")?;

        let metrics: Metrics = serde_json::from_str(SKL_METRICS)?;

        let mut items = vec![];

        for metric in metrics
            .metrics
            .iter()
            .filter(|m| m.metric_name != "Info_System_Power")
        {
            let spec = CounterSpec::new(&metric, &ed)?;
            items.push(spec.as_rust_code());
        }

        let generated_code = quote! {
            #(#items)*
        }
        .to_string();

        println!("{}", generated_code);

        Ok(())
    }

    #[test]
    fn slurp_events() -> anyhow::Result<()> {
        let ec = EventDefinitions::slurp("../../extern/perfmon/SKL/events/")?;

        let metrics: Metrics = serde_json::from_str(SKL_METRICS)?;

        for Metric { events, .. } in metrics.metrics {
            for e in events {
                match ec.lookup(&e) {
                    Some(_e2) => {}
                    None => {
                        // Oddly, this one is missing for some reason.
                        assert_eq!(e.name, "UNC_PKG_ENERGY_STATUS");
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    fn event_code() {
        let e: Events = serde_json::from_str(SKL_CORE_EVENTS).unwrap();

        let evt = e
            .events
            .iter()
            .find(|e| e.event_name == "DTLB_LOAD_MISSES.WALK_COMPLETED_2M_4M")
            .unwrap();

        assert_eq!(evt.raw(), 0x408);
    }

    #[test]
    fn event_code_and_any_thread() {
        let e: Events = serde_json::from_str(SKL_CORE_EVENTS).unwrap();

        let evt = e
            .events
            .iter()
            .find(|e| e.event_name == "INT_MISC.RECOVERY_CYCLES_ANY")
            .unwrap();

        assert_eq!(evt.raw(), 0x20010d);

        let evt = e
            .events
            .iter()
            .find(|e| e.event_name == "CPU_CLK_UNHALTED.THREAD_P_ANY")
            .unwrap();

        assert_eq!(evt.raw(), 0x20003c);
    }

    #[test]
    fn counter_mask_and_invert() {
        let e: Events = serde_json::from_str(SKL_CORE_EVENTS).unwrap();
        let evt = e
            .events
            .iter()
            .find(|e| e.event_name == "UOPS_RETIRED.TOTAL_CYCLES")
            .unwrap();

        assert_eq!(evt.raw(), 0x108002c2);
    }
}
