#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use crate::Counter;

#[derive(Debug, Counter)]
pub struct Bottleneck_Mispredictions {
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x1003079)]
    IDQ__MS_SWITCHES: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x187)]
    DECODE__LCP: u64,
    #[raw(0x100010d)]
    INT_MISC__CLEARS_COUNT: u64,
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Big_Code {
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
    #[raw(0x1003079)]
    IDQ__MS_SWITCHES: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x187)]
    DECODE__LCP: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Instruction_Fetch_BW {
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x1001eca)]
    FP_ASSIST__ANY: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x100010d)]
    INT_MISC__CLEARS_COUNT: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0x3fc1)]
    OTHER_ASSISTS__ANY: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x187)]
    DECODE__LCP: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
    #[raw(0x1003079)]
    IDQ__MS_SWITCHES: u64,
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Cache_Memory_Bandwidth {
    #[raw(0x2d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HIT: u64,
    #[raw(0x60006a3)]
    CYCLE_ACTIVITY__STALLS_L3_MISS: u64,
    #[raw(0xc000ca3)]
    CYCLE_ACTIVITY__STALLS_L1D_MISS: u64,
    #[raw(0x50005a3)]
    CYCLE_ACTIVITY__STALLS_L2_MISS: u64,
    #[raw(0x107)]
    LD_BLOCKS_PARTIAL__ADDRESS_ALIAS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x2008)]
    DTLB_LOAD_MISSES__STLB_HIT_c1: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x860)]
    OFFCORE_REQUESTS_OUTSTANDING__ALL_DATA_RD_c4: u64,
    #[raw(0x248)]
    L1D_PEND_MISS__FB_FULL_c1: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x803)]
    LD_BLOCKS__NO_SR: u64,
    #[raw(0x4d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HITM: u64,
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0xc224)]
    L2_RQSTS__RFO_HIT: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x82d0)]
    MEM_INST_RETIRED__ALL_STORES: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x80008a3)]
    CYCLE_ACTIVITY__CYCLES_L1D_MISS: u64,
    #[raw(0x1000460)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DEMAND_RFO: u64,
    #[raw(0x203)]
    LD_BLOCKS__STORE_FORWARD: u64,
    #[raw(0x148)]
    L1D_PEND_MISS__PENDING: u64,
    #[raw(0x21d0)]
    MEM_INST_RETIRED__LOCK_LOADS: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x100010a3)]
    CYCLE_ACTIVITY__CYCLES_MEM_ANY: u64,
    #[raw(0x1d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_MISS: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
    #[raw(0x81d0)]
    MEM_INST_RETIRED__ALL_LOADS: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x2d1)]
    MEM_LOAD_RETIRED__L2_HIT: u64,
    #[raw(0x1b2)]
    OFFCORE_REQUESTS_BUFFER__SQ_FULL: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x4d1)]
    MEM_LOAD_RETIRED__L3_HIT: u64,
    #[raw(0x1000860)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DATA_RD: u64,
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0xe224)]
    L2_RQSTS__ALL_RFO: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Cache_Memory_Latency {
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x82d0)]
    MEM_INST_RETIRED__ALL_STORES: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x42d0)]
    MEM_INST_RETIRED__SPLIT_STORES: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x81d0)]
    MEM_INST_RETIRED__ALL_LOADS: u64,
    #[raw(0x1b2)]
    OFFCORE_REQUESTS_BUFFER__SQ_FULL: u64,
    #[raw(0x60006a3)]
    CYCLE_ACTIVITY__STALLS_L3_MISS: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x107)]
    LD_BLOCKS_PARTIAL__ADDRESS_ALIAS: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0xc000ca3)]
    CYCLE_ACTIVITY__STALLS_L1D_MISS: u64,
    #[raw(0x860)]
    OFFCORE_REQUESTS_OUTSTANDING__ALL_DATA_RD_c4: u64,
    #[raw(0x80008a3)]
    CYCLE_ACTIVITY__CYCLES_L1D_MISS: u64,
    #[raw(0x1000460)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DEMAND_RFO: u64,
    #[raw(0x2049)]
    DTLB_STORE_MISSES__STLB_HIT_c1: u64,
    #[raw(0x1000860)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DATA_RD: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
    #[raw(0x50005a3)]
    CYCLE_ACTIVITY__STALLS_L2_MISS: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x21d0)]
    MEM_INST_RETIRED__LOCK_LOADS: u64,
    #[raw(0x4d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HITM: u64,
    #[raw(0x1d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_MISS: u64,
    #[raw(0x148)]
    L1D_PEND_MISS__PENDING: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0xe224)]
    L2_RQSTS__ALL_RFO: u64,
    #[raw(0x2d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HIT: u64,
    #[raw(0xc224)]
    L2_RQSTS__RFO_HIT: u64,
    #[raw(0x803)]
    LD_BLOCKS__NO_SR: u64,
    #[raw(0x203)]
    LD_BLOCKS__STORE_FORWARD: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x2d1)]
    MEM_LOAD_RETIRED__L2_HIT: u64,
    #[raw(0x248)]
    L1D_PEND_MISS__FB_FULL_c1: u64,
    #[raw(0x1b7)]
    OFFCORE_RESPONSE__DEMAND_RFO__L3_HIT__SNOOP_HITM: u64,
    #[raw(0x4d1)]
    MEM_LOAD_RETIRED__L3_HIT: u64,
    #[raw(0x2008)]
    DTLB_LOAD_MISSES__STLB_HIT_c1: u64,
    #[raw(0x100010a3)]
    CYCLE_ACTIVITY__CYCLES_MEM_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Memory_Data_TLBs {
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x80008a3)]
    CYCLE_ACTIVITY__CYCLES_L1D_MISS: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x42d0)]
    MEM_INST_RETIRED__SPLIT_STORES: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x2049)]
    DTLB_STORE_MISSES__STLB_HIT_c1: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0x50005a3)]
    CYCLE_ACTIVITY__STALLS_L2_MISS: u64,
    #[raw(0x60006a3)]
    CYCLE_ACTIVITY__STALLS_L3_MISS: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0x2008)]
    DTLB_LOAD_MISSES__STLB_HIT_c1: u64,
    #[raw(0x100010a3)]
    CYCLE_ACTIVITY__CYCLES_MEM_ANY: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0xc224)]
    L2_RQSTS__RFO_HIT: u64,
    #[raw(0x203)]
    LD_BLOCKS__STORE_FORWARD: u64,
    #[raw(0xc000ca3)]
    CYCLE_ACTIVITY__STALLS_L1D_MISS: u64,
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0xe224)]
    L2_RQSTS__ALL_RFO: u64,
    #[raw(0x803)]
    LD_BLOCKS__NO_SR: u64,
    #[raw(0x82d0)]
    MEM_INST_RETIRED__ALL_STORES: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
    #[raw(0x148)]
    L1D_PEND_MISS__PENDING: u64,
    #[raw(0x1b7)]
    OFFCORE_RESPONSE__DEMAND_RFO__L3_HIT__SNOOP_HITM: u64,
    #[raw(0x107)]
    LD_BLOCKS_PARTIAL__ADDRESS_ALIAS: u64,
    #[raw(0x1000460)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DEMAND_RFO: u64,
    #[raw(0x248)]
    L1D_PEND_MISS__FB_FULL_c1: u64,
    #[raw(0x21d0)]
    MEM_INST_RETIRED__LOCK_LOADS: u64,
    #[raw(0x2d1)]
    MEM_LOAD_RETIRED__L2_HIT: u64,
    #[raw(0x81d0)]
    MEM_INST_RETIRED__ALL_LOADS: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Memory_Synchronization {
    #[raw(0x2c3)]
    MACHINE_CLEARS__MEMORY_ORDERING: u64,
    #[raw(0x60006a3)]
    CYCLE_ACTIVITY__STALLS_L3_MISS: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
    #[raw(0x1d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_MISS: u64,
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x4d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HITM: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0xc224)]
    L2_RQSTS__RFO_HIT: u64,
    #[raw(0x2049)]
    DTLB_STORE_MISSES__STLB_HIT_c1: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x2d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HIT: u64,
    #[raw(0x42d0)]
    MEM_INST_RETIRED__SPLIT_STORES: u64,
    #[raw(0x1b2)]
    OFFCORE_REQUESTS_BUFFER__SQ_FULL: u64,
    #[raw(0x82d0)]
    MEM_INST_RETIRED__ALL_STORES: u64,
    #[raw(0xc000ca3)]
    CYCLE_ACTIVITY__STALLS_L1D_MISS: u64,
    #[raw(0x21d0)]
    MEM_INST_RETIRED__LOCK_LOADS: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x1000460)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DEMAND_RFO: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x50005a3)]
    CYCLE_ACTIVITY__STALLS_L2_MISS: u64,
    #[raw(0x248)]
    L1D_PEND_MISS__FB_FULL_c1: u64,
    #[raw(0x4d1)]
    MEM_LOAD_RETIRED__L3_HIT: u64,
    #[raw(0x2d1)]
    MEM_LOAD_RETIRED__L2_HIT: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x1b7)]
    OFFCORE_RESPONSE__DEMAND_RFO__L3_HIT__SNOOP_HITM: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Compute_Bound_Est {
    #[raw(0x159)]
    PARTIAL_RAT_STALLS__SCOREBOARD: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x20002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_2: u64,
    #[raw(0x1000114)]
    ARITH__DIVIDER_ACTIVE: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0x30002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_3: u64,
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x10002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_1: u64,
    #[raw(0x1a6)]
    EXE_ACTIVITY__EXE_BOUND_0_PORTS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Irregular_Overhead {
    #[raw(0x187)]
    DECODE__LCP: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x100010d)]
    INT_MISC__CLEARS_COUNT: u64,
    #[raw(0x1a6)]
    EXE_ACTIVITY__EXE_BOUND_0_PORTS: u64,
    #[raw(0x15e)]
    RS_EVENTS__EMPTY_CYCLES: u64,
    #[raw(0x159)]
    PARTIAL_RAT_STALLS__SCOREBOARD: u64,
    #[raw(0x1000114)]
    ARITH__DIVIDER_ACTIVE: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x1001eca)]
    FP_ASSIST__ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x3fc1)]
    OTHER_ASSISTS__ANY: u64,
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
    #[raw(0x1003079)]
    IDQ__MS_SWITCHES: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x2c3)]
    MACHINE_CLEARS__MEMORY_ORDERING: u64,
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Other_Bottlenecks {
    #[raw(0x803)]
    LD_BLOCKS__NO_SR: u64,
    #[raw(0x60006a3)]
    CYCLE_ACTIVITY__STALLS_L3_MISS: u64,
    #[raw(0x148)]
    L1D_PEND_MISS__PENDING: u64,
    #[raw(0x80008a3)]
    CYCLE_ACTIVITY__CYCLES_L1D_MISS: u64,
    #[raw(0x1d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_MISS: u64,
    #[raw(0x20002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_2: u64,
    #[raw(0x1003079)]
    IDQ__MS_SWITCHES: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x2049)]
    DTLB_STORE_MISSES__STLB_HIT_c1: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x50005a3)]
    CYCLE_ACTIVITY__STALLS_L2_MISS: u64,
    #[raw(0x15e)]
    RS_EVENTS__EMPTY_CYCLES: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x82d0)]
    MEM_INST_RETIRED__ALL_STORES: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1001eca)]
    FP_ASSIST__ANY: u64,
    #[raw(0xe224)]
    L2_RQSTS__ALL_RFO: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x1000860)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DATA_RD: u64,
    #[raw(0x2c4)]
    BR_INST_RETIRED__NEAR_CALL: u64,
    #[raw(0x248)]
    L1D_PEND_MISS__FB_FULL_c1: u64,
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
    #[raw(0x2d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HIT: u64,
    #[raw(0x107)]
    LD_BLOCKS_PARTIAL__ADDRESS_ALIAS: u64,
    #[raw(0x10002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_1: u64,
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x159)]
    PARTIAL_RAT_STALLS__SCOREBOARD: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
    #[raw(0x4d1)]
    MEM_LOAD_RETIRED__L3_HIT: u64,
    #[raw(0x21d0)]
    MEM_INST_RETIRED__LOCK_LOADS: u64,
    #[raw(0x1a6)]
    EXE_ACTIVITY__EXE_BOUND_0_PORTS: u64,
    #[raw(0x42d0)]
    MEM_INST_RETIRED__SPLIT_STORES: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x2c0)]
    INST_RETIRED__NOP: u64,
    #[raw(0x4d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HITM: u64,
    #[raw(0x1b7)]
    OFFCORE_RESPONSE__DEMAND_RFO__L3_HIT__SNOOP_HITM: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0x187)]
    DECODE__LCP: u64,
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0x2008)]
    DTLB_LOAD_MISSES__STLB_HIT_c1: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0x2c3)]
    MACHINE_CLEARS__MEMORY_ORDERING: u64,
    #[raw(0x100010d)]
    INT_MISC__CLEARS_COUNT: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x100010a3)]
    CYCLE_ACTIVITY__CYCLES_MEM_ANY: u64,
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x1b2)]
    OFFCORE_REQUESTS_BUFFER__SQ_FULL: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
    #[raw(0x203)]
    LD_BLOCKS__STORE_FORWARD: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
    #[raw(0x1000114)]
    ARITH__DIVIDER_ACTIVE: u64,
    #[raw(0xc000ca3)]
    CYCLE_ACTIVITY__STALLS_L1D_MISS: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x81d0)]
    MEM_INST_RETIRED__ALL_LOADS: u64,
    #[raw(0x30002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_3: u64,
    #[raw(0x2d1)]
    MEM_LOAD_RETIRED__L2_HIT: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x1000460)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DEMAND_RFO: u64,
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
    #[raw(0x3fc1)]
    OTHER_ASSISTS__ANY: u64,
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x860)]
    OFFCORE_REQUESTS_OUTSTANDING__ALL_DATA_RD_c4: u64,
    #[raw(0xc224)]
    L2_RQSTS__RFO_HIT: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Branching_Overhead {
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x2c0)]
    INST_RETIRED__NOP: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x2c4)]
    BR_INST_RETIRED__NEAR_CALL: u64,
}
#[derive(Debug, Counter)]
pub struct Bottleneck_Useful_Work {
    #[raw(0x2c4)]
    BR_INST_RETIRED__NEAR_CALL: u64,
    #[raw(0x1001eca)]
    FP_ASSIST__ANY: u64,
    #[raw(0x3fc1)]
    OTHER_ASSISTS__ANY: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x2c0)]
    INST_RETIRED__NOP: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
}
#[derive(Debug, Counter)]
pub struct Frontend_Bound {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Fetch_Latency {
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct ICache_Misses {
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct ITLB_Misses {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
}
#[derive(Debug, Counter)]
pub struct Code_STLB_Hit {
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1001085)]
    ITLB_MISSES__WALK_ACTIVE: u64,
}
#[derive(Debug, Counter)]
pub struct Code_STLB_Miss {
    #[raw(0x1001085)]
    ITLB_MISSES__WALK_ACTIVE: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Code_STLB_Miss_4K {
    #[raw(0x285)]
    ITLB_MISSES__WALK_COMPLETED_4K: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x485)]
    ITLB_MISSES__WALK_COMPLETED_2M_4M: u64,
    #[raw(0x1001085)]
    ITLB_MISSES__WALK_ACTIVE: u64,
}
#[derive(Debug, Counter)]
pub struct Code_STLB_Miss_2M {
    #[raw(0x485)]
    ITLB_MISSES__WALK_COMPLETED_2M_4M: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1001085)]
    ITLB_MISSES__WALK_ACTIVE: u64,
    #[raw(0x285)]
    ITLB_MISSES__WALK_COMPLETED_4K: u64,
}
#[derive(Debug, Counter)]
pub struct Branch_Resteers {
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Mispredicts_Resteers {
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Clears_Resteers {
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
}
#[derive(Debug, Counter)]
pub struct Unknown_Branches {
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct MS_Switches {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1003079)]
    IDQ__MS_SWITCHES: u64,
}
#[derive(Debug, Counter)]
pub struct LCP {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x187)]
    DECODE__LCP: u64,
}
#[derive(Debug, Counter)]
pub struct DSB_Switches {
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Fetch_Bandwidth {
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct MITE {
    #[raw(0x1002479)]
    IDQ__ALL_MITE_CYCLES_ANY_UOPS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x4002479)]
    IDQ__ALL_MITE_CYCLES_4_UOPS: u64,
}
#[derive(Debug, Counter)]
pub struct Decoder0_Alone {
    #[raw(0x155)]
    INST_DECODED__DECODERS_c1: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x155)]
    INST_DECODED__DECODERS_c2: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct DSB {
    #[raw(0x4001879)]
    IDQ__DSB_CYCLES_OK: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1001879)]
    IDQ__DSB_CYCLES_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Bad_Speculation {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Branch_Mispredicts {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
}
#[derive(Debug, Counter)]
pub struct Other_Mispredicts {
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x100010d)]
    INT_MISC__CLEARS_COUNT: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
}
#[derive(Debug, Counter)]
pub struct Machine_Clears {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Other_Nukes {
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x2c3)]
    MACHINE_CLEARS__MEMORY_ORDERING: u64,
}
#[derive(Debug, Counter)]
pub struct Backend_Bound {
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Memory_Bound {
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
}
#[derive(Debug, Counter)]
pub struct L1_Bound {
    #[raw(0xc000ca3)]
    CYCLE_ACTIVITY__STALLS_L1D_MISS: u64,
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct DTLB_Load {
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
    #[raw(0x2008)]
    DTLB_LOAD_MISSES__STLB_HIT_c1: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x80008a3)]
    CYCLE_ACTIVITY__CYCLES_L1D_MISS: u64,
    #[raw(0x100010a3)]
    CYCLE_ACTIVITY__CYCLES_MEM_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Load_STLB_Hit {
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
    #[raw(0x2008)]
    DTLB_LOAD_MISSES__STLB_HIT_c1: u64,
    #[raw(0x100010a3)]
    CYCLE_ACTIVITY__CYCLES_MEM_ANY: u64,
    #[raw(0x80008a3)]
    CYCLE_ACTIVITY__CYCLES_L1D_MISS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Load_STLB_Miss {
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Load_STLB_Miss_4K {
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x208)]
    DTLB_LOAD_MISSES__WALK_COMPLETED_4K: u64,
    #[raw(0x408)]
    DTLB_LOAD_MISSES__WALK_COMPLETED_2M_4M: u64,
    #[raw(0x808)]
    DTLB_LOAD_MISSES__WALK_COMPLETED_1G: u64,
}
#[derive(Debug, Counter)]
pub struct Load_STLB_Miss_2M {
    #[raw(0x408)]
    DTLB_LOAD_MISSES__WALK_COMPLETED_2M_4M: u64,
    #[raw(0x808)]
    DTLB_LOAD_MISSES__WALK_COMPLETED_1G: u64,
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x208)]
    DTLB_LOAD_MISSES__WALK_COMPLETED_4K: u64,
}
#[derive(Debug, Counter)]
pub struct Load_STLB_Miss_1G {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x408)]
    DTLB_LOAD_MISSES__WALK_COMPLETED_2M_4M: u64,
    #[raw(0x208)]
    DTLB_LOAD_MISSES__WALK_COMPLETED_4K: u64,
    #[raw(0x808)]
    DTLB_LOAD_MISSES__WALK_COMPLETED_1G: u64,
    #[raw(0x1001008)]
    DTLB_LOAD_MISSES__WALK_ACTIVE: u64,
}
#[derive(Debug, Counter)]
pub struct Store_Fwd_Blk {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x203)]
    LD_BLOCKS__STORE_FORWARD: u64,
}
#[derive(Debug, Counter)]
pub struct L1_Latency_Dependency {
    #[raw(0x81d0)]
    MEM_INST_RETIRED__ALL_LOADS: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x100010a3)]
    CYCLE_ACTIVITY__CYCLES_MEM_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x80008a3)]
    CYCLE_ACTIVITY__CYCLES_L1D_MISS: u64,
}
#[derive(Debug, Counter)]
pub struct Lock_Latency {
    #[raw(0xe224)]
    L2_RQSTS__ALL_RFO: u64,
    #[raw(0x82d0)]
    MEM_INST_RETIRED__ALL_STORES: u64,
    #[raw(0x1000460)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DEMAND_RFO: u64,
    #[raw(0x21d0)]
    MEM_INST_RETIRED__LOCK_LOADS: u64,
    #[raw(0xc224)]
    L2_RQSTS__RFO_HIT: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Split_Loads {
    #[raw(0x803)]
    LD_BLOCKS__NO_SR: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x148)]
    L1D_PEND_MISS__PENDING: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct _4K_Aliasing {
    #[raw(0x107)]
    LD_BLOCKS_PARTIAL__ADDRESS_ALIAS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct FB_Full {
    #[raw(0x148)]
    L1D_PEND_MISS__PENDING: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x248)]
    L1D_PEND_MISS__FB_FULL_c1: u64,
}
#[derive(Debug, Counter)]
pub struct L2_Bound {
    #[raw(0x50005a3)]
    CYCLE_ACTIVITY__STALLS_L2_MISS: u64,
    #[raw(0xc000ca3)]
    CYCLE_ACTIVITY__STALLS_L1D_MISS: u64,
    #[raw(0x248)]
    L1D_PEND_MISS__FB_FULL_c1: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x2d1)]
    MEM_LOAD_RETIRED__L2_HIT: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct L2_Hit_Latency {
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x2d1)]
    MEM_LOAD_RETIRED__L2_HIT: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
}
#[derive(Debug, Counter)]
pub struct L3_Bound {
    #[raw(0x60006a3)]
    CYCLE_ACTIVITY__STALLS_L3_MISS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x50005a3)]
    CYCLE_ACTIVITY__STALLS_L2_MISS: u64,
}
#[derive(Debug, Counter)]
pub struct Contested_Accesses {
    #[raw(0x1d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_MISS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x4d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HITM: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
}
#[derive(Debug, Counter)]
pub struct Data_Sharing {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x2d2)]
    MEM_LOAD_L3_HIT_RETIRED__XSNP_HIT: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
}
#[derive(Debug, Counter)]
pub struct L3_Hit_Latency {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x4d1)]
    MEM_LOAD_RETIRED__L3_HIT: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
}
#[derive(Debug, Counter)]
pub struct SQ_Full {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1b2)]
    OFFCORE_REQUESTS_BUFFER__SQ_FULL: u64,
}
#[derive(Debug, Counter)]
pub struct DRAM_Bound {
    #[raw(0x248)]
    L1D_PEND_MISS__FB_FULL_c1: u64,
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x50005a3)]
    CYCLE_ACTIVITY__STALLS_L2_MISS: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x60006a3)]
    CYCLE_ACTIVITY__STALLS_L3_MISS: u64,
    #[raw(0xc000ca3)]
    CYCLE_ACTIVITY__STALLS_L1D_MISS: u64,
    #[raw(0x2d1)]
    MEM_LOAD_RETIRED__L2_HIT: u64,
}
#[derive(Debug, Counter)]
pub struct MEM_Bandwidth {
    #[raw(0x860)]
    OFFCORE_REQUESTS_OUTSTANDING__ALL_DATA_RD_c4: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct MEM_Latency {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1000860)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DATA_RD: u64,
    #[raw(0x860)]
    OFFCORE_REQUESTS_OUTSTANDING__ALL_DATA_RD_c4: u64,
}
#[derive(Debug, Counter)]
pub struct Store_Bound {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
}
#[derive(Debug, Counter)]
pub struct Store_Latency {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0xc224)]
    L2_RQSTS__RFO_HIT: u64,
    #[raw(0x1000460)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DEMAND_RFO: u64,
    #[raw(0x21d0)]
    MEM_INST_RETIRED__LOCK_LOADS: u64,
    #[raw(0x82d0)]
    MEM_INST_RETIRED__ALL_STORES: u64,
}
#[derive(Debug, Counter)]
pub struct False_Sharing {
    #[raw(0x1b7)]
    OFFCORE_RESPONSE__DEMAND_RFO__L3_HIT__SNOOP_HITM: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
}
#[derive(Debug, Counter)]
pub struct Split_Stores {
    #[raw(0x42d0)]
    MEM_INST_RETIRED__SPLIT_STORES: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct DTLB_Store {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x2049)]
    DTLB_STORE_MISSES__STLB_HIT_c1: u64,
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Store_STLB_Hit {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
    #[raw(0x2049)]
    DTLB_STORE_MISSES__STLB_HIT_c1: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Store_STLB_Miss {
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Store_STLB_Miss_4K {
    #[raw(0x249)]
    DTLB_STORE_MISSES__WALK_COMPLETED_4K: u64,
    #[raw(0x849)]
    DTLB_STORE_MISSES__WALK_COMPLETED_1G: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x449)]
    DTLB_STORE_MISSES__WALK_COMPLETED_2M_4M: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
}
#[derive(Debug, Counter)]
pub struct Store_STLB_Miss_2M {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
    #[raw(0x249)]
    DTLB_STORE_MISSES__WALK_COMPLETED_4K: u64,
    #[raw(0x449)]
    DTLB_STORE_MISSES__WALK_COMPLETED_2M_4M: u64,
    #[raw(0x849)]
    DTLB_STORE_MISSES__WALK_COMPLETED_1G: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Store_STLB_Miss_1G {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x849)]
    DTLB_STORE_MISSES__WALK_COMPLETED_1G: u64,
    #[raw(0x1001049)]
    DTLB_STORE_MISSES__WALK_ACTIVE: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x249)]
    DTLB_STORE_MISSES__WALK_COMPLETED_4K: u64,
    #[raw(0x449)]
    DTLB_STORE_MISSES__WALK_COMPLETED_2M_4M: u64,
}
#[derive(Debug, Counter)]
pub struct Core_Bound {
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
}
#[derive(Debug, Counter)]
pub struct Divider {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1000114)]
    ARITH__DIVIDER_ACTIVE: u64,
}
#[derive(Debug, Counter)]
pub struct Serializing_Operation {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x159)]
    PARTIAL_RAT_STALLS__SCOREBOARD: u64,
}
#[derive(Debug, Counter)]
pub struct Ports_Utilization {
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0x1a6)]
    EXE_ACTIVITY__EXE_BOUND_0_PORTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x1000114)]
    ARITH__DIVIDER_ACTIVE: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Ports_Utilized_0 {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1a6)]
    EXE_ACTIVITY__EXE_BOUND_0_PORTS: u64,
}
#[derive(Debug, Counter)]
pub struct Mixing_Vectors {
    #[raw(0x20e)]
    UOPS_ISSUED__VECTOR_WIDTH_MISMATCH: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Ports_Utilized_1 {
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x10002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_1: u64,
    #[raw(0x20002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_2: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Ports_Utilized_2 {
    #[raw(0x20002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_2: u64,
    #[raw(0x30002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_3: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Ports_Utilized_3m {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x30002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_3: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct ALU_Op_Utilization {
    #[raw(0x40a1)]
    UOPS_DISPATCHED_PORT__PORT_6: u64,
    #[raw(0x20a1)]
    UOPS_DISPATCHED_PORT__PORT_5: u64,
    #[raw(0x1a1)]
    UOPS_DISPATCHED_PORT__PORT_0: u64,
    #[raw(0x2a1)]
    UOPS_DISPATCHED_PORT__PORT_1: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Port_0 {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1a1)]
    UOPS_DISPATCHED_PORT__PORT_0: u64,
}
#[derive(Debug, Counter)]
pub struct Port_1 {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x2a1)]
    UOPS_DISPATCHED_PORT__PORT_1: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Port_5 {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x20a1)]
    UOPS_DISPATCHED_PORT__PORT_5: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Port_6 {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x40a1)]
    UOPS_DISPATCHED_PORT__PORT_6: u64,
}
#[derive(Debug, Counter)]
pub struct Load_Op_Utilization {
    #[raw(0x4a1)]
    UOPS_DISPATCHED_PORT__PORT_2: u64,
    #[raw(0x8a1)]
    UOPS_DISPATCHED_PORT__PORT_3: u64,
    #[raw(0x10a1)]
    UOPS_DISPATCHED_PORT__PORT_4: u64,
    #[raw(0x80a1)]
    UOPS_DISPATCHED_PORT__PORT_7: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Port_2 {
    #[raw(0x4a1)]
    UOPS_DISPATCHED_PORT__PORT_2: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Port_3 {
    #[raw(0x8a1)]
    UOPS_DISPATCHED_PORT__PORT_3: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Store_Op_Utilization {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x10a1)]
    UOPS_DISPATCHED_PORT__PORT_4: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Port_4 {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x10a1)]
    UOPS_DISPATCHED_PORT__PORT_4: u64,
}
#[derive(Debug, Counter)]
pub struct Port_7 {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x80a1)]
    UOPS_DISPATCHED_PORT__PORT_7: u64,
}
#[derive(Debug, Counter)]
pub struct Retiring {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Light_Operations {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct FP_Arith {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x10b1)]
    UOPS_EXECUTED__X87: u64,
    #[raw(0x3c7)]
    FP_ARITH_INST_RETIRED__SCALAR: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0xfcc7)]
    FP_ARITH_INST_RETIRED__VECTOR: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x1b1)]
    UOPS_EXECUTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct X87_Use {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1b1)]
    UOPS_EXECUTED__THREAD: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x10b1)]
    UOPS_EXECUTED__X87: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
}
#[derive(Debug, Counter)]
pub struct FP_Scalar {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x3c7)]
    FP_ARITH_INST_RETIRED__SCALAR: u64,
}
#[derive(Debug, Counter)]
pub struct FP_Vector {
    #[raw(0xfcc7)]
    FP_ARITH_INST_RETIRED__VECTOR: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
}
#[derive(Debug, Counter)]
pub struct FP_Vector_128b {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x4c7)]
    FP_ARITH_INST_RETIRED__128B_PACKED_DOUBLE: u64,
    #[raw(0x8c7)]
    FP_ARITH_INST_RETIRED__128B_PACKED_SINGLE: u64,
}
#[derive(Debug, Counter)]
pub struct FP_Vector_256b {
    #[raw(0x20c7)]
    FP_ARITH_INST_RETIRED__256B_PACKED_SINGLE: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x10c7)]
    FP_ARITH_INST_RETIRED__256B_PACKED_DOUBLE: u64,
}
#[derive(Debug, Counter)]
pub struct Memory_Operations {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0x83d0)]
    MEM_INST_RETIRED__ANY: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Fused_Instructions {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
}
#[derive(Debug, Counter)]
pub struct Non_Fused_Branches {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
}
#[derive(Debug, Counter)]
pub struct Other_Light_Ops {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x83d0)]
    MEM_INST_RETIRED__ANY: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x3c7)]
    FP_ARITH_INST_RETIRED__SCALAR: u64,
    #[raw(0x1b1)]
    UOPS_EXECUTED__THREAD: u64,
    #[raw(0xfcc7)]
    FP_ARITH_INST_RETIRED__VECTOR: u64,
    #[raw(0x10b1)]
    UOPS_EXECUTED__X87: u64,
}
#[derive(Debug, Counter)]
pub struct Nop_Instructions {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x2c0)]
    INST_RETIRED__NOP: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Heavy_Operations {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
}
#[derive(Debug, Counter)]
pub struct Few_Uops_Instructions {
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x4c2)]
    UOPS_RETIRED__MACRO_FUSED: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Microcode_Sequencer {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
}
#[derive(Debug, Counter)]
pub struct Assists {
    #[raw(0x1001eca)]
    FP_ASSIST__ANY: u64,
    #[raw(0x3fc1)]
    OTHER_ASSISTS__ANY: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct FP_Assists {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1001eca)]
    FP_ASSIST__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct CISC {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1001eca)]
    FP_ASSIST__ANY: u64,
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x3fc1)]
    OTHER_ASSISTS__ANY: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Botlnk_L0_Core_Bound_Likely {
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x140014a3)]
    CYCLE_ACTIVITY__STALLS_MEM_ANY: u64,
    #[raw(0x40a6)]
    EXE_ACTIVITY__BOUND_ON_STORES: u64,
    #[raw(0x4a6)]
    EXE_ACTIVITY__2_PORTS_UTIL: u64,
    #[raw(0x2a6)]
    EXE_ACTIVITY__1_PORTS_UTIL: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x20013c)]
    CPU_CLK_UNHALTED__REF_XCLK_ANY: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x23c)]
    CPU_CLK_UNHALTED__ONE_THREAD_ACTIVE: u64,
    #[raw(0x1000114)]
    ARITH__DIVIDER_ACTIVE: u64,
    #[raw(0x1a6)]
    EXE_ACTIVITY__EXE_BOUND_0_PORTS: u64,
    #[raw(0x40004a3)]
    CYCLE_ACTIVITY__STALLS_TOTAL: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Thread_IPC {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Thread_UopPI {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Thread_UpTB {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x20c4)]
    BR_INST_RETIRED__NEAR_TAKEN: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Thread_CPI {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Thread_CLKS {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Thread_SLOTS {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Thread_Execute_per_Issue {
    #[raw(0x1b1)]
    UOPS_EXECUTED__THREAD: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Core_CoreIPC {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Core_FLOPc {
    #[raw(0x3c7)]
    FP_ARITH_INST_RETIRED__SCALAR: u64,
    #[raw(0x18c7)]
    FP_ARITH_INST_RETIRED__4_FLOPS: u64,
    #[raw(0x4c7)]
    FP_ARITH_INST_RETIRED__128B_PACKED_DOUBLE: u64,
    #[raw(0x20c7)]
    FP_ARITH_INST_RETIRED__256B_PACKED_SINGLE: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Core_FP_Arith_Utilization {
    #[raw(0x3c7)]
    FP_ARITH_INST_RETIRED__SCALAR: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0xfcc7)]
    FP_ARITH_INST_RETIRED__VECTOR: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Core_ILP {
    #[raw(0x1b1)]
    UOPS_EXECUTED__THREAD: u64,
    #[raw(0x1b1)]
    UOPS_EXECUTED__THREAD_c1: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Core_EPC {
    #[raw(0x1b1)]
    UOPS_EXECUTED__THREAD: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Core_CORE_CLKS {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpLoad {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x81d0)]
    MEM_INST_RETIRED__ALL_LOADS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpStore {
    #[raw(0x82d0)]
    MEM_INST_RETIRED__ALL_STORES: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpBranch {
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpCall {
    #[raw(0x2c4)]
    BR_INST_RETIRED__NEAR_CALL: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpTB {
    #[raw(0x20c4)]
    BR_INST_RETIRED__NEAR_TAKEN: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_BpTkBranch {
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x20c4)]
    BR_INST_RETIRED__NEAR_TAKEN: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpFLOP {
    #[raw(0x3c7)]
    FP_ARITH_INST_RETIRED__SCALAR: u64,
    #[raw(0x20c7)]
    FP_ARITH_INST_RETIRED__256B_PACKED_SINGLE: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x4c7)]
    FP_ARITH_INST_RETIRED__128B_PACKED_DOUBLE: u64,
    #[raw(0x18c7)]
    FP_ARITH_INST_RETIRED__4_FLOPS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpArith {
    #[raw(0xfcc7)]
    FP_ARITH_INST_RETIRED__VECTOR: u64,
    #[raw(0x3c7)]
    FP_ARITH_INST_RETIRED__SCALAR: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpArith_Scalar_SP {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x2c7)]
    FP_ARITH_INST_RETIRED__SCALAR_SINGLE: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpArith_Scalar_DP {
    #[raw(0x1c7)]
    FP_ARITH_INST_RETIRED__SCALAR_DOUBLE: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpArith_AVX128 {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x4c7)]
    FP_ARITH_INST_RETIRED__128B_PACKED_DOUBLE: u64,
    #[raw(0x8c7)]
    FP_ARITH_INST_RETIRED__128B_PACKED_SINGLE: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpArith_AVX256 {
    #[raw(0x20c7)]
    FP_ARITH_INST_RETIRED__256B_PACKED_SINGLE: u64,
    #[raw(0x10c7)]
    FP_ARITH_INST_RETIRED__256B_PACKED_DOUBLE: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_IpSWPF {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0xf32)]
    SW_PREFETCH_ACCESS__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Inst_Mix_Instructions {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Pipeline_Retire {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS_c1: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Pipeline_IpAssist {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x1001eca)]
    FP_ASSIST__ANY: u64,
    #[raw(0x3fc1)]
    OTHER_ASSISTS__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Pipeline_Execute {
    #[raw(0x10002b1)]
    UOPS_EXECUTED__CORE_CYCLES_GE_1: u64,
    #[raw(0x1b1)]
    UOPS_EXECUTED__THREAD_c1: u64,
    #[raw(0x1b1)]
    UOPS_EXECUTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Pipeline_Fetch_DSB {
    #[raw(0x879)]
    IDQ__DSB_UOPS: u64,
    #[raw(0x1001879)]
    IDQ__DSB_CYCLES_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Pipeline_Fetch_MITE {
    #[raw(0x1000479)]
    IDQ__MITE_CYCLES: u64,
    #[raw(0x479)]
    IDQ__MITE_UOPS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Frontend_Fetch_UpC {
    #[raw(0x10e)]
    UOPS_ISSUED__ANY_c1: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Frontend_DSB_Coverage {
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
    #[raw(0x879)]
    IDQ__DSB_UOPS: u64,
    #[raw(0x479)]
    IDQ__MITE_UOPS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Frontend_DSB_Switch_Cost {
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0x1ab)]
    DSB2MITE_SWITCHES__COUNT: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Frontend_TBpC {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x20c4)]
    BR_INST_RETIRED__NEAR_TAKEN: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Frontend_ICache_Miss_Latency {
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Frontend_IpDSB_Miss_Ret {
    #[raw(0x1c6)]
    FRONTEND_RETIRED__ANY_DSB_MISS: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Frontend_IpUnknown_Branch {
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Frontend_L2MPKI_Code {
    #[raw(0x1c6)]
    FRONTEND_RETIRED__L2_MISS: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Frontend_L2MPKI_Code_All {
    #[raw(0x2424)]
    L2_RQSTS__CODE_RD_MISS: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Botlnk_L2_DSB_Misses {
    #[raw(0x1002479)]
    IDQ__ALL_MITE_CYCLES_ANY_UOPS: u64,
    #[raw(0x4001879)]
    IDQ__DSB_CYCLES_OK: u64,
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x1001879)]
    IDQ__DSB_CYCLES_ANY: u64,
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0x187)]
    DECODE__LCP: u64,
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x1003079)]
    IDQ__MS_SWITCHES: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x4002479)]
    IDQ__ALL_MITE_CYCLES_4_UOPS: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Botlnk_L2_DSB_Bandwidth {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x1002479)]
    IDQ__ALL_MITE_CYCLES_ANY_UOPS: u64,
    #[raw(0x1001879)]
    IDQ__DSB_CYCLES_ANY: u64,
    #[raw(0x4002479)]
    IDQ__ALL_MITE_CYCLES_4_UOPS: u64,
    #[raw(0x19c)]
    IDQ_UOPS_NOT_DELIVERED__CORE: u64,
    #[raw(0x4001879)]
    IDQ__DSB_CYCLES_OK: u64,
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Botlnk_L2_IC_Misses {
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x187)]
    DECODE__LCP: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x1003079)]
    IDQ__MS_SWITCHES: u64,
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Bad_Spec_IpMispredict {
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Bad_Spec_IpMisp_Indirect {
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0xe489)]
    BR_MISP_EXEC__INDIRECT: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Bad_Spec_Branch_Misprediction_Cost {
    #[raw(0x800d)]
    INT_MISC__CLEAR_RESTEER_CYCLES: u64,
    #[raw(0x10e)]
    UOPS_ISSUED__ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x2ab)]
    DSB2MITE_SWITCHES__PENALTY_CYCLES: u64,
    #[raw(0x3079)]
    IDQ__MS_UOPS: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0x100010d)]
    INT_MISC__CLEARS_COUNT: u64,
    #[raw(0x10d)]
    INT_MISC__RECOVERY_CYCLES: u64,
    #[raw(0x2c2)]
    UOPS_RETIRED__RETIRE_SLOTS: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL: u64,
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x1e6)]
    BACLEARS__ANY: u64,
    #[raw(0x1003079)]
    IDQ__MS_SWITCHES: u64,
    #[raw(0x400019c)]
    IDQ_UOPS_NOT_DELIVERED__CYCLES_0_UOPS_DELIV__CORE: u64,
    #[raw(0x187)]
    DECODE__LCP: u64,
    #[raw(0x480)]
    ICACHE_16B__IFDATA_STALL_c1_e1: u64,
    #[raw(0x20010d)]
    INT_MISC__RECOVERY_CYCLES_ANY: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x483)]
    ICACHE_TAG__STALLS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Bad_Spec_Spec_Clears_Ratio {
    #[raw(0x100010d)]
    INT_MISC__CLEARS_COUNT: u64,
    #[raw(0x10001c3)]
    MACHINE_CLEARS__COUNT: u64,
    #[raw(0xc5)]
    BR_MISP_RETIRED__ALL_BRANCHES: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Branches_Cond_NT {
    #[raw(0x10c4)]
    BR_INST_RETIRED__NOT_TAKEN: u64,
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Branches_Cond_TK {
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x10c4)]
    BR_INST_RETIRED__NOT_TAKEN: u64,
    #[raw(0x1c4)]
    BR_INST_RETIRED__CONDITIONAL: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Branches_CallRet {
    #[raw(0x2c4)]
    BR_INST_RETIRED__NEAR_CALL: u64,
    #[raw(0x8c4)]
    BR_INST_RETIRED__NEAR_RETURN: u64,
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Branches_Jump {
    #[raw(0xc4)]
    BR_INST_RETIRED__ALL_BRANCHES: u64,
    #[raw(0x2c4)]
    BR_INST_RETIRED__NEAR_CALL: u64,
    #[raw(0x1c4)]
    BR_INST_RETIRED__COND: u64,
    #[raw(0x20c4)]
    BR_INST_RETIRED__NEAR_TAKEN: u64,
    #[raw(0x10c4)]
    BR_INST_RETIRED__NOT_TAKEN: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_Load_Miss_Real_Latency {
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
    #[raw(0x148)]
    L1D_PEND_MISS__PENDING: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_MLP {
    #[raw(0x1000148)]
    L1D_PEND_MISS__PENDING_CYCLES: u64,
    #[raw(0x148)]
    L1D_PEND_MISS__PENDING: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L1MPKI {
    #[raw(0x8d1)]
    MEM_LOAD_RETIRED__L1_MISS: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L1MPKI_Load {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0xe124)]
    L2_RQSTS__ALL_DEMAND_DATA_RD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L2MPKI {
    #[raw(0x10d1)]
    MEM_LOAD_RETIRED__L2_MISS: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L2MPKI_All {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x3f24)]
    L2_RQSTS__MISS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L2MPKI_Load {
    #[raw(0x2124)]
    L2_RQSTS__DEMAND_DATA_RD_MISS: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L2MPKI_RFO {
    #[raw(0x4b0)]
    OFFCORE_REQUESTS__DEMAND_RFO: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L2HPKI_All {
    #[raw(0x3f24)]
    L2_RQSTS__MISS: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0xff24)]
    L2_RQSTS__REFERENCES: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L2HPKI_Load {
    #[raw(0xc124)]
    L2_RQSTS__DEMAND_DATA_RD_HIT: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L3MPKI {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x20d1)]
    MEM_LOAD_RETIRED__L3_MISS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_FB_HPKI {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x40d1)]
    MEM_LOAD_RETIRED__FB_HIT: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L1D_Cache_Fill_BW {
    #[raw(0x151)]
    L1D__REPLACEMENT: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L2_Cache_Fill_BW {
    #[raw(0x1ff1)]
    L2_LINES_IN__ALL: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L3_Cache_Fill_BW {
    #[raw(0x412e)]
    LONGEST_LAT_CACHE__MISS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_L3_Cache_Access_BW {
    #[raw(0x80b0)]
    OFFCORE_REQUESTS__ALL_REQUESTS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_TLB_Page_Walks_Utilization {
    #[raw(0x200200)]
    CPU_CLK_UNHALTED__THREAD_ANY: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x1085)]
    ITLB_MISSES__WALK_PENDING: u64,
    #[raw(0x1008)]
    DTLB_LOAD_MISSES__WALK_PENDING: u64,
    #[raw(0x1049)]
    DTLB_STORE_MISSES__WALK_PENDING: u64,
    #[raw(0x104f)]
    EPT__WALK_PENDING: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_TLB_Code_STLB_MPKI {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0xe85)]
    ITLB_MISSES__WALK_COMPLETED: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_TLB_Load_STLB_MPKI {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0xe08)]
    DTLB_LOAD_MISSES__WALK_COMPLETED: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_TLB_Store_STLB_MPKI {
    #[raw(0xe49)]
    DTLB_STORE_MISSES__WALK_COMPLETED: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_Core_L1D_Cache_Fill_BW_2T {
    #[raw(0x151)]
    L1D__REPLACEMENT: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_Core_L2_Cache_Fill_BW_2T {
    #[raw(0x1ff1)]
    L2_LINES_IN__ALL: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_Core_L3_Cache_Fill_BW_2T {
    #[raw(0x412e)]
    LONGEST_LAT_CACHE__MISS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_Core_L3_Cache_Access_BW_2T {
    #[raw(0x80b0)]
    OFFCORE_REQUESTS__ALL_REQUESTS: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_Latency_Load_L2_Miss_Latency {
    #[raw(0x1b0)]
    OFFCORE_REQUESTS__DEMAND_DATA_RD: u64,
    #[raw(0x160)]
    OFFCORE_REQUESTS_OUTSTANDING__DEMAND_DATA_RD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_Latency_Load_L2_MLP {
    #[raw(0x1000160)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DEMAND_DATA_RD: u64,
    #[raw(0x160)]
    OFFCORE_REQUESTS_OUTSTANDING__DEMAND_DATA_RD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_Latency_Data_L2_MLP {
    #[raw(0x1000860)]
    OFFCORE_REQUESTS_OUTSTANDING__CYCLES_WITH_DATA_RD: u64,
    #[raw(0x860)]
    OFFCORE_REQUESTS_OUTSTANDING__ALL_DATA_RD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_Memory_Mix_UC_Load_PKI {
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
    #[raw(0x4d4)]
    MEM_LOAD_MISC_RETIRED__UC: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_CPU_Utilization {
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_CPUs_Utilized {
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_Core_Frequency {
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_GFLOPs {
    #[raw(0x3c7)]
    FP_ARITH_INST_RETIRED__SCALAR: u64,
    #[raw(0x18c7)]
    FP_ARITH_INST_RETIRED__4_FLOPS: u64,
    #[raw(0x4c7)]
    FP_ARITH_INST_RETIRED__128B_PACKED_DOUBLE: u64,
    #[raw(0x20c7)]
    FP_ARITH_INST_RETIRED__256B_PACKED_SINGLE: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_Turbo_Utilization {
    #[raw(0x300)]
    CPU_CLK_UNHALTED__REF_TSC: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_SMT_2T_Utilization {
    #[raw(0x23c)]
    CPU_CLK_UNHALTED__ONE_THREAD_ACTIVE: u64,
    #[raw(0x20013c)]
    CPU_CLK_UNHALTED__REF_XCLK_ANY: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_Kernel_Utilization {
    #[raw(0x3c)]
    CPU_CLK_UNHALTED__THREAD_P_SUP: u64,
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_Kernel_CPI {
    #[raw(0x3c)]
    CPU_CLK_UNHALTED__THREAD_P_SUP: u64,
    #[raw(0xc0)]
    INST_RETIRED__ANY_P_SUP: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_DRAM_BW_Use {
    #[raw(0x181)]
    UNC_ARB_TRK_REQUESTS__ALL: u64,
    #[raw(0x184)]
    UNC_ARB_COH_TRK_REQUESTS__ALL: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_MEM_Read_Latency {
    #[raw(0x280)]
    UNC_ARB_TRK_OCCUPANCY__DATA_READ: u64,
    #[raw(0x100)]
    UNC_CLOCK__SOCKET: u64,
    #[raw(0x281)]
    UNC_ARB_TRK_REQUESTS__DATA_READ: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_MEM_Parallel_Reads {
    #[raw(0x280)]
    UNC_ARB_TRK_OCCUPANCY__DATA_READ: u64,
    #[raw(0x280)]
    UNC_ARB_TRK_OCCUPANCY__DATA_READ_c1: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_Time {}
#[derive(Debug, Counter)]
pub struct Info_System_MUX {
    #[raw(0x200)]
    CPU_CLK_UNHALTED__THREAD: u64,
    #[raw(0x3c)]
    CPU_CLK_UNHALTED__THREAD_P: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_Socket_CLKS {
    #[raw(0x100)]
    UNC_CLOCK__SOCKET: u64,
}
#[derive(Debug, Counter)]
pub struct Info_System_IpFarBranch {
    #[raw(0x40c4)]
    BR_INST_RETIRED__FAR_BRANCH_USER: u64,
    #[raw(0x100)]
    INST_RETIRED__ANY: u64,
}
