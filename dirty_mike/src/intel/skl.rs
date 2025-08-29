use crate::Counter;

#[derive(Debug, Counter)]
pub struct BottleneckMispredictions {
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckBigCode {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckInstructionFetchBw {
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckCacheMemoryBandwidth {
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckCacheMemoryLatency {
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckMemoryDataTlBs {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckMemorySynchronization {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts the number of memory ordering Machine Clears detected. Memory Ordering Machine Clears can result from one of the following:a. memory disambiguation,b. external snoop, orc. cross SMT-HW-thread snoop (stores) hitting load buffer."]
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckComputeBoundEst {
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "This event counts cycles during which the microcode scoreboard stalls happen."]
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles at least 2 micro-op is executed from any thread on physical core."]
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
    #[doc = "Cycles at least 3 micro-op is executed from any thread on physical core."]
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Cycles at least 1 micro-op is executed from any thread on physical core."]
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckIrregularOverhead {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the number of memory ordering Machine Clears detected. Memory Ordering Machine Clears can result from one of the following:a. memory disambiguation,b. external snoop, orc. cross SMT-HW-thread snoop (stores) hitting load buffer."]
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "This event counts cycles during which the microcode scoreboard stalls happen."]
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts cycles during which the reservation station (RS) is empty for the thread.; Note: In ST-mode, not active thread should drive 0. This is usually caused by severely costly branch mispredictions, or allocator/FE issues."]
    #[raw(0x15e)]
    pub rs_events_empty_cycles: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckOtherBottlenecks {
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Counts the number of memory ordering Machine Clears detected. Memory Ordering Machine Clears can result from one of the following:a. memory disambiguation,b. external snoop, orc. cross SMT-HW-thread snoop (stores) hitting load buffer."]
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts cycles during which the reservation station (RS) is empty for the thread.; Note: In ST-mode, not active thread should drive 0. This is usually caused by severely costly branch mispredictions, or allocator/FE issues."]
    #[raw(0x15e)]
    pub rs_events_empty_cycles: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Number of all retired NOP instructions."]
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Cycles at least 1 micro-op is executed from any thread on physical core."]
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[doc = "Cycles at least 3 micro-op is executed from any thread on physical core."]
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "This event counts cycles during which the microcode scoreboard stalls happen."]
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Cycles at least 2 micro-op is executed from any thread on physical core."]
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckBranchingOverhead {
    #[doc = "Number of all retired NOP instructions."]
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckUsefulWork {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Number of all retired NOP instructions."]
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct FrontendBound {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct FetchLatency {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
}
#[derive(Debug, Counter)]
pub struct ICacheMisses {
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct ItlbMisses {
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct CodeStlbHit {
    #[doc = "Cycles when at least one PMH is busy with a page walk for code (instruction fetch) request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
}
#[derive(Debug, Counter)]
pub struct CodeStlbMiss {
    #[doc = "Cycles when at least one PMH is busy with a page walk for code (instruction fetch) request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct CodeStlbMiss4k {
    #[doc = "Cycles when at least one PMH is busy with a page walk for code (instruction fetch) request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
    #[doc = "Counts completed page walks (2M/4M page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x485)]
    pub itlb_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts completed page walks (4K page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x285)]
    pub itlb_misses_walk_completed_4k: u64,
}
#[derive(Debug, Counter)]
pub struct CodeStlbMiss2m {
    #[doc = "Counts completed page walks (2M/4M page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x485)]
    pub itlb_misses_walk_completed_2m_4m: u64,
    #[doc = "Cycles when at least one PMH is busy with a page walk for code (instruction fetch) request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
    #[doc = "Counts completed page walks (4K page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x285)]
    pub itlb_misses_walk_completed_4k: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct BranchResteers {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
}
#[derive(Debug, Counter)]
pub struct MispredictsResteers {
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct ClearsResteers {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct UnknownBranches {
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct MsSwitches {
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Lcp {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
}
#[derive(Debug, Counter)]
pub struct DsbSwitches {
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct FetchBandwidth {
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Mite {
    #[doc = "Counts the number of cycles uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x1002479)]
    pub idq_all_mite_cycles_any_uops: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of cycles 4 uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x4002479)]
    pub idq_all_mite_cycles_4_uops: u64,
}
#[derive(Debug, Counter)]
pub struct Decoder0Alone {
    #[doc = "Number of decoders utilized in a cycle when the MITE (legacy decode pipeline) fetches instructions."]
    #[raw(0x155)]
    pub inst_decoded_decoders_c1: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of decoders utilized in a cycle when the MITE (legacy decode pipeline) fetches instructions."]
    #[raw(0x155)]
    pub inst_decoded_decoders_c2: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct Dsb {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of cycles 4 or more uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_4_UOPS]"]
    #[raw(0x4001879)]
    pub idq_dsb_cycles_ok: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of cycles uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_ANY_UOPS]"]
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
}
#[derive(Debug, Counter)]
pub struct BadSpeculation {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
}
#[derive(Debug, Counter)]
pub struct BranchMispredicts {
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
}
#[derive(Debug, Counter)]
pub struct OtherMispredicts {
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Counter)]
pub struct MachineClears {
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct OtherNukes {
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of memory ordering Machine Clears detected. Memory Ordering Machine Clears can result from one of the following:a. memory disambiguation,b. external snoop, orc. cross SMT-HW-thread snoop (stores) hitting load buffer."]
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
}
#[derive(Debug, Counter)]
pub struct BackendBound {
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Counter)]
pub struct MemoryBound {
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
}
#[derive(Debug, Counter)]
pub struct L1Bound {
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct DtlbLoad {
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbHit {
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbMiss {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbMiss4k {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x408)]
    pub dtlb_load_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x208)]
    pub dtlb_load_misses_walk_completed_4k: u64,
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x808)]
    pub dtlb_load_misses_walk_completed_1g: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbMiss2m {
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x408)]
    pub dtlb_load_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x808)]
    pub dtlb_load_misses_walk_completed_1g: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x208)]
    pub dtlb_load_misses_walk_completed_4k: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbMiss1g {
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x408)]
    pub dtlb_load_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x808)]
    pub dtlb_load_misses_walk_completed_1g: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x208)]
    pub dtlb_load_misses_walk_completed_4k: u64,
}
#[derive(Debug, Counter)]
pub struct StoreFwdBlk {
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct L1LatencyDependency {
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct LockLatency {
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
}
#[derive(Debug, Counter)]
pub struct SplitLoads {
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
}
#[derive(Debug, Counter)]
pub struct _4kAliasing {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
}
#[derive(Debug, Counter)]
pub struct FbFull {
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
}
#[derive(Debug, Counter)]
pub struct L2Bound {
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
}
#[derive(Debug, Counter)]
pub struct L2HitLatency {
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
}
#[derive(Debug, Counter)]
pub struct L3Bound {
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct ContestedAccesses {
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
}
#[derive(Debug, Counter)]
pub struct DataSharing {
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
}
#[derive(Debug, Counter)]
pub struct L3HitLatency {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
}
#[derive(Debug, Counter)]
pub struct SqFull {
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct DramBound {
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
}
#[derive(Debug, Counter)]
pub struct MemBandwidth {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
}
#[derive(Debug, Counter)]
pub struct MemLatency {
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct StoreBound {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
}
#[derive(Debug, Counter)]
pub struct StoreLatency {
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct FalseSharing {
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct SplitStores {
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct DtlbStore {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbHit {
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbMiss {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbMiss4k {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x249)]
    pub dtlb_store_misses_walk_completed_4k: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x849)]
    pub dtlb_store_misses_walk_completed_1g: u64,
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x449)]
    pub dtlb_store_misses_walk_completed_2m_4m: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbMiss2m {
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x849)]
    pub dtlb_store_misses_walk_completed_1g: u64,
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x449)]
    pub dtlb_store_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x249)]
    pub dtlb_store_misses_walk_completed_4k: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbMiss1g {
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x449)]
    pub dtlb_store_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x849)]
    pub dtlb_store_misses_walk_completed_1g: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x249)]
    pub dtlb_store_misses_walk_completed_4k: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct CoreBound {
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
}
#[derive(Debug, Counter)]
pub struct Divider {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
}
#[derive(Debug, Counter)]
pub struct SerializingOperation {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "This event counts cycles during which the microcode scoreboard stalls happen."]
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilization {
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilized0 {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
}
#[derive(Debug, Counter)]
pub struct MixingVectors {
    #[doc = "Counts the number of Blend Uops issued by the Resource Allocation Table (RAT) to the reservation station (RS) in order to preserve upper bits of vector registers. Starting with the Skylake microarchitecture, these Blend uops are needed since every Intel SSE instruction executed in Dirty Upper State needs to preserve bits 128-255 of the destination register. For more information, refer to Mixing Intel AVX and Intel SSE Code section of the Optimization Guide."]
    #[raw(0x20e)]
    pub uops_issued_vector_width_mismatch: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilized1 {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Cycles at least 1 micro-op is executed from any thread on physical core."]
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[doc = "Cycles at least 2 micro-op is executed from any thread on physical core."]
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilized2 {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Cycles at least 3 micro-op is executed from any thread on physical core."]
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Cycles at least 2 micro-op is executed from any thread on physical core."]
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilized3m {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles at least 3 micro-op is executed from any thread on physical core."]
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
}
#[derive(Debug, Counter)]
pub struct AluOpUtilization {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 1."]
    #[raw(0x2a1)]
    pub uops_dispatched_port_port_1: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 6."]
    #[raw(0x40a1)]
    pub uops_dispatched_port_port_6: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 0."]
    #[raw(0x1a1)]
    pub uops_dispatched_port_port_0: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 5."]
    #[raw(0x20a1)]
    pub uops_dispatched_port_port_5: u64,
}
#[derive(Debug, Counter)]
pub struct Port0 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 0."]
    #[raw(0x1a1)]
    pub uops_dispatched_port_port_0: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct Port1 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 1."]
    #[raw(0x2a1)]
    pub uops_dispatched_port_port_1: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Port5 {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 5."]
    #[raw(0x20a1)]
    pub uops_dispatched_port_port_5: u64,
}
#[derive(Debug, Counter)]
pub struct Port6 {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 6."]
    #[raw(0x40a1)]
    pub uops_dispatched_port_port_6: u64,
}
#[derive(Debug, Counter)]
pub struct LoadOpUtilization {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 4."]
    #[raw(0x10a1)]
    pub uops_dispatched_port_port_4: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 2."]
    #[raw(0x4a1)]
    pub uops_dispatched_port_port_2: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 3."]
    #[raw(0x8a1)]
    pub uops_dispatched_port_port_3: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 7."]
    #[raw(0x80a1)]
    pub uops_dispatched_port_port_7: u64,
}
#[derive(Debug, Counter)]
pub struct Port2 {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 2."]
    #[raw(0x4a1)]
    pub uops_dispatched_port_port_2: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Port3 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 3."]
    #[raw(0x8a1)]
    pub uops_dispatched_port_port_3: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct StoreOpUtilization {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 4."]
    #[raw(0x10a1)]
    pub uops_dispatched_port_port_4: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Port4 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 4."]
    #[raw(0x10a1)]
    pub uops_dispatched_port_port_4: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Port7 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 7."]
    #[raw(0x80a1)]
    pub uops_dispatched_port_port_7: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Retiring {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct LightOperations {
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct FpArith {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Counts the number of x87 uops executed."]
    #[raw(0x10b1)]
    pub uops_executed_x87: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct X87Use {
    #[doc = "Counts the number of x87 uops executed."]
    #[raw(0x10b1)]
    pub uops_executed_x87: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct FpScalar {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
}
#[derive(Debug, Counter)]
pub struct FpVector {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
}
#[derive(Debug, Counter)]
pub struct FpVector128b {
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational single precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 4 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x8c7)]
    pub fp_arith_inst_retired_128b_packed_single: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct FpVector256b {
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[doc = "Counts once for most SIMD 256-bit packed double computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 4 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x10c7)]
    pub fp_arith_inst_retired_256b_packed_double: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct MemoryOperations {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts all retired memory instructions - loads and stores."]
    #[raw(0x83d0)]
    pub mem_inst_retired_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct FusedInstructions {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct NonFusedBranches {
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct OtherLightOps {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[doc = "Counts all retired memory instructions - loads and stores."]
    #[raw(0x83d0)]
    pub mem_inst_retired_any: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts the number of x87 uops executed."]
    #[raw(0x10b1)]
    pub uops_executed_x87: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct NopInstructions {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of all retired NOP instructions."]
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct HeavyOperations {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct FewUopsInstructions {
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct MicrocodeSequencer {
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Assists {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
}
#[derive(Debug, Counter)]
pub struct FpAssists {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct Cisc {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBotlnkL0CoreBoundLikely {
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core crystal clock cycles when this thread is unhalted and the other thread is halted."]
    #[raw(0x23c)]
    pub cpu_clk_unhalted_one_thread_active: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Core crystal clock cycles when at least one thread on the physical core is unhalted."]
    #[raw(0x20013c)]
    pub cpu_clk_unhalted_ref_xclk_any: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadIpc {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadUopPi {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadUpTb {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadCpi {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadClks {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadSlots {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadExecutePerIssue {
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreCoreIpc {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreFloPc {
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[doc = "Number of SSE/AVX computational 128-bit packed single precision and 256-bit packed double precision  floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 or/and 4 computation operations, one for each element.  Applies to SSE* and AVX* packed single precision floating-point and packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX RCP14 RSQRT14 SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x18c7)]
    pub fp_arith_inst_retired_4_flops: u64,
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreFpArithUtilization {
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreIlp {
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread_c1: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreEpc {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreCoreClks {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpLoad {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpStore {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpBranch {
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpCall {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpTb {
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixBpTkBranch {
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpFlop {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Number of SSE/AVX computational 128-bit packed single precision and 256-bit packed double precision  floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 or/and 4 computation operations, one for each element.  Applies to SSE* and AVX* packed single precision floating-point and packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX RCP14 RSQRT14 SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x18c7)]
    pub fp_arith_inst_retired_4_flops: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArith {
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArithScalarSp {
    #[doc = "Counts once for most SIMD scalar computational single precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x2c7)]
    pub fp_arith_inst_retired_scalar_single: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArithScalarDp {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts once for most SIMD scalar computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar double precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x1c7)]
    pub fp_arith_inst_retired_scalar_double: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArithAvx128 {
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational single precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 4 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x8c7)]
    pub fp_arith_inst_retired_128b_packed_single: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArithAvx256 {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[doc = "Counts once for most SIMD 256-bit packed double computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 4 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x10c7)]
    pub fp_arith_inst_retired_256b_packed_double: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpSwpf {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of PREFETCHNTA, PREFETCHW, PREFETCHT0, PREFETCHT1 or PREFETCHT2 instructions executed."]
    #[raw(0xf32)]
    pub sw_prefetch_access_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixInstructions {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineRetire {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots_c1: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineIpAssist {
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineExecute {
    #[doc = "Cycles at least 1 micro-op is executed from any thread on physical core."]
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread_c1: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineFetchDsb {
    #[doc = "Counts the number of uops delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Counting includes uops that may 'bypass' the IDQ."]
    #[raw(0x879)]
    pub idq_dsb_uops: u64,
    #[doc = "Counts the number of cycles uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_ANY_UOPS]"]
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineFetchMite {
    #[doc = "Counts cycles during which uops are being delivered to Instruction Decode Queue (IDQ) from the MITE path. Counting includes uops that may 'bypass' the IDQ."]
    #[raw(0x1000479)]
    pub idq_mite_cycles: u64,
    #[doc = "Counts the number of uops delivered to Instruction Decode Queue (IDQ) from the MITE path. Counting includes uops that may 'bypass' the IDQ. This also means that uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x479)]
    pub idq_mite_uops: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendFetchUpC {
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any_c1: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendDsbCoverage {
    #[doc = "Counts the number of uops delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Counting includes uops that may 'bypass' the IDQ."]
    #[raw(0x879)]
    pub idq_dsb_uops: u64,
    #[doc = "Counts the number of uops delivered to Instruction Decode Queue (IDQ) from the MITE path. Counting includes uops that may 'bypass' the IDQ. This also means that uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x479)]
    pub idq_mite_uops: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendDsbSwitchCost {
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "This event counts the number of the Decode Stream Buffer (DSB)-to-MITE switches including all misses because of missing Decode Stream Buffer (DSB) cache and u-arch forced misses. Note: Invoking MITE requires two or three cycles delay."]
    #[raw(0x1ab)]
    pub dsb2mite_switches_count: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendTBpC {
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendICacheMissLatency {
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendIpDsbMissRet {
    #[doc = "Counts retired Instructions that experienced DSB (Decode stream buffer i.e. the decoded instruction-cache) miss."]
    #[raw(0x1c6)]
    pub frontend_retired_any_dsb_miss: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendIpUnknownBranch {
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendL2mpkiCode {
    #[doc = "Retired Instructions who experienced Instruction L2 Cache true miss."]
    #[raw(0x1c6)]
    pub frontend_retired_l2_miss: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendL2mpkiCodeAll {
    #[doc = "Counts L2 cache misses when fetching instructions."]
    #[raw(0x2424)]
    pub l2_rqsts_code_rd_miss: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBotlnkL2DsbMisses {
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts the number of cycles 4 uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x4002479)]
    pub idq_all_mite_cycles_4_uops: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Counts the number of cycles 4 or more uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_4_UOPS]"]
    #[raw(0x4001879)]
    pub idq_dsb_cycles_ok: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the number of cycles uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x1002479)]
    pub idq_all_mite_cycles_any_uops: u64,
    #[doc = "Counts the number of cycles uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_ANY_UOPS]"]
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBotlnkL2DsbBandwidth {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts the number of cycles uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_ANY_UOPS]"]
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts the number of cycles uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x1002479)]
    pub idq_all_mite_cycles_any_uops: u64,
    #[doc = "Counts the number of cycles 4 uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x4002479)]
    pub idq_all_mite_cycles_4_uops: u64,
    #[doc = "Counts the number of cycles 4 or more uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_4_UOPS]"]
    #[raw(0x4001879)]
    pub idq_dsb_cycles_ok: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBotlnkL2IcMisses {
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBadSpecIpMispredict {
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBadSpecIpMispIndirect {
    #[doc = "Counts speculatively miss-predicted indirect branches at execution time. Counts for indirect near CALL or JMP instructions (RET excluded)."]
    #[raw(0xe489)]
    pub br_misp_exec_indirect: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBadSpecBranchMispredictionCost {
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Core cycles the allocator was stalled due to recovery from earlier clear event for any thread running on the physical core (e.g. misprediction or memory nuke)."]
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBadSpecSpecClearsRatio {
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBranchesCondNt {
    #[doc = "This event counts not taken branch instructions retired."]
    #[raw(0x10c4)]
    pub br_inst_retired_not_taken: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBranchesCondTk {
    #[doc = "This event counts not taken branch instructions retired."]
    #[raw(0x10c4)]
    pub br_inst_retired_not_taken: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "This event counts conditional branch instructions retired. [This event is alias to BR_INST_RETIRED.COND]"]
    #[raw(0x1c4)]
    pub br_inst_retired_conditional: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBranchesCallRet {
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "This event counts return instructions retired."]
    #[raw(0x8c4)]
    pub br_inst_retired_near_return: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBranchesJump {
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "This event counts not taken branch instructions retired."]
    #[raw(0x10c4)]
    pub br_inst_retired_not_taken: u64,
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
    #[doc = "This event counts conditional branch instructions retired. [This event is alias to BR_INST_RETIRED.CONDITIONAL]"]
    #[raw(0x1c4)]
    pub br_inst_retired_cond: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryLoadMissRealLatency {
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryMlp {
    #[doc = "Counts duration of L1D miss outstanding in cycles."]
    #[raw(0x1000148)]
    pub l1d_pend_miss_pending_cycles: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL1mpki {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL1mpkiLoad {
    #[doc = "Counts the number of demand Data Read requests (including requests from L1D hardware prefetchers). These loads may hit or miss L2 cache. Only non rejected loads are counted."]
    #[raw(0xe124)]
    pub l2_rqsts_all_demand_data_rd: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2mpki {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Retired load instructions missed L2 cache as data sources."]
    #[raw(0x10d1)]
    pub mem_load_retired_l2_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2mpkiAll {
    #[doc = "All requests that miss L2 cache."]
    #[raw(0x3f24)]
    pub l2_rqsts_miss: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2mpkiLoad {
    #[doc = "Counts the number of demand Data Read requests that miss L2 cache. Only not rejected loads are counted."]
    #[raw(0x2124)]
    pub l2_rqsts_demand_data_rd_miss: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2mpkiRfo {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts the demand RFO (read for ownership) requests including regular RFOs, locks, ItoM."]
    #[raw(0x4b0)]
    pub offcore_requests_demand_rfo: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2hpkiAll {
    #[doc = "All L2 requests."]
    #[raw(0xff24)]
    pub l2_rqsts_references: u64,
    #[doc = "All requests that miss L2 cache."]
    #[raw(0x3f24)]
    pub l2_rqsts_miss: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2hpkiLoad {
    #[doc = "Counts the number of demand Data Read requests, initiated by load instructions, that hit L2 cache"]
    #[raw(0xc124)]
    pub l2_rqsts_demand_data_rd_hit: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL3mpki {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L3 cache."]
    #[raw(0x20d1)]
    pub mem_load_retired_l3_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryFbHpki {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL1dCacheFillBw {
    #[doc = "Counts L1D data line replacements including opportunistic replacements, and replacements that require stall-for-replace or block-for-replace."]
    #[raw(0x151)]
    pub l1d_replacement: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2CacheFillBw {
    #[doc = "Counts the number of L2 cache lines filling the L2. Counting does not cover rejects."]
    #[raw(0x1ff1)]
    pub l2_lines_in_all: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL3CacheFillBw {
    #[doc = "Counts core-originated cacheable requests that miss the L3 cache (Longest Latency cache). Requests include data and code reads, Reads-for-Ownership (RFOs), speculative accesses and hardware prefetches from L1 and L2. It does not include all misses to the L3."]
    #[raw(0x412e)]
    pub longest_lat_cache_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL3CacheAccessBw {
    #[doc = "Counts memory transactions reached the super queue including requests initiated by the core, all L3 prefetches, page walks, etc.."]
    #[raw(0x80b0)]
    pub offcore_requests_all_requests: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryTlbPageWalksUtilization {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts 1 per cycle for each PMH that is busy with a page walk for a store. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1049)]
    pub dtlb_store_misses_walk_pending: u64,
    #[doc = "Counts cycles for each PMH (Page Miss Handler) that is busy with an EPT (Extended Page Table) walk for any request type."]
    #[raw(0x104f)]
    pub ept_walk_pending: u64,
    #[doc = "Counts 1 per cycle for each PMH (Page Miss Handler) that is busy with a page walk for an instruction fetch request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1085)]
    pub itlb_misses_walk_pending: u64,
    #[doc = "Counts 1 per cycle for each PMH that is busy with a page walk for a load. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1008)]
    pub dtlb_load_misses_walk_pending: u64,
    #[doc = "Core cycles when at least one thread on the physical core is not in halt state."]
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryTlbCodeStlbMpki {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "Counts completed page walks (all page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0xe85)]
    pub itlb_misses_walk_completed: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryTlbLoadStlbMpki {
    #[doc = "Counts completed page walks  (all page sizes) caused by demand data loads. This implies it missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0xe08)]
    pub dtlb_load_misses_walk_completed: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryTlbStoreStlbMpki {
    #[doc = "Counts completed page walks  (all page sizes) caused by demand data stores. This implies it missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0xe49)]
    pub dtlb_store_misses_walk_completed: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryCoreL1dCacheFillBw2t {
    #[doc = "Counts L1D data line replacements including opportunistic replacements, and replacements that require stall-for-replace or block-for-replace."]
    #[raw(0x151)]
    pub l1d_replacement: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryCoreL2CacheFillBw2t {
    #[doc = "Counts the number of L2 cache lines filling the L2. Counting does not cover rejects."]
    #[raw(0x1ff1)]
    pub l2_lines_in_all: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryCoreL3CacheFillBw2t {
    #[doc = "Counts core-originated cacheable requests that miss the L3 cache (Longest Latency cache). Requests include data and code reads, Reads-for-Ownership (RFOs), speculative accesses and hardware prefetches from L1 and L2. It does not include all misses to the L3."]
    #[raw(0x412e)]
    pub longest_lat_cache_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryCoreL3CacheAccessBw2t {
    #[doc = "Counts memory transactions reached the super queue including requests initiated by the core, all L3 prefetches, page walks, etc.."]
    #[raw(0x80b0)]
    pub offcore_requests_all_requests: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryLatencyLoadL2MissLatency {
    #[doc = "Counts the number of offcore outstanding Demand Data Read transactions in the super queue (SQ) every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor. See the corresponding Umask under OFFCORE_REQUESTS.Note: A prefetch promoted to Demand is counted from the promotion point."]
    #[raw(0x160)]
    pub offcore_requests_outstanding_demand_data_rd: u64,
    #[doc = "Counts the Demand Data Read requests sent to uncore. Use it in conjunction with OFFCORE_REQUESTS_OUTSTANDING to determine average latency in the uncore."]
    #[raw(0x1b0)]
    pub offcore_requests_demand_data_rd: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryLatencyLoadL2Mlp {
    #[doc = "Counts cycles when offcore outstanding Demand Data Read transactions are present in the super queue (SQ). A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation)."]
    #[raw(0x1000160)]
    pub offcore_requests_outstanding_cycles_with_demand_data_rd: u64,
    #[doc = "Counts the number of offcore outstanding Demand Data Read transactions in the super queue (SQ) every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor. See the corresponding Umask under OFFCORE_REQUESTS.Note: A prefetch promoted to Demand is counted from the promotion point."]
    #[raw(0x160)]
    pub offcore_requests_outstanding_demand_data_rd: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryLatencyDataL2Mlp {
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryMixUcLoadPki {
    #[doc = "Retired instructions with at least 1 uncacheable load or lock."]
    #[raw(0x4d4)]
    pub mem_load_misc_retired_uc: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemCpuUtilization {
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemCpUsUtilized {
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemCoreFrequency {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemGfloPs {
    #[doc = "Number of SSE/AVX computational 128-bit packed single precision and 256-bit packed double precision  floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 or/and 4 computation operations, one for each element.  Applies to SSE* and AVX* packed single precision floating-point and packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX RCP14 RSQRT14 SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x18c7)]
    pub fp_arith_inst_retired_4_flops: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemTurboUtilization {
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemSmt2tUtilization {
    #[doc = "Core crystal clock cycles when at least one thread on the physical core is unhalted."]
    #[raw(0x20013c)]
    pub cpu_clk_unhalted_ref_xclk_any: u64,
    #[doc = "Core crystal clock cycles when this thread is unhalted and the other thread is halted."]
    #[raw(0x23c)]
    pub cpu_clk_unhalted_one_thread_active: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemKernelUtilization {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "This is an architectural event that counts the number of thread cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. The core frequency may change from time to time due to power or thermal throttling. For this reason, this event may have a changing ratio with regards to wall clock time."]
    #[raw(0x3c)]
    pub cpu_clk_unhalted_thread_p_sup: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemKernelCpi {
    #[doc = "This is an architectural event that counts the number of thread cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. The core frequency may change from time to time due to power or thermal throttling. For this reason, this event may have a changing ratio with regards to wall clock time."]
    #[raw(0x3c)]
    pub cpu_clk_unhalted_thread_p_sup: u64,
    #[doc = "Counts the number of instructions (EOMs) retired. Counting covers macro-fused instructions individually (that is, increments by two)."]
    #[raw(0xc0)]
    pub inst_retired_any_p_sup: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemDramBwUse {
    #[doc = "UNC_ARB_TRK_REQUESTS.ALL"]
    #[raw(0x181)]
    pub unc_arb_trk_requests_all: u64,
    #[doc = "Number of entries allocated. Account for Any type: e.g. Snoop, Core aperture, etc."]
    #[raw(0x184)]
    pub unc_arb_coh_trk_requests_all: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemMemReadLatency {
    #[doc = "This 48-bit fixed counter counts the UCLK cycles."]
    #[raw(0x100)]
    pub unc_clock_socket: u64,
    #[doc = "Number of Core coherent Data Read requests sent to memory controller whose data is returned directly to requesting agent."]
    #[raw(0x281)]
    pub unc_arb_trk_requests_data_read: u64,
    #[doc = "Number of Core Data Read entries outstanding for the memory controller. The outstanding interval starts after LLC miss till return of first data chunk."]
    #[raw(0x280)]
    pub unc_arb_trk_occupancy_data_read: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemMemParallelReads {
    #[doc = "Number of Core Data Read entries outstanding for the memory controller. The outstanding interval starts after LLC miss till return of first data chunk."]
    #[raw(0x280)]
    pub unc_arb_trk_occupancy_data_read_c1: u64,
    #[doc = "Number of Core Data Read entries outstanding for the memory controller. The outstanding interval starts after LLC miss till return of first data chunk."]
    #[raw(0x280)]
    pub unc_arb_trk_occupancy_data_read: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemTime {}
#[derive(Debug, Counter)]
pub struct InfoSystemMux {
    #[doc = "This is an architectural event that counts the number of thread cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. The core frequency may change from time to time due to power or thermal throttling. For this reason, this event may have a changing ratio with regards to wall clock time."]
    #[raw(0x3c)]
    pub cpu_clk_unhalted_thread_p: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemSocketClks {
    #[doc = "This 48-bit fixed counter counts the UCLK cycles."]
    #[raw(0x100)]
    pub unc_clock_socket: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemIpFarBranch {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[doc = "This event counts far branch instructions retired."]
    #[raw(0x40c4)]
    pub br_inst_retired_far_branch_user: u64,
}
