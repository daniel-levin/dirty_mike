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
struct Constant {
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
    category: Category,
    parent_category: Option<ParentCategory>,
    base_formula: String,
    formula: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Metrics {
    pub header: Header,
    #[serde(default)]
    pub constants: Vec<Constant>,
    pub metrics: Vec<Metric>,
}

fn main() {
    let x = include_str!("../../../extern/perfmon/SKL/events/skylake_core.json");
    let e: Events = serde_json::from_str(x).unwrap();

    let y = include_str!("../../../extern/perfmon/SKL/metrics/skylake_metrics.json");
    let m: Metrics = serde_json::from_str(y).unwrap();

    dbg!(m);
}
