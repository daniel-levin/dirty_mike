use crate::Observation;

#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of Branch Misprediction related bottlenecks"]
pub struct BottleneckMispredictions {
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of instruction fetch related bottlenecks by large code footprint programs (i-side cache; TLB and BTB misses)"]
pub struct BottleneckBigCode {
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
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
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of instruction fetch bandwidth related bottlenecks (when the front-end could not sustain operations delivery to the back-end)"]
pub struct BottleneckInstructionFetchBw {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of external Memory- or Cache-Bandwidth related bottlenecks"]
pub struct BottleneckCacheMemoryBandwidth {
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of external Memory- or Cache-Latency related bottlenecks"]
pub struct BottleneckCacheMemoryLatency {
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of Memory Address Translation related bottlenecks (data-side TLBs)"]
pub struct BottleneckMemoryDataTlBs {
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of Memory Synchronization related bottlenecks (data transfers and coherency updates across processors)"]
pub struct BottleneckMemorySynchronization {
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts the number of memory ordering Machine Clears detected. Memory Ordering Machine Clears can result from one of the following:a. memory disambiguation,b. external snoop, orc. cross SMT-HW-thread snoop (stores) hitting load buffer."]
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost when the execution is compute-bound - an estimation. Covers Core Bound when High ILP as well as when long-latency execution units are busy."]
pub struct BottleneckComputeBoundEst {
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Cycles at least 2 micro-op is executed from any thread on physical core."]
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Cycles at least 3 micro-op is executed from any thread on physical core."]
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "This event counts cycles during which the microcode scoreboard stalls happen."]
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[doc = "Cycles at least 1 micro-op is executed from any thread on physical core."]
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of irregular execution (e.g. FP-assists in HPC, Wait time with work imbalance multithreaded workloads, overhead in system services or virtualized environments)"]
pub struct BottleneckIrregularOverhead {
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts cycles during which the reservation station (RS) is empty for the thread.; Note: In ST-mode, not active thread should drive 0. This is usually caused by severely costly branch mispredictions, or allocator/FE issues."]
    #[raw(0x15e)]
    pub rs_events_empty_cycles: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "This event counts cycles during which the microcode scoreboard stalls happen."]
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts the number of memory ordering Machine Clears detected. Memory Ordering Machine Clears can result from one of the following:a. memory disambiguation,b. external snoop, orc. cross SMT-HW-thread snoop (stores) hitting load buffer."]
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of remaining bottlenecks in the back-end. Examples include data-dependencies (Core Bound when Low ILP) and other unlisted memory-related stalls."]
pub struct BottleneckOtherBottlenecks {
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Number of all retired NOP instructions."]
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts cycles during which the reservation station (RS) is empty for the thread.; Note: In ST-mode, not active thread should drive 0. This is usually caused by severely costly branch mispredictions, or allocator/FE issues."]
    #[raw(0x15e)]
    pub rs_events_empty_cycles: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Cycles at least 1 micro-op is executed from any thread on physical core."]
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Cycles at least 3 micro-op is executed from any thread on physical core."]
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "This event counts cycles during which the microcode scoreboard stalls happen."]
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts the number of memory ordering Machine Clears detected. Memory Ordering Machine Clears can result from one of the following:a. memory disambiguation,b. external snoop, orc. cross SMT-HW-thread snoop (stores) hitting load buffer."]
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Cycles at least 2 micro-op is executed from any thread on physical core."]
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of instructions used for program control-flow - a subset of the Retiring category in TMA. Examples include function calls; loops and alignments. (A lower bound)"]
pub struct BottleneckBranchingOverhead {
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "Number of all retired NOP instructions."]
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of \"useful operations\" - the portion of Retiring category not covered by Branching_Overhead nor Irregular_Overhead."]
pub struct BottleneckUsefulWork {
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Number of all retired NOP instructions."]
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
}
#[derive(Debug, Observation)]
#[doc = "This category represents fraction of slots where the processor's Frontend undersupplies its Backend. Frontend denotes the first part of the processor core responsible to fetch operations that are executed later on by the Backend part. Within the Frontend; a branch predictor predicts the next address to fetch; cache-lines are fetched from the memory subsystem; parsed into instructions; and lastly decoded into micro-operations (uops). Ideally the Frontend can issue Pipeline_Width uops every cycle to the Backend. Frontend Bound denotes unutilized issue-slots when there is no Backend stall; i.e. bubbles where Frontend delivered no uops while Backend could have accepted them. For example; stalls due to instruction-cache misses would be categorized under Frontend Bound."]
pub struct FrontendBound {
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots the CPU was stalled due to Frontend latency issues.  For example; instruction-cache misses; iTLB misses or fetch stalls after a branch misprediction are categorized under Frontend Latency. In such cases; the Frontend eventually delivers no uops for some period."]
pub struct FetchLatency {
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles the CPU was stalled due to instruction cache misses."]
pub struct ICacheMisses {
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles the CPU was stalled due to Instruction TLB (ITLB) misses."]
pub struct ItlbMisses {
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric roughly estimates the fraction of cycles where the (first level) ITLB was missed by instructions fetches, that later on hit in second-level TLB (STLB)"]
pub struct CodeStlbHit {
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Cycles when at least one PMH is busy with a page walk for code (instruction fetch) request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles where the Second-level TLB (STLB) was missed by instruction fetches, performing a hardware page walk"]
pub struct CodeStlbMiss {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles when at least one PMH is busy with a page walk for code (instruction fetch) request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles to walk the memory paging structures to cache translation of 4 KB pages for (instruction) code accesses."]
pub struct CodeStlbMiss4k {
    #[doc = "Counts completed page walks (2M/4M page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x485)]
    pub itlb_misses_walk_completed_2m_4m: u64,
    #[doc = "Cycles when at least one PMH is busy with a page walk for code (instruction fetch) request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts completed page walks (4K page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x285)]
    pub itlb_misses_walk_completed_4k: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles to walk the memory paging structures to cache translation of 2 or 4 MB pages for (instruction) code accesses."]
pub struct CodeStlbMiss2m {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles when at least one PMH is busy with a page walk for code (instruction fetch) request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
    #[doc = "Counts completed page walks (4K page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x285)]
    pub itlb_misses_walk_completed_4k: u64,
    #[doc = "Counts completed page walks (2M/4M page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x485)]
    pub itlb_misses_walk_completed_2m_4m: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles the CPU was stalled due to Branch Resteers. Branch Resteers estimates the Frontend delay in fetching operations from corrected path; following all sorts of miss-predicted branches. For example; branchy code with lots of miss-predictions might get categorized under Branch Resteers. Note the value of this node may overlap with its siblings."]
pub struct BranchResteers {
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles the CPU was stalled due to Branch Resteers as a result of Branch Misprediction at execution stage. "]
pub struct MispredictsResteers {
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles the CPU was stalled due to Branch Resteers as a result of Machine Clears. "]
pub struct ClearsResteers {
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles the CPU was stalled due to new branch address clears. These are fetched branches the Branch Prediction Unit was unable to recognize (e.g. first time the branch is fetched or hitting BTB capacity limit) hence called Unknown Branches"]
pub struct UnknownBranches {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles when the CPU was stalled due to switches of uop delivery to the Microcode Sequencer (MS). Commonly used instructions are optimized for delivery by the DSB (decoded i-cache) or MITE (legacy instruction decode) pipelines. Certain operations cannot be handled natively by the execution pipeline; and must be performed by microcode (small programs injected into the execution stream). Switching to the MS too often can negatively impact performance. The MS is designated to deliver long uop flows required by CISC instructions like CPUID; or uncommon conditions like Floating Point Assists when dealing with Denormals."]
pub struct MsSwitches {
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles CPU was stalled due to Length Changing Prefixes (LCPs). Using proper compiler flags or Intel Compiler by default will certainly avoid this. "]
pub struct Lcp {
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles the CPU was stalled due to switches from DSB to MITE pipelines. The DSB (decoded i-cache) is a Uop Cache where the front-end directly delivers Uops (micro operations) avoiding heavy x86 decoding. The DSB pipeline has shorter latency and delivered higher bandwidth than the MITE (legacy instruction decode pipeline). Switching between the two pipelines can cause penalties hence this metric measures the exposed penalty."]
pub struct DsbSwitches {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots the CPU was stalled due to Frontend bandwidth issues.  For example; inefficiencies at the instruction decoders; or restrictions for caching in the DSB (decoded uops cache) are categorized under Fetch Bandwidth. In such cases; the Frontend typically delivers suboptimal amount of uops to the Backend."]
pub struct FetchBandwidth {
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles in which CPU was likely limited due to the MITE pipeline (the legacy decode pipeline). This pipeline is used for code that was not pre-cached in the DSB or LSD. For example; inefficiencies due to asymmetric decoders; use of long immediate or LCP can manifest as MITE fetch bandwidth bottleneck."]
pub struct Mite {
    #[doc = "Counts the number of cycles uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x1002479)]
    pub idq_all_mite_cycles_any_uops: u64,
    #[doc = "Counts the number of cycles 4 uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x4002479)]
    pub idq_all_mite_cycles_4_uops: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles where decoder-0 was the only active decoder"]
pub struct Decoder0Alone {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of decoders utilized in a cycle when the MITE (legacy decode pipeline) fetches instructions."]
    #[raw(0x155)]
    pub inst_decoded_decoders_c2: u64,
    #[doc = "Number of decoders utilized in a cycle when the MITE (legacy decode pipeline) fetches instructions."]
    #[raw(0x155)]
    pub inst_decoded_decoders_c1: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles in which CPU was likely limited due to DSB (decoded uop cache) fetch pipeline.  For example; inefficient utilization of the DSB cache structure or bank conflict when reading from it; are categorized here."]
pub struct Dsb {
    #[doc = "Counts the number of cycles uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_ANY_UOPS]"]
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of cycles 4 or more uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_4_UOPS]"]
    #[raw(0x4001879)]
    pub idq_dsb_cycles_ok: u64,
}
#[derive(Debug, Observation)]
#[doc = "This category represents fraction of slots wasted due to incorrect speculations. This include slots used to issue uops that do not eventually get retired and slots for which the issue-pipeline was blocked due to recovery from earlier incorrect speculation. For example; wasted work due to miss-predicted branches are categorized under Bad Speculation category. Incorrect data speculation followed by Memory Ordering Nukes is another example."]
pub struct BadSpeculation {
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}

#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots the CPU has wasted due to Branch Misprediction.  These slots are either wasted by uops fetched from an incorrectly speculated program path; or stalls when the out-of-order part of the machine needs to recover its state from a speculative path."]
pub struct BranchMispredicts {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
}

#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of slots the CPU was stalled due to other cases of misprediction (non-retired x86 branches or other types)."]
pub struct OtherMispredicts {
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots the CPU has wasted due to Machine Clears.  These slots are either wasted by uops fetched prior to the clear; or stalls the out-of-order portion of the machine needs to recover its state after the clear. For example; this can happen due to memory ordering Nukes (e.g. Memory Disambiguation) or Self-Modifying-Code (SMC) nukes."]
pub struct MachineClears {
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots the CPU has wasted due to Nukes (Machine Clears) not related to memory ordering."]
pub struct OtherNukes {
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of memory ordering Machine Clears detected. Memory Ordering Machine Clears can result from one of the following:a. memory disambiguation,b. external snoop, orc. cross SMT-HW-thread snoop (stores) hitting load buffer."]
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "This category represents fraction of slots where no uops are being delivered due to a lack of required resources for accepting new uops in the Backend. Backend is the portion of the processor core where the out-of-order scheduler dispatches ready uops into their respective execution units; and once completed these uops get retired according to program order. For example; stalls due to data-cache misses or stalls due to the divider unit being overloaded are both categorized under Backend Bound. Backend Bound is further divided into two main categories: Memory Bound and Core Bound."]
pub struct BackendBound {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots the Memory subsystem within the Backend was a bottleneck.  Memory Bound estimates fraction of slots where pipeline is likely stalled due to demand load or store instructions. This accounts mainly for (1) non-completed in-flight memory demand loads which coincides with execution units starvation; in addition to (2) cases where stores could impose backpressure on the pipeline when many of them get buffered at the same time (less common out of the two)."]
pub struct MemoryBound {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates how often the CPU was stalled without loads missing the L1 Data (L1D) cache.  The L1D cache typically has the shortest latency.  However; in certain cases like loads blocked on older stores; a load might suffer due to high latency even though it is being satisfied by the L1D. Another example is loads who miss in the TLB. These cases are characterized by execution unit stalls; while some non-completed demand load lives in the machine without having that demand load missing the L1 cache."]
pub struct L1Bound {
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric roughly estimates the fraction of cycles where the Data TLB (DTLB) was missed by load accesses. TLBs (Translation Look-aside Buffers) are processor caches for recently used entries out of the Page Tables that are used to map virtual- to physical-addresses by the operating system. This metric approximates the potential delay of demand loads missing the first-level data TLB (assuming worst case scenario with back to back misses to different pages). This includes hitting in the second-level TLB (STLB) as well as performing a hardware page walk on an STLB miss."]
pub struct DtlbLoad {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric roughly estimates the fraction of cycles where the (first level) DTLB was missed by load accesses, that later on hit in second-level TLB (STLB)"]
pub struct LoadStlbHit {
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts loads that miss the DTLB (Data TLB) and hit the STLB (Second level TLB)."]
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles where the Second-level TLB (STLB) was missed by load accesses, performing a hardware page walk"]
pub struct LoadStlbMiss {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles to walk the memory paging structures to cache translation of 4 KB pages for data load accesses."]
pub struct LoadStlbMiss4k {
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x408)]
    pub dtlb_load_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x208)]
    pub dtlb_load_misses_walk_completed_4k: u64,
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x808)]
    pub dtlb_load_misses_walk_completed_1g: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles to walk the memory paging structures to cache translation of 2 or 4 MB pages for data load accesses."]
pub struct LoadStlbMiss2m {
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x808)]
    pub dtlb_load_misses_walk_completed_1g: u64,
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x408)]
    pub dtlb_load_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x208)]
    pub dtlb_load_misses_walk_completed_4k: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles to walk the memory paging structures to cache translation of 1 GB pages for data load accesses."]
pub struct LoadStlbMiss1g {
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a load."]
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x208)]
    pub dtlb_load_misses_walk_completed_4k: u64,
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x408)]
    pub dtlb_load_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data loads. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x808)]
    pub dtlb_load_misses_walk_completed_1g: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric roughly estimates fraction of cycles when the memory subsystem had loads blocked since they could not forward data from earlier (in program order) overlapping stores. To streamline memory operations in the pipeline; a load can avoid waiting for memory if a prior in-flight store is writing the data that the load wants to read (store forwarding process). However; in some cases the load may be blocked for a significant time pending the store forward. For example; when the prior store is writing a smaller region than the load is reading."]
pub struct StoreFwdBlk {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of times where store forwarding was prevented for a load operation. The most common case is a load blocked due to the address of memory access (partially) overlapping with a preceding uncompleted store. Note: See the table of not supported store forwards in the Optimization Guide."]
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric([SKL+] roughly; [LNL]) estimates fraction of cycles with demand load accesses that hit the L1D cache. The short latency of the L1D cache may be exposed in pointer-chasing memory access patterns as an example."]
pub struct L1LatencyDependency {
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[doc = "Cycles while L1 cache miss demand load is outstanding."]
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles while memory subsystem has an outstanding load."]
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles the CPU spent handling cache misses due to lock operations. Due to the microarchitecture handling of locks; they are classified as L1_Bound regardless of what memory source satisfied them."]
pub struct LockLatency {
    #[doc = "Counts the total number of RFO (read for ownership) requests to L2 cache. L2 RFO requests include both L1D demand RFO misses as well as L1D RFO prefetches."]
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of cycles handling memory load split accesses - load that cross 64-byte cache line boundary. "]
pub struct SplitLoads {
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "The number of times that split load operations are temporarily blocked because all resources for handling the split accesses are in use."]
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates how often memory load accesses were aliased by preceding stores (in program order) with a 4K address offset. False match is possible; which incur a few cycles load re-issue. However; the short re-issue duration is often hidden by the out-of-order core and HW optimizations; hence a user may safely ignore a high value of this metric unless it manages to propagate up into parent nodes of the hierarchy (e.g. to L1_Bound)."]
pub struct _4kAliasing {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts false dependencies in MOB when the partial comparison upon loose net check and dependency was resolved by the Enhanced Loose net mechanism. This may not result in high performance penalties. Loose net checks can fail when loads and stores are 4k aliased."]
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric does a *rough estimation* of how often L1D Fill Buffer unavailability limited additional L1D miss memory access requests to proceed. The higher the metric value; the deeper the memory hierarchy level the misses are satisfied from (metric values >1 are valid). Often it hints on approaching bandwidth limits (to L2 cache; L3 cache or external memory)."]
pub struct FbFull {
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates how often the CPU was stalled due to L2 cache accesses by loads.  Avoiding cache misses (i.e. L1 misses/L2 hits) can improve the latency and increase performance."]
pub struct L2Bound {
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles with demand load accesses that hit the L2 cache under unloaded scenarios (possibly L2 latency limited).  Avoiding L1 cache misses (i.e. L1 misses/L2 hits) will improve the latency."]
pub struct L2HitLatency {
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates how often the CPU was stalled due to loads accesses to L3 cache or contended with a sibling Core.  Avoiding cache misses (i.e. L2 misses/L3 hits) can improve the latency and increase performance."]
pub struct L3Bound {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of cycles while the memory subsystem was handling synchronizations due to contested accesses. Contested accesses occur when data written by one Logical Processor are read by another Logical Processor on a different Physical Core. Examples of contested accesses include synchronizations such as locks; true data sharing such as modified locked variables; and false sharing."]
pub struct ContestedAccesses {
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Retired load instructions which data sources were HitM responses from shared L3."]
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Retired load instructions which data sources were L3 hit and cross-core snoop missed in on-pkg core cache."]
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of cycles while the memory subsystem was handling synchronizations due to data-sharing accesses. Data shared by multiple Logical Processors (even just read shared) may cause increased access latency due to cache coherency. Excessive data sharing can drastically harm multithreaded performance."]
pub struct DataSharing {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Retired load instructions which data sources were L3 and cross-core snoop hits in on-pkg core cache."]
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of cycles with demand load accesses that hit the L3 cache under unloaded scenarios (possibly L3 latency limited).  Avoiding private cache misses (i.e. L2 misses/L3 hits) will improve the latency; reduce contention with sibling physical cores and increase performance.  Note the value of this node may overlap with its siblings."]
pub struct L3HitLatency {
    #[doc = "Counts retired load instructions with at least one uop that hit in the L3 cache."]
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric measures fraction of cycles where the Super Queue (SQ) was full taking into account all request-types and both hardware SMT threads (Logical Processors)."]
pub struct SqFull {
    #[doc = "Counts the number of cases when the offcore requests buffer cannot take more entries for the core. This can happen when the superqueue does not contain eligible entries, or when L1D writeback pending FIFO requests is full.Note: Writeback pending FIFO has six entries."]
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates how often the CPU was stalled on accesses to external memory (DRAM) by loads. Better caching can improve the latency and increase performance."]
pub struct DramBound {
    #[doc = "Retired load instructions with L2 cache hits as data sources."]
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[doc = "Execution stalls while L1 cache miss demand load is outstanding."]
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[doc = "Execution stalls while L2 cache miss demand load is outstanding."]
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Number of times a request needed a FB (Fill Buffer) entry but there was no entry available for it. A request includes cacheable/uncacheable demands that are load, store or SW prefetch instructions."]
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[doc = "Execution stalls while L3 cache miss demand load is outstanding."]
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of cycles where the core's performance was likely hurt due to approaching bandwidth limits of external memory - DRAM ([SPR-HBM] and/or HBM).  The underlying heuristic assumes that a similar off-core traffic is generated by all IA cores. This metric does not aggregate non-data-read requests by this logical processor; requests from other IA Logical Processors/Physical Cores/sockets; or other non-IA devices like GPU; hence the maximum external memory bandwidth limits may or may not be approached when this metric is flagged (see Uncore counters for that)."]
pub struct MemBandwidth {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of cycles where the performance was likely hurt due to latency from external memory - DRAM ([SPR-HBM] and/or HBM).  This metric does not aggregate requests from other Logical Processors/Physical Cores/sockets (see Uncore counters for that)."]
pub struct MemLatency {
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates how often CPU was stalled  due to RFO store memory accesses; RFO store issue a read-for-ownership request before the write. Even though store accesses do not typically stall out-of-order CPUs; there are few cases where stores can lead to actual stalls. This metric will be flagged should RFO stores be a bottleneck."]
pub struct StoreBound {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of cycles the CPU spent handling L1D store misses. Store accesses usually less impact out-of-order core performance; however; holding resources for longer time can lead into undesired implications (e.g. contention on L1D fill-buffer entries - see FB_Full)"]
pub struct StoreLatency {
    #[doc = "Retired load instructions with locked access."]
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[doc = "Counts the RFO (Read-for-Ownership) requests that hit L2 cache."]
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[doc = "Counts the number of offcore outstanding demand rfo Reads transactions in the super queue every cycle. The 'Offcore outstanding' state of the transaction lasts from the L2 miss until the sending transaction completion to requestor (SQ deallocation). See the corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric roughly estimates how often CPU was handling synchronizations due to False Sharing. False Sharing is a multithreading hiccup; where multiple Logical Processors contend on different data-elements mapped into the same cache line. "]
pub struct FalseSharing {
    #[doc = "Counts all demand data writes (RFOs)"]
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents rate of split store accesses.  Consider aligning your data to the 64-byte cache line granularity."]
pub struct SplitStores {
    #[doc = "Counts retired store instructions that split across a cacheline boundary."]
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric roughly estimates the fraction of cycles spent handling first-level data TLB store misses.  As with ordinary data caching; focus on improving data locality and reducing working-set size to reduce DTLB overhead.  Additionally; consider using profile-guided optimization (PGO) to collocate frequently-used data on the same page.  Try using larger page sizes for large amounts of frequently-used data."]
pub struct DtlbStore {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric roughly estimates the fraction of cycles where the TLB was missed by store accesses, hitting in the second-level TLB (STLB)"]
pub struct StoreStlbHit {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Stores that miss the DTLB (Data TLB) and hit the STLB (2nd Level TLB)."]
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles where the STLB was missed by store accesses, performing a hardware page walk"]
pub struct StoreStlbMiss {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles to walk the memory paging structures to cache translation of 4 KB pages for data store accesses."]
pub struct StoreStlbMiss4k {
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x249)]
    pub dtlb_store_misses_walk_completed_4k: u64,
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x449)]
    pub dtlb_store_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x849)]
    pub dtlb_store_misses_walk_completed_1g: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles to walk the memory paging structures to cache translation of 2 or 4 MB pages for data store accesses."]
pub struct StoreStlbMiss2m {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
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
#[derive(Debug, Observation)]
#[doc = "This metric estimates the fraction of cycles to walk the memory paging structures to cache translation of 1 GB pages for data store accesses."]
pub struct StoreStlbMiss1g {
    #[doc = "Counts completed page walks  (1G sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x849)]
    pub dtlb_store_misses_walk_completed_1g: u64,
    #[doc = "Counts completed page walks  (2M/4M sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x449)]
    pub dtlb_store_misses_walk_completed_2m_4m: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles when at least one PMH (Page Miss Handler) is busy with a page walk for a store."]
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[doc = "Counts completed page walks  (4K sizes) caused by demand data stores. This implies address translations missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0x249)]
    pub dtlb_store_misses_walk_completed_4k: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots where Core non-memory issues were of a bottleneck.  Shortage in hardware compute resources; or dependencies in software's instructions are both categorized under Core Bound. Hence it may indicate the machine ran out of an out-of-order resource; certain execution units are overloaded or dependencies in program's data- or instruction-flow are limiting the performance (e.g. FP-chained long-latency arithmetic operations)."]
pub struct CoreBound {
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles where the Divider unit was active. Divide and square root instructions are performed by the Divider unit and can take considerably longer latency than integer or Floating Point addition; subtraction; or multiplication."]
pub struct Divider {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles the CPU issue-pipeline was stalled due to serializing operations. Instructions like CPUID; WRMSR or LFENCE serialize the out-of-order execution which may limit performance."]
pub struct SerializingOperation {
    #[doc = "This event counts cycles during which the microcode scoreboard stalls happen."]
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of cycles the CPU performance was potentially limited due to Core computation issues (non divider-related).  Two distinct categories can be attributed into this metric: (1) heavy data-dependency among contiguous instructions would manifest in this metric - such cases are often referred to as low Instruction Level Parallelism (ILP). (2) Contention on some hardware execution unit other than Divider. For example; when there are too many multiply operations."]
pub struct PortsUtilization {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles CPU executed no uops on any execution port (Logical Processor cycles since ICL, Physical Core cycles otherwise). Long-latency instructions like divides may contribute to this metric."]
pub struct PortsUtilized0 {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates penalty in terms of percentage of([SKL+] injected blend uops out of all Uops Issued , the Count Domain; [ADL+] cycles). Usually a Mixing_Vectors over 5% is worth investigating. Read more in Appendix B1 of the Optimizations Guide for this topic."]
pub struct MixingVectors {
    #[doc = "Counts the number of Blend Uops issued by the Resource Allocation Table (RAT) to the reservation station (RS) in order to preserve upper bits of vector registers. Starting with the Skylake microarchitecture, these Blend uops are needed since every Intel SSE instruction executed in Dirty Upper State needs to preserve bits 128-255 of the destination register. For more information, refer to Mixing Intel AVX and Intel SSE Code section of the Optimization Guide."]
    #[raw(0x20e)]
    pub uops_issued_vector_width_mismatch: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles where the CPU executed total of 1 uop per cycle on all execution ports (Logical Processor cycles since ICL, Physical Core cycles otherwise). This can be due to heavy data-dependency among software instructions; or over oversubscribing a particular hardware resource. In some other cases with high 1_Port_Utilized and L1_Bound; this metric can point to L1 data-cache latency bottleneck that may not necessarily manifest with complete execution starvation (due to the short L1 latency e.g. walking a linked list) - looking at the assembly can be helpful."]
pub struct PortsUtilized1 {
    #[doc = "Cycles at least 2 micro-op is executed from any thread on physical core."]
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles at least 1 micro-op is executed from any thread on physical core."]
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles CPU executed total of 2 uops per cycle on all execution ports (Logical Processor cycles since ICL, Physical Core cycles otherwise).  Loop Vectorization -most compilers feature auto-Vectorization options today- reduces pressure on the execution ports as multiple elements are calculated with same uop."]
pub struct PortsUtilized2 {
    #[doc = "Cycles at least 2 micro-op is executed from any thread on physical core."]
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
    #[doc = "Cycles at least 3 micro-op is executed from any thread on physical core."]
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of cycles CPU executed total of 3 or more uops per cycle on all execution ports (Logical Processor cycles since ICL, Physical Core cycles otherwise)."]
pub struct PortsUtilized3m {
    #[doc = "Cycles at least 3 micro-op is executed from any thread on physical core."]
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution ports for ALU operations."]
pub struct AluOpUtilization {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 5."]
    #[raw(0x20a1)]
    pub uops_dispatched_port_port_5: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 0."]
    #[raw(0x1a1)]
    pub uops_dispatched_port_port_0: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 1."]
    #[raw(0x2a1)]
    pub uops_dispatched_port_port_1: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 6."]
    #[raw(0x40a1)]
    pub uops_dispatched_port_port_6: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port 0 ([SNB+] ALU; [HSW+] ALU and 2nd branch)"]
pub struct Port0 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 0."]
    #[raw(0x1a1)]
    pub uops_dispatched_port_port_0: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port 1 (ALU)"]
pub struct Port1 {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 1."]
    #[raw(0x2a1)]
    pub uops_dispatched_port_port_1: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port 5 ([SNB+] Branches and ALU; [HSW+] ALU)"]
pub struct Port5 {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 5."]
    #[raw(0x20a1)]
    pub uops_dispatched_port_port_5: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port 6 ([HSW+] Primary Branch and simple ALU)"]
pub struct Port6 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 6."]
    #[raw(0x40a1)]
    pub uops_dispatched_port_port_6: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port for Load operations"]
pub struct LoadOpUtilization {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 2."]
    #[raw(0x4a1)]
    pub uops_dispatched_port_port_2: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 4."]
    #[raw(0x10a1)]
    pub uops_dispatched_port_port_4: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 3."]
    #[raw(0x8a1)]
    pub uops_dispatched_port_port_3: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 7."]
    #[raw(0x80a1)]
    pub uops_dispatched_port_port_7: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port 2 ([SNB+]Loads and Store-address; [ICL+] Loads)"]
pub struct Port2 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 2."]
    #[raw(0x4a1)]
    pub uops_dispatched_port_port_2: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port 3 ([SNB+]Loads and Store-address; [ICL+] Loads)"]
pub struct Port3 {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 3."]
    #[raw(0x8a1)]
    pub uops_dispatched_port_port_3: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port for Store operations"]
pub struct StoreOpUtilization {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 4."]
    #[raw(0x10a1)]
    pub uops_dispatched_port_port_4: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port 4 (Store-data)"]
pub struct Port4 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 4."]
    #[raw(0x10a1)]
    pub uops_dispatched_port_port_4: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents Core fraction of cycles CPU dispatched uops on execution port 7 ([HSW+]simple Store-address)"]
pub struct Port7 {
    #[doc = "Counts, on the per-thread basis, cycles during which at least one uop is dispatched from the Reservation Station (RS) to port 7."]
    #[raw(0x80a1)]
    pub uops_dispatched_port_port_7: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This category represents fraction of slots utilized by useful work i.e. issued uops that eventually get retired. Ideally; all pipeline slots would be attributed to the Retiring category.  Retiring of 100% would indicate the maximum Pipeline_Width throughput was achieved.  Maximizing Retiring typically increases the Instructions-per-cycle (see IPC metric). Note that a high Retiring value does not necessary mean there is no room for more performance.  For example; Heavy-operations or Microcode Assists are categorized under Retiring. They often indicate suboptimal performance and can often be optimized or avoided. "]
pub struct Retiring {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots where the CPU was retiring light-weight operations , instructions that require no more than one uop (micro-operation). This correlates with total number of instructions used by the program. A uops-per-instruction (see UopPI metric) ratio of 1 or less should be expected for decently optimized code running on Intel Core/Xeon products. While this often indicates efficient X86 instructions were executed; high value does not necessarily mean better performance cannot be achieved. ([ICL+] Note this may undercount due to approximation using indirect events; [ADL+] .)"]
pub struct LightOperations {
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents overall arithmetic floating-point (FP) operations fraction the CPU has executed (retired). Note this metric's value may exceed its parent due to use of \"Uops\" CountDomain and FMA double-counting."]
pub struct FpArith {
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[doc = "Counts the number of x87 uops executed."]
    #[raw(0x10b1)]
    pub uops_executed_x87: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric serves as an approximation of legacy x87 usage. It accounts for instructions beyond X87 FP arithmetic operations; hence may be used as a thermometer to avoid X87 high usage and preferably upgrade to modern ISA. See Tip under Tuning Hint."]
pub struct X87Use {
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[doc = "Counts the number of x87 uops executed."]
    #[raw(0x10b1)]
    pub uops_executed_x87: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric approximates arithmetic floating-point (FP) scalar uops fraction the CPU has retired. May overcount due to FMA double counting."]
pub struct FpScalar {
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric approximates arithmetic floating-point (FP) vector uops fraction the CPU has retired aggregated across all vector widths. May overcount due to FMA double counting."]
pub struct FpVector {
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric approximates arithmetic FP vector uops fraction the CPU has retired for 128-bit wide vectors. May overcount due to FMA double counting prior to LNL."]
pub struct FpVector128b {
    #[doc = "Counts once for most SIMD 128-bit packed computational single precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 4 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x8c7)]
    pub fp_arith_inst_retired_128b_packed_single: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric approximates arithmetic FP vector uops fraction the CPU has retired for 256-bit wide vectors. May overcount due to FMA double counting prior to LNL."]
pub struct FpVector256b {
    #[doc = "Counts once for most SIMD 256-bit packed double computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 4 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x10c7)]
    pub fp_arith_inst_retired_256b_packed_double: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots where the CPU was retiring memory operations , uops for memory load or store accesses."]
pub struct MemoryOperations {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts all retired memory instructions - loads and stores."]
    #[raw(0x83d0)]
    pub mem_inst_retired_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots where the CPU was retiring fused instructions , where one uop can represent multiple contiguous instructions. CMP+JCC or DEC+JCC are common examples of legacy fusions. {([MTL] Note new MOV+OP and Load+OP fusions appear under Other_Light_Ops in MTL!)}"]
pub struct FusedInstructions {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots where the CPU was retiring branch instructions that were not fused. Non-conditional branches like direct JMP or CALL would count here. Can be used to examine fusible conditional jumps that were not fused."]
pub struct NonFusedBranches {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents the remaining light uops fraction the CPU has executed - remaining means not covered by other sibling nodes. May undercount due to FMA double counting"]
pub struct OtherLightOps {
    #[doc = "Counts all retired memory instructions - loads and stores."]
    #[raw(0x83d0)]
    pub mem_inst_retired_any: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Counts the number of x87 uops executed."]
    #[raw(0x10b1)]
    pub uops_executed_x87: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots where the CPU was retiring NOP (no op) instructions. Compilers often use NOPs for certain address alignments - e.g. start address of a function or loop body."]
pub struct NopInstructions {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Number of all retired NOP instructions."]
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots where the CPU was retiring heavy-weight operations , instructions that require two or more uops or micro-coded sequences. This highly-correlates with the uop length of these instructions/sequences.([ICL+] Note this may overcount due to approximation using indirect events; [ADL+])"]
pub struct HeavyOperations {
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots where the CPU was retiring instructions that that are decoder into two or more uops. This highly-correlates with the number of uops in such instructions."]
pub struct FewUopsInstructions {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts the number of macro-fused uops retired. (non precise)"]
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric represents fraction of slots the CPU was retiring uops fetched by the Microcode Sequencer (MS) unit.  The MS is used for CISC instructions not supported by the default decoders (like repeat move strings; or CPUID); or by microcode assists used to address some operation modes (like in Floating Point assists). These cases can often be avoided."]
pub struct MicrocodeSequencer {
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of slots the CPU retired uops delivered by the Microcode_Sequencer as a result of Assists. Assists are long sequences of uops that are required in certain corner-cases for operations that cannot be handled natively by the execution pipeline. For example; when working with very small floating point values (so-called Denormals); the FP units are not set up to perform these operations natively. Instead; a sequence of instructions to perform the computation on the Denormals is injected into the pipeline. Since these microcode sequences might be dozens of uops long; Assists can be extremely deleterious to performance and they can be avoided in many cases."]
pub struct Assists {
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric roughly estimates fraction of slots the CPU retired uops as a result of handing Floating Point (FP) Assists. FP Assist may apply when working with very small floating point values (so-called Denormals)."]
pub struct FpAssists {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "This metric estimates fraction of cycles the CPU retired uops originated from CISC (complex instruction set computer) instruction. A CISC instruction has multiple uops that are required to perform the instruction's functionality as in the case of read-modify-write as an example. Since these instructions require multiple uops they may or may not imply sub-optimal use of machine resources."]
pub struct Cisc {
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "Probability of Core Bound bottleneck hidden by SMT-profiling artifacts"]
pub struct InfoBotlnkL0CoreBoundLikely {
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Cycles when divide unit is busy executing divide or square root operations. Accounts for integer and floating-point operations."]
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[doc = "Counts cycles during which a total of 2 uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[doc = "Total execution stalls."]
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[doc = "Execution stalls while memory subsystem has an outstanding load."]
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[doc = "Core crystal clock cycles when at least one thread on the physical core is unhalted."]
    #[raw(0x20013c)]
    pub cpu_clk_unhalted_ref_xclk_any: u64,
    #[doc = "Core crystal clock cycles when this thread is unhalted and the other thread is halted."]
    #[raw(0x23c)]
    pub cpu_clk_unhalted_one_thread_active: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Counts cycles during which no uops were executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[doc = "Cycles where the Store Buffer was full and no outstanding load."]
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[doc = "Counts cycles during which a total of 1 uop was executed on all ports and Reservation Station (RS) was not empty."]
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions Per Cycle (per Logical Processor)"]
pub struct InfoThreadIpc {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Uops Per Instruction"]
pub struct InfoThreadUopPi {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Uops per taken branch"]
pub struct InfoThreadUpTb {
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Observation)]
#[doc = "Cycles Per Instruction (per Logical Processor)"]
pub struct InfoThreadCpi {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "Per-Logical Processor actual clocks when the Logical Processor is active."]
pub struct InfoThreadClks {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total issue-pipeline slots (per-Physical Core till ICL; per-Logical Processor ICL onward)"]
pub struct InfoThreadSlots {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "The ratio of Executed- by Issued-Uops. Ratio > 1 suggests high rate of uop micro-fusions. Ratio < 1 suggest high rate of \"execute\" at rename stage."]
pub struct InfoThreadExecutePerIssue {
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions Per Cycle across hyper-threads (per physical core)"]
pub struct InfoCoreCoreIpc {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Floating Point Operations Per Cycle"]
pub struct InfoCoreFloPc {
    #[doc = "Number of SSE/AVX computational 128-bit packed single precision and 256-bit packed double precision  floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 or/and 4 computation operations, one for each element.  Applies to SSE* and AVX* packed single precision floating-point and packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX RCP14 RSQRT14 SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x18c7)]
    pub fp_arith_inst_retired_4_flops: u64,
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
}
#[derive(Debug, Observation)]
#[doc = "Actual per-core usage of the Floating Point non-X87 execution units (regardless of precision or vector-width). Values > 1 are possible due to ([BDW+] Fused-Multiply Add (FMA) counting - common; [ADL+] use all of ADD/MUL/FMA in Scalar or 128/256-bit vectors - less common)."]
pub struct InfoCoreFpArithUtilization {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instruction-Level-Parallelism (average number of uops executed when there is execution) per thread (logical-processor)"]
pub struct InfoCoreIlp {
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread_c1: u64,
}
#[derive(Debug, Observation)]
#[doc = "uops Executed per Cycle"]
pub struct InfoCoreEpc {
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "Core actual clocks when any Logical Processor is active on the Physical Core"]
pub struct InfoCoreCoreClks {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per Load (lower number means higher occurrence rate)"]
pub struct InfoInstMixIpLoad {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts all retired load instructions. This event accounts for SW prefetch instructions of PREFETCHNTA or PREFETCHT0/1/2 or PREFETCHW."]
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per Store (lower number means higher occurrence rate)"]
pub struct InfoInstMixIpStore {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts all retired store instructions."]
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per Branch (lower number means higher occurrence rate)"]
pub struct InfoInstMixIpBranch {
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per (near) call (lower number means higher occurrence rate)"]
pub struct InfoInstMixIpCall {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per taken branch"]
pub struct InfoInstMixIpTb {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
}
#[derive(Debug, Observation)]
#[doc = "Branch instructions per taken branch. "]
pub struct InfoInstMixBpTkBranch {
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per Floating Point (FP) Operation (lower number means higher occurrence rate)"]
pub struct InfoInstMixIpFlop {
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[doc = "Number of SSE/AVX computational 128-bit packed single precision and 256-bit packed double precision  floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 or/and 4 computation operations, one for each element.  Applies to SSE* and AVX* packed single precision floating-point and packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX RCP14 RSQRT14 SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x18c7)]
    pub fp_arith_inst_retired_4_flops: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per FP Arithmetic instruction (lower number means higher occurrence rate). Values < 1 are possible due to intentional FMA double counting. Approximated prior to BDW."]
pub struct InfoInstMixIpArith {
    #[doc = "Number of any Vector retired FP arithmetic instructions"]
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per FP Arithmetic Scalar Single-Precision instruction (lower number means higher occurrence rate). Values < 1 are possible due to intentional FMA double counting."]
pub struct InfoInstMixIpArithScalarSp {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x2c7)]
    pub fp_arith_inst_retired_scalar_single: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per FP Arithmetic Scalar Double-Precision instruction (lower number means higher occurrence rate). Values < 1 are possible due to intentional FMA double counting."]
pub struct InfoInstMixIpArithScalarDp {
    #[doc = "Counts once for most SIMD scalar computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar double precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x1c7)]
    pub fp_arith_inst_retired_scalar_double: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per FP Arithmetic AVX/SSE 128-bit instruction (lower number means higher occurrence rate). Values < 1 are possible due to intentional FMA double counting."]
pub struct InfoInstMixIpArithAvx128 {
    #[doc = "Counts once for most SIMD 128-bit packed computational single precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 4 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x8c7)]
    pub fp_arith_inst_retired_128b_packed_single: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per FP Arithmetic AVX* 256-bit instruction (lower number means higher occurrence rate). Values < 1 are possible due to intentional FMA double counting."]
pub struct InfoInstMixIpArithAvx256 {
    #[doc = "Counts once for most SIMD 256-bit packed double computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 4 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x10c7)]
    pub fp_arith_inst_retired_256b_packed_double: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per Software prefetch instruction (of any type: NTA/T0/T1/T2/Prefetch) (lower number means higher occurrence rate)"]
pub struct InfoInstMixIpSwpf {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of PREFETCHNTA, PREFETCHW, PREFETCHT0, PREFETCHT1 or PREFETCHT2 instructions executed."]
    #[raw(0xf32)]
    pub sw_prefetch_access_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total number of retired Instructions"]
pub struct InfoInstMixInstructions {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average number of Uops retired in cycles where at least one uop has retired."]
pub struct InfoPipelineRetire {
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots_c1: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per a microcode Assist invocation. See Assists tree node for details (lower number means higher occurrence rate)"]
pub struct InfoPipelineIpAssist {
    #[doc = "Counts cycles with any input and output SSE or x87 FP assist. If an input and output assist are detected on the same cycle the event increments by 1."]
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[doc = "Number of times a microcode assist is invoked by HW other than FP-assist. Examples include AD (page Access Dirty) and AVX* related assists."]
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instruction-Level-Parallelism (average number of uops executed when there is execution) per core"]
pub struct InfoPipelineExecute {
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread_c1: u64,
    #[doc = "Number of uops to be executed per-thread each cycle."]
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[doc = "Cycles at least 1 micro-op is executed from any thread on physical core."]
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average number of uops fetched from DSB per cycle"]
pub struct InfoPipelineFetchDsb {
    #[doc = "Counts the number of uops delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Counting includes uops that may 'bypass' the IDQ."]
    #[raw(0x879)]
    pub idq_dsb_uops: u64,
    #[doc = "Counts the number of cycles uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_ANY_UOPS]"]
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average number of uops fetched from MITE per cycle"]
pub struct InfoPipelineFetchMite {
    #[doc = "Counts the number of uops delivered to Instruction Decode Queue (IDQ) from the MITE path. Counting includes uops that may 'bypass' the IDQ. This also means that uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x479)]
    pub idq_mite_uops: u64,
    #[doc = "Counts cycles during which uops are being delivered to Instruction Decode Queue (IDQ) from the MITE path. Counting includes uops that may 'bypass' the IDQ."]
    #[raw(0x1000479)]
    pub idq_mite_cycles: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average number of Uops issued by front-end when it issued something"]
pub struct InfoFrontendFetchUpC {
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any_c1: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Fraction of Uops delivered by the DSB (aka Decoded ICache; or Uop Cache)"]
pub struct InfoFrontendDsbCoverage {
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts the number of uops delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Counting includes uops that may 'bypass' the IDQ."]
    #[raw(0x879)]
    pub idq_dsb_uops: u64,
    #[doc = "Counts the number of uops delivered to Instruction Decode Queue (IDQ) from the MITE path. Counting includes uops that may 'bypass' the IDQ. This also means that uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x479)]
    pub idq_mite_uops: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average number of cycles of a switch from the DSB fetch-unit to MITE fetch unit - see DSB_Switches tree node for details."]
pub struct InfoFrontendDsbSwitchCost {
    #[doc = "This event counts the number of the Decode Stream Buffer (DSB)-to-MITE switches including all misses because of missing Decode Stream Buffer (DSB) cache and u-arch forced misses. Note: Invoking MITE requires two or three cycles delay."]
    #[raw(0x1ab)]
    pub dsb2mite_switches_count: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
}
#[derive(Debug, Observation)]
#[doc = "Taken Branches retired Per Cycle"]
pub struct InfoFrontendTBpC {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average Latency for L1 instruction cache misses"]
pub struct InfoFrontendICacheMissLatency {
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per non-speculative DSB miss (lower number means higher occurrence rate)"]
pub struct InfoFrontendIpDsbMissRet {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts retired Instructions that experienced DSB (Decode stream buffer i.e. the decoded instruction-cache) miss."]
    #[raw(0x1c6)]
    pub frontend_retired_any_dsb_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per speculative Unknown Branch Misprediction (BAClear) (lower number means higher occurrence rate)"]
pub struct InfoFrontendIpUnknownBranch {
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "L2 cache true code cacheline misses per kilo instruction "]
pub struct InfoFrontendL2mpkiCode {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Retired Instructions who experienced Instruction L2 Cache true miss."]
    #[raw(0x1c6)]
    pub frontend_retired_l2_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "L2 cache speculative code cacheline misses per kilo instruction "]
pub struct InfoFrontendL2mpkiCodeAll {
    #[doc = "Counts L2 cache misses when fetching instructions."]
    #[raw(0x2424)]
    pub l2_rqsts_code_rd_miss: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of DSB (uop cache) misses - subset of the Instruction_Fetch_BW Bottleneck."]
pub struct InfoBotlnkL2DsbMisses {
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Counts the number of cycles 4 or more uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_4_UOPS]"]
    #[raw(0x4001879)]
    pub idq_dsb_cycles_ok: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts the number of cycles uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x1002479)]
    pub idq_all_mite_cycles_any_uops: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Counts the number of cycles uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_ANY_UOPS]"]
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
    #[doc = "Counts the number of cycles 4 uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x4002479)]
    pub idq_all_mite_cycles_4_uops: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of DSB (uop cache) hits - subset of the Instruction_Fetch_BW Bottleneck."]
pub struct InfoBotlnkL2DsbBandwidth {
    #[doc = "Counts the number of cycles 4 uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x4002479)]
    pub idq_all_mite_cycles_4_uops: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of cycles 4 or more uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_4_UOPS]"]
    #[raw(0x4001879)]
    pub idq_dsb_cycles_ok: u64,
    #[doc = "Counts the number of cycles uops were delivered to the Instruction Decode Queue (IDQ) from the MITE (legacy decode pipeline) path. Counting includes uops that may 'bypass' the IDQ. During these cycles uops are not being delivered from the Decode Stream Buffer (DSB)."]
    #[raw(0x1002479)]
    pub idq_all_mite_cycles_any_uops: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts the number of uops not delivered to Resource Allocation Table (RAT) per thread adding 4  x when Resource Allocation Table (RAT) is not stalled and Instruction Decode Queue (IDQ) delivers x uops to Resource Allocation Table (RAT) (where x belongs to {0,1,2,3}). Counting does not cover cases when: a. IDQ-Resource Allocation Table (RAT) pipe serves the other thread. b. Resource Allocation Table (RAT) is stalled for the thread (including uop drops and clear BE conditions).  c. Instruction Decode Queue (IDQ) delivers four uops."]
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[doc = "Counts the number of cycles uops were delivered to Instruction Decode Queue (IDQ) from the Decode Stream Buffer (DSB) path. Count includes uops that may 'bypass' the IDQ. [This event is alias to IDQ.ALL_DSB_CYCLES_ANY_UOPS]"]
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Total pipeline cost of Instruction Cache misses - subset of the Big_Code Bottleneck."]
pub struct InfoBotlnkL2IcMisses {
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
}
#[derive(Debug, Observation)]
#[doc = "Number of Instructions per non-speculative Branch Misprediction (JEClear) (lower number means higher occurrence rate)"]
pub struct InfoBadSpecIpMispredict {
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per retired Mispredicts for indirect CALL or JMP branches (lower number means higher occurrence rate)."]
pub struct InfoBadSpecIpMispIndirect {
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts speculatively miss-predicted indirect branches at execution time. Counts for indirect near CALL or JMP instructions (RET excluded)."]
    #[raw(0xe489)]
    pub br_misp_exec_indirect: u64,
}
#[derive(Debug, Observation)]
#[doc = "Branch Misprediction Cost: Cycles representing fraction of TMA slots wasted per non-speculative branch misprediction (retired JEClear)"]
pub struct InfoBadSpecBranchMispredictionCost {
    #[doc = "Cycles the issue-stage is waiting for front-end to fetch from resteered path following branch misprediction or machine clear events."]
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[doc = "Counts, on the per-thread basis, cycles when no uops are delivered to Resource Allocation Table (RAT). IDQ_Uops_Not_Delivered.core =4."]
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Number of switches from DSB (Decode Stream Buffer) or MITE (legacy decode pipeline) to the Microcode Sequencer."]
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[doc = "Counts cycles that the Instruction Length decoder (ILD) stalls occurred due to dynamically changing prefix length of the decoded instruction (by operand size prefix instruction 0x66, address size prefix instruction 0x67 or REX.W for Intel64). Count is proportional to the number of prefixes in a 16B-line. This may result in a three-cycle penalty for each LCP (Length changing prefix) in a 16-byte chunk. [This event is alias to ILD_STALL.LCP]"]
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[doc = "Counts the total number of uops delivered by the Microcode Sequencer (MS). Any instruction over 4 uops will be delivered by the MS. Some instructions such as transcendentals may additionally generate uops from the MS."]
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[doc = "Counts the number of times the front-end is resteered when it finds a branch instruction in a fetch line. This occurs for the first time a branch instruction is fetched or when the branch is not tracked by the BPU (Branch Prediction Unit) anymore."]
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[doc = "Counts the retirement slots used."]
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[doc = "Counts the number of uops that the Resource Allocation Table (RAT) issues to the Reservation Station (RS)."]
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[doc = "Core cycles the Resource allocator was stalled due to recovery from an earlier branch misprediction or machine clear event."]
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[doc = "Cycles where a code fetch is stalled due to L1 instruction cache tag miss. [This event is alias to ICACHE_64B.IFTAG_STALL]"]
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[doc = "Counts Decode Stream Buffer (DSB)-to-MITE switch true penalty cycles. These cycles do not include uops routed through because of the switch itself, for example, when Instruction Decode Queue (IDQ) pre-allocation is unavailable, or Instruction Decode Queue (IDQ) is full. SBD-to-MITE switch true penalty cycles happen after the merge mux (MM) receives Decode Stream Buffer (DSB) Sync-indication until receiving the first MITE uop. MM is placed before Instruction Decode Queue (IDQ) to merge uops being fed from the MITE and Decode Stream Buffer (DSB) paths. Decode Stream Buffer (DSB) inserts the Sync-indication whenever a Decode Stream Buffer (DSB)-to-MITE switch occurs.Penalty: A Decode Stream Buffer (DSB) hit followed by a Decode Stream Buffer (DSB) miss can cost up to six cycles in which no uops are delivered to the IDQ. Most often, such switches from the Decode Stream Buffer (DSB) to the legacy pipeline cost 02 cycles."]
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Cycles where a code line fetch is stalled due to an L1 instruction cache miss. The legacy decode pipeline works at a 16 Byte granularity."]
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
}
#[derive(Debug, Observation)]
#[doc = "Speculative to Retired ratio of all clears (covering Mispredicts and nukes)"]
pub struct InfoBadSpecSpecClearsRatio {
    #[doc = "Counts the number of speculative clears due to any type of branch misprediction or machine clears"]
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[doc = "Number of machine clears (nukes) of any type."]
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[doc = "Counts all the retired branch instructions that were mispredicted by the processor. A branch misprediction occurs when the processor incorrectly predicts the destination of the branch.  When the misprediction is discovered at execution, all the instructions executed in the wrong (speculative) path must be discarded, and the processor must start fetching from the correct path."]
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
}
#[derive(Debug, Observation)]
#[doc = "Fraction of branches that are non-taken conditionals"]
pub struct InfoBranchesCondNt {
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "This event counts not taken branch instructions retired."]
    #[raw(0x10c4)]
    pub br_inst_retired_not_taken: u64,
}
#[derive(Debug, Observation)]
#[doc = "Fraction of branches that are taken conditionals"]
pub struct InfoBranchesCondTk {
    #[doc = "This event counts not taken branch instructions retired."]
    #[raw(0x10c4)]
    pub br_inst_retired_not_taken: u64,
    #[doc = "This event counts conditional branch instructions retired. [This event is alias to BR_INST_RETIRED.COND]"]
    #[raw(0x1c4)]
    pub br_inst_retired_conditional: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Observation)]
#[doc = "Fraction of branches that are CALL or RET"]
pub struct InfoBranchesCallRet {
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "This event counts return instructions retired."]
    #[raw(0x8c4)]
    pub br_inst_retired_near_return: u64,
}
#[derive(Debug, Observation)]
#[doc = "Fraction of branches that are unconditional (direct or indirect) jumps"]
pub struct InfoBranchesJump {
    #[doc = "This event counts taken branch instructions retired."]
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
    #[doc = "This event counts not taken branch instructions retired."]
    #[raw(0x10c4)]
    pub br_inst_retired_not_taken: u64,
    #[doc = "This event counts both direct and indirect near call instructions retired."]
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[doc = "Counts all (macro) branch instructions retired."]
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[doc = "This event counts conditional branch instructions retired. [This event is alias to BR_INST_RETIRED.CONDITIONAL]"]
    #[raw(0x1c4)]
    pub br_inst_retired_cond: u64,
}
#[derive(Debug, Observation)]
#[doc = "Actual Average Latency for L1 data-cache miss demand load operations (in core cycles)"]
pub struct InfoMemoryLoadMissRealLatency {
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "Memory-Level-Parallelism (average number of L1 miss demand load when there is at least one such miss. Per-Logical Processor)"]
pub struct InfoMemoryMlp {
    #[doc = "Counts duration of L1D miss outstanding in cycles."]
    #[raw(0x1000148)]
    pub l1d_pend_miss_pending_cycles: u64,
    #[doc = "Counts duration of L1D miss outstanding, that is each cycle number of Fill Buffers (FB) outstanding required by Demand Reads. FB either is held by demand loads, or it is held by non-demand loads and gets hit at least once by demand. The valid outstanding interval is defined until the FB deallocation by one of the following ways: from FB allocation, if FB is allocated by demand from the demand Hit FB, if it is allocated by hardware or software prefetch.Note: In the L1D, a Demand Read contains cacheable or noncacheable demand loads, including ones causing cache-line splits and reads due to page walks resulted from any request type."]
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
}
#[derive(Debug, Observation)]
#[doc = "L1 cache true misses per kilo instruction for retired demand loads"]
pub struct InfoMemoryL1mpki {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L1 cache."]
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "L1 cache true misses per kilo instruction for all demand loads (including speculative)"]
pub struct InfoMemoryL1mpkiLoad {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of demand Data Read requests (including requests from L1D hardware prefetchers). These loads may hit or miss L2 cache. Only non rejected loads are counted."]
    #[raw(0xe124)]
    pub l2_rqsts_all_demand_data_rd: u64,
}
#[derive(Debug, Observation)]
#[doc = "L2 cache true misses per kilo instruction for retired demand loads"]
pub struct InfoMemoryL2mpki {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Retired load instructions missed L2 cache as data sources."]
    #[raw(0x10d1)]
    pub mem_load_retired_l2_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "L2 cache ([RKL+] true) misses per kilo instruction for all request types (including speculative)"]
pub struct InfoMemoryL2mpkiAll {
    #[doc = "All requests that miss L2 cache."]
    #[raw(0x3f24)]
    pub l2_rqsts_miss: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "L2 cache ([RKL+] true) misses per kilo instruction for all demand loads  (including speculative)"]
pub struct InfoMemoryL2mpkiLoad {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of demand Data Read requests that miss L2 cache. Only not rejected loads are counted."]
    #[raw(0x2124)]
    pub l2_rqsts_demand_data_rd_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "Offcore requests (L2 cache miss) per kilo instruction for demand RFOs"]
pub struct InfoMemoryL2mpkiRfo {
    #[doc = "Counts the demand RFO (read for ownership) requests including regular RFOs, locks, ItoM."]
    #[raw(0x4b0)]
    pub offcore_requests_demand_rfo: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "L2 cache hits per kilo instruction for all request types (including speculative)"]
pub struct InfoMemoryL2hpkiAll {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "All L2 requests."]
    #[raw(0xff24)]
    pub l2_rqsts_references: u64,
    #[doc = "All requests that miss L2 cache."]
    #[raw(0x3f24)]
    pub l2_rqsts_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "L2 cache hits per kilo instruction for all demand loads  (including speculative)"]
pub struct InfoMemoryL2hpkiLoad {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts the number of demand Data Read requests, initiated by load instructions, that hit L2 cache"]
    #[raw(0xc124)]
    pub l2_rqsts_demand_data_rd_hit: u64,
}
#[derive(Debug, Observation)]
#[doc = "L3 cache true misses per kilo instruction for retired demand loads"]
pub struct InfoMemoryL3mpki {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Counts retired load instructions with at least one uop that missed in the L3 cache."]
    #[raw(0x20d1)]
    pub mem_load_retired_l3_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "Fill Buffer (FB) hits per kilo instructions for retired demand loads (L1D misses that merge into ongoing miss-handling entries)"]
pub struct InfoMemoryFbHpki {
    #[doc = "Counts retired load instructions with at least one uop was load missed in L1 but hit FB (Fill Buffers) due to preceding miss to the same cache line with data not ready."]
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average per-thread data fill bandwidth to the L1 data cache [GB / sec]"]
pub struct InfoMemoryL1dCacheFillBw {
    #[doc = "Counts L1D data line replacements including opportunistic replacements, and replacements that require stall-for-replace or block-for-replace."]
    #[raw(0x151)]
    pub l1d_replacement: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average per-thread data fill bandwidth to the L2 cache [GB / sec]"]
pub struct InfoMemoryL2CacheFillBw {
    #[doc = "Counts the number of L2 cache lines filling the L2. Counting does not cover rejects."]
    #[raw(0x1ff1)]
    pub l2_lines_in_all: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average per-thread data fill bandwidth to the L3 cache [GB / sec]"]
pub struct InfoMemoryL3CacheFillBw {
    #[doc = "Counts core-originated cacheable requests that miss the L3 cache (Longest Latency cache). Requests include data and code reads, Reads-for-Ownership (RFOs), speculative accesses and hardware prefetches from L1 and L2. It does not include all misses to the L3."]
    #[raw(0x412e)]
    pub longest_lat_cache_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average per-thread data access bandwidth to the L3 cache [GB / sec]"]
pub struct InfoMemoryL3CacheAccessBw {
    #[doc = "Counts memory transactions reached the super queue including requests initiated by the core, all L3 prefetches, page walks, etc.."]
    #[raw(0x80b0)]
    pub offcore_requests_all_requests: u64,
}
#[derive(Debug, Observation)]
#[doc = "Utilization of the core's Page Walker(s) serving STLB misses triggered by instruction/Load/Store accesses"]
pub struct InfoMemoryTlbPageWalksUtilization {
    #[doc = "Counts 1 per cycle for each PMH that is busy with a page walk for a load. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1008)]
    pub dtlb_load_misses_walk_pending: u64,
    #[doc = "Counts 1 per cycle for each PMH that is busy with a page walk for a store. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1049)]
    pub dtlb_store_misses_walk_pending: u64,
    #[doc = "Counts 1 per cycle for each PMH (Page Miss Handler) that is busy with a page walk for an instruction fetch request. EPT page walk duration are excluded in Skylake microarchitecture."]
    #[raw(0x1085)]
    pub itlb_misses_walk_pending: u64,
    #[doc = "Counts cycles for each PMH (Page Miss Handler) that is busy with an EPT (Extended Page Table) walk for any request type."]
    #[raw(0x104f)]
    pub ept_walk_pending: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "STLB (2nd level TLB) code speculative misses per kilo instruction (misses of any page-size that complete the page walk)"]
pub struct InfoMemoryTlbCodeStlbMpki {
    #[doc = "Counts completed page walks (all page sizes) caused by a code fetch. This implies it missed in the ITLB (Instruction TLB) and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0xe85)]
    pub itlb_misses_walk_completed: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "STLB (2nd level TLB) data load speculative misses per kilo instruction (misses of any page-size that complete the page walk)"]
pub struct InfoMemoryTlbLoadStlbMpki {
    #[doc = "Counts completed page walks  (all page sizes) caused by demand data loads. This implies it missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0xe08)]
    pub dtlb_load_misses_walk_completed: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "STLB (2nd level TLB) data store speculative misses per kilo instruction (misses of any page-size that complete the page walk)"]
pub struct InfoMemoryTlbStoreStlbMpki {
    #[doc = "Counts completed page walks  (all page sizes) caused by demand data stores. This implies it missed in the DTLB and further levels of TLB. The page walk can end with or without a fault."]
    #[raw(0xe49)]
    pub dtlb_store_misses_walk_completed: u64,
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average per-core data fill bandwidth to the L1 data cache [GB / sec]"]
pub struct InfoMemoryCoreL1dCacheFillBw2t {
    #[doc = "Counts L1D data line replacements including opportunistic replacements, and replacements that require stall-for-replace or block-for-replace."]
    #[raw(0x151)]
    pub l1d_replacement: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average per-core data fill bandwidth to the L2 cache [GB / sec]"]
pub struct InfoMemoryCoreL2CacheFillBw2t {
    #[doc = "Counts the number of L2 cache lines filling the L2. Counting does not cover rejects."]
    #[raw(0x1ff1)]
    pub l2_lines_in_all: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average per-core data fill bandwidth to the L3 cache [GB / sec]"]
pub struct InfoMemoryCoreL3CacheFillBw2t {
    #[doc = "Counts core-originated cacheable requests that miss the L3 cache (Longest Latency cache). Requests include data and code reads, Reads-for-Ownership (RFOs), speculative accesses and hardware prefetches from L1 and L2. It does not include all misses to the L3."]
    #[raw(0x412e)]
    pub longest_lat_cache_miss: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average per-core data access bandwidth to the L3 cache [GB / sec]"]
pub struct InfoMemoryCoreL3CacheAccessBw2t {
    #[doc = "Counts memory transactions reached the super queue including requests initiated by the core, all L3 prefetches, page walks, etc.."]
    #[raw(0x80b0)]
    pub offcore_requests_all_requests: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average Latency for L2 cache miss demand Loads"]
pub struct InfoMemoryLatencyLoadL2MissLatency {
    #[doc = "Counts the number of offcore outstanding Demand Data Read transactions in the super queue (SQ) every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor. See the corresponding Umask under OFFCORE_REQUESTS.Note: A prefetch promoted to Demand is counted from the promotion point."]
    #[raw(0x160)]
    pub offcore_requests_outstanding_demand_data_rd: u64,
    #[doc = "Counts the Demand Data Read requests sent to uncore. Use it in conjunction with OFFCORE_REQUESTS_OUTSTANDING to determine average latency in the uncore."]
    #[raw(0x1b0)]
    pub offcore_requests_demand_data_rd: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average Parallel L2 cache miss demand Loads"]
pub struct InfoMemoryLatencyLoadL2Mlp {
    #[doc = "Counts the number of offcore outstanding Demand Data Read transactions in the super queue (SQ) every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor. See the corresponding Umask under OFFCORE_REQUESTS.Note: A prefetch promoted to Demand is counted from the promotion point."]
    #[raw(0x160)]
    pub offcore_requests_outstanding_demand_data_rd: u64,
    #[doc = "Counts cycles when offcore outstanding Demand Data Read transactions are present in the super queue (SQ). A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation)."]
    #[raw(0x1000160)]
    pub offcore_requests_outstanding_cycles_with_demand_data_rd: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average Parallel L2 cache miss data reads"]
pub struct InfoMemoryLatencyDataL2Mlp {
    #[doc = "Counts the number of offcore outstanding cacheable Core Data Read transactions in the super queue every cycle. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd: u64,
    #[doc = "Counts cycles when offcore outstanding cacheable Core Data Read transactions are present in the super queue. A transaction is considered to be in the Offcore outstanding state between L2 miss and transaction completion sent to requestor (SQ de-allocation). See corresponding Umask under OFFCORE_REQUESTS."]
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
}
#[derive(Debug, Observation)]
#[doc = "Un-cacheable retired load per kilo instruction"]
pub struct InfoMemoryMixUcLoadPki {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "Retired instructions with at least 1 uncacheable load or lock."]
    #[raw(0x4d4)]
    pub mem_load_misc_retired_uc: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average CPU Utilization (percentage)"]
pub struct InfoSystemCpuUtilization {
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average number of utilized CPUs"]
pub struct InfoSystemCpUsUtilized {
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Observation)]
#[doc = "Measured Average Core Frequency for unhalted processors [GHz]"]
pub struct InfoSystemCoreFrequency {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Observation)]
#[doc = "Giga Floating Point Operations Per Second. Aggregate across all supported options of: FP precisions, scalar and vector instructions, vector-width"]
pub struct InfoSystemGfloPs {
    #[doc = "Number of SSE/AVX computational 128-bit packed single precision and 256-bit packed double precision  floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 or/and 4 computation operations, one for each element.  Applies to SSE* and AVX* packed single precision floating-point and packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX RCP14 RSQRT14 SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x18c7)]
    pub fp_arith_inst_retired_4_flops: u64,
    #[doc = "Counts once for most SIMD 128-bit packed computational double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 2 computation operations, one for each element.  Applies to packed double precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[doc = "Counts once for most SIMD scalar computational single precision and double precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 1 computational operation. Applies to SIMD scalar single precision floating-point instructions: ADD SUB MUL DIV MIN MAX SQRT RSQRT RCP FM(N)ADD/SUB.  FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[doc = "Counts once for most SIMD 256-bit packed single computational precision floating-point instructions retired; some instructions will count twice as noted below.  Each count represents 8 computation operations, one for each element.  Applies to packed single precision floating-point instructions: ADD SUB HADD HSUB SUBADD MUL DIV MIN MAX SQRT RSQRT RCP DPP FM(N)ADD/SUB.  DPP and FM(N)ADD/SUB instructions count twice as they perform 2 calculations per element. The DAZ and FTZ flags in the MXCSR register need to be set when using these events."]
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average Frequency Utilization relative nominal frequency"]
pub struct InfoSystemTurboUtilization {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "Counts the number of reference cycles when the core is not in a halt state. The core enters the halt state when it is running the HLT instruction or the MWAIT instruction. This event is not affected by core frequency changes (for example, P states, TM2 transitions) but has the same incrementing frequency as the time stamp counter. This event can approximate elapsed time while the core was not in a halt state. This event has a constant ratio with the CPU_CLK_UNHALTED.REF_XCLK event. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. Note: On all current platforms this event stops counting during 'throttling (TM)' states duty off periods the processor is 'halted'.  The counter update is done at a lower clock rate then the core clock the overflow status bit for this counter may appear 'sticky'.  After the counter has overflowed and software clears the overflow status bit and resets the counter to less than MAX. The reset value to the counter is not clocked immediately so the overflow status bit will flip 'high (1)' and generate another PMI (if enabled) after which the reset value gets clocked into the counter. Therefore, software will get the interrupt, read the overflow status bit '1 for bit 34 while the counter value is less than MAX. Software should ignore this case."]
    #[hardware(REF_CPU_CYCLES)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Observation)]
#[doc = "Fraction of cycles where both hardware Logical Processors were active"]
pub struct InfoSystemSmt2tUtilization {
    #[doc = "Core crystal clock cycles when this thread is unhalted and the other thread is halted."]
    #[raw(0x23c)]
    pub cpu_clk_unhalted_one_thread_active: u64,
    #[doc = "Core crystal clock cycles when at least one thread on the physical core is unhalted."]
    #[raw(0x20013c)]
    pub cpu_clk_unhalted_ref_xclk_any: u64,
}
#[derive(Debug, Observation)]
#[doc = "Fraction of cycles spent in the Operating System (OS) Kernel mode"]
pub struct InfoSystemKernelUtilization {
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
    #[doc = "This is an architectural event that counts the number of thread cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. The core frequency may change from time to time due to power or thermal throttling. For this reason, this event may have a changing ratio with regards to wall clock time."]
    #[raw(0x3c)]
    pub cpu_clk_unhalted_thread_p_sup: u64,
}
#[derive(Debug, Observation)]
#[doc = "Cycles Per Instruction for the Operating System (OS) Kernel mode"]
pub struct InfoSystemKernelCpi {
    #[doc = "Counts the number of instructions (EOMs) retired. Counting covers macro-fused instructions individually (that is, increments by two)."]
    #[raw(0xc0)]
    pub inst_retired_any_p_sup: u64,
    #[doc = "This is an architectural event that counts the number of thread cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. The core frequency may change from time to time due to power or thermal throttling. For this reason, this event may have a changing ratio with regards to wall clock time."]
    #[raw(0x3c)]
    pub cpu_clk_unhalted_thread_p_sup: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average external Memory Bandwidth Use for reads and writes [GB / sec]"]
pub struct InfoSystemDramBwUse {
    #[doc = "Number of entries allocated. Account for Any type: e.g. Snoop, Core aperture, etc."]
    #[raw(0x184)]
    pub unc_arb_coh_trk_requests_all: u64,
    #[doc = "UNC_ARB_TRK_REQUESTS.ALL"]
    #[raw(0x181)]
    pub unc_arb_trk_requests_all: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average latency of data read request to external memory (in nanoseconds). Accounts for demand loads and L1/L2 prefetches. ([RKL+]memory-controller only)"]
pub struct InfoSystemMemReadLatency {
    #[doc = "Number of Core coherent Data Read requests sent to memory controller whose data is returned directly to requesting agent."]
    #[raw(0x281)]
    pub unc_arb_trk_requests_data_read: u64,
    #[doc = "Number of Core Data Read entries outstanding for the memory controller. The outstanding interval starts after LLC miss till return of first data chunk."]
    #[raw(0x280)]
    pub unc_arb_trk_occupancy_data_read: u64,
    #[doc = "This 48-bit fixed counter counts the UCLK cycles."]
    #[raw(0x100)]
    pub unc_clock_socket: u64,
}
#[derive(Debug, Observation)]
#[doc = "Average number of parallel data read requests to external memory. Accounts for demand loads and L1/L2 prefetches"]
pub struct InfoSystemMemParallelReads {
    #[doc = "Number of Core Data Read entries outstanding for the memory controller. The outstanding interval starts after LLC miss till return of first data chunk."]
    #[raw(0x280)]
    pub unc_arb_trk_occupancy_data_read_c1: u64,
    #[doc = "Number of Core Data Read entries outstanding for the memory controller. The outstanding interval starts after LLC miss till return of first data chunk."]
    #[raw(0x280)]
    pub unc_arb_trk_occupancy_data_read: u64,
}
#[derive(Debug, Observation)]
#[doc = "Run duration time in seconds"]
pub struct InfoSystemTime {}
#[derive(Debug, Observation)]
#[doc = "PerfMon Event Multiplexing accuracy indicator"]
pub struct InfoSystemMux {
    #[doc = "This is an architectural event that counts the number of thread cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. The core frequency may change from time to time due to power or thermal throttling. For this reason, this event may have a changing ratio with regards to wall clock time."]
    #[raw(0x3c)]
    pub cpu_clk_unhalted_thread_p: u64,
    #[doc = "Counts the number of core cycles while the thread is not in a halt state. The thread enters the halt state when it is running the HLT instruction. This event is a component in many key event ratios. The core frequency may change from time to time due to transitions associated with Enhanced Intel SpeedStep Technology or TM2. For this reason this event may have a changing ratio with regards to time. When the core frequency is constant, this event can approximate elapsed time while the core was not in the halt state. It is counted on a dedicated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events."]
    #[hardware(CPU_CYCLES)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Observation)]
#[doc = "Socket actual clocks when any core is active on that socket"]
pub struct InfoSystemSocketClks {
    #[doc = "This 48-bit fixed counter counts the UCLK cycles."]
    #[raw(0x100)]
    pub unc_clock_socket: u64,
}
#[derive(Debug, Observation)]
#[doc = "Instructions per Far Branch ( Far Branches apply upon transition from application to operating system, handling interrupts, exceptions) [lower number means higher occurrence rate]"]
pub struct InfoSystemIpFarBranch {
    #[doc = "Counts the number of instructions retired from execution. For instructions that consist of multiple micro-ops, Counts the retirement of the last micro-op of the instruction. Counting continues during hardware interrupts, traps, and inside interrupt handlers. Notes: INST_RETIRED.ANY is counted by a designated fixed counter, leaving the four (eight when Hyperthreading is disabled) programmable counters available for other events. INST_RETIRED.ANY_P is counted by a programmable counter and it is an architectural performance event. Counting: Faulting executions of GETSEC/VM entry/VM Exit/MWait will not count as retired instructions."]
    #[hardware(INSTRUCTIONS)]
    pub inst_retired_any: u64,
    #[doc = "This event counts far branch instructions retired."]
    #[raw(0x40c4)]
    pub br_inst_retired_far_branch_user: u64,
}
