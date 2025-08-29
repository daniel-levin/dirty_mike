use crate::Counter;
#[derive(Debug, Counter)]
pub struct BottleneckMispredictions {
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckBigCode {
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckInstructionFetchBw {
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckCacheMemoryBandwidth {
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckCacheMemoryLatency {
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckMemoryDataTlBs {
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckMemorySynchronization {
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckComputeBoundEst {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckIrregularOverhead {
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x15e)]
    pub rs_events_empty_cycles: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckOtherBottlenecks {
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[raw(0x15e)]
    pub rs_events_empty_cycles: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckBranchingOverhead {
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct BottleneckUsefulWork {
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
}
#[derive(Debug, Counter)]
pub struct FrontendBound {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct FetchLatency {
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct ICacheMisses {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
}
#[derive(Debug, Counter)]
pub struct ItlbMisses {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
}
#[derive(Debug, Counter)]
pub struct CodeStlbHit {
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
}
#[derive(Debug, Counter)]
pub struct CodeStlbMiss {
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct CodeStlbMiss4k {
    #[raw(0x485)]
    pub itlb_misses_walk_completed_2m_4m: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x285)]
    pub itlb_misses_walk_completed_4k: u64,
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
}
#[derive(Debug, Counter)]
pub struct CodeStlbMiss2m {
    #[raw(0x485)]
    pub itlb_misses_walk_completed_2m_4m: u64,
    #[raw(0x285)]
    pub itlb_misses_walk_completed_4k: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1001085)]
    pub itlb_misses_walk_active: u64,
}
#[derive(Debug, Counter)]
pub struct BranchResteers {
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
}
#[derive(Debug, Counter)]
pub struct MispredictsResteers {
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct ClearsResteers {
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct UnknownBranches {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
}
#[derive(Debug, Counter)]
pub struct MsSwitches {
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Lcp {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x187)]
    pub decode_lcp: u64,
}
#[derive(Debug, Counter)]
pub struct DsbSwitches {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct FetchBandwidth {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Mite {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x4002479)]
    pub idq_all_mite_cycles_4_uops: u64,
    #[raw(0x1002479)]
    pub idq_all_mite_cycles_any_uops: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Decoder0Alone {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x155)]
    pub inst_decoded_decoders_c2: u64,
    #[raw(0x155)]
    pub inst_decoded_decoders_c1: u64,
}
#[derive(Debug, Counter)]
pub struct Dsb {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x4001879)]
    pub idq_dsb_cycles_ok: u64,
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct BadSpeculation {
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct BranchMispredicts {
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct OtherMispredicts {
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct MachineClears {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct OtherNukes {
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x2c3)]
    pub machine_clears_memory_ordering: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct BackendBound {
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
}
#[derive(Debug, Counter)]
pub struct MemoryBound {
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct L1Bound {
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
}
#[derive(Debug, Counter)]
pub struct DtlbLoad {
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbHit {
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[raw(0x2008)]
    pub dtlb_load_misses_stlb_hit_c1: u64,
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbMiss {
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbMiss4k {
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x408)]
    pub dtlb_load_misses_walk_completed_2m_4m: u64,
    #[raw(0x808)]
    pub dtlb_load_misses_walk_completed_1g: u64,
    #[raw(0x208)]
    pub dtlb_load_misses_walk_completed_4k: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbMiss2m {
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0x208)]
    pub dtlb_load_misses_walk_completed_4k: u64,
    #[raw(0x808)]
    pub dtlb_load_misses_walk_completed_1g: u64,
    #[raw(0x408)]
    pub dtlb_load_misses_walk_completed_2m_4m: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct LoadStlbMiss1g {
    #[raw(0x408)]
    pub dtlb_load_misses_walk_completed_2m_4m: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1001008)]
    pub dtlb_load_misses_walk_active: u64,
    #[raw(0x808)]
    pub dtlb_load_misses_walk_completed_1g: u64,
    #[raw(0x208)]
    pub dtlb_load_misses_walk_completed_4k: u64,
}
#[derive(Debug, Counter)]
pub struct StoreFwdBlk {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x203)]
    pub ld_blocks_store_forward: u64,
}
#[derive(Debug, Counter)]
pub struct L1LatencyDependency {
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
    #[raw(0x100010a3)]
    pub cycle_activity_cycles_mem_any: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x80008a3)]
    pub cycle_activity_cycles_l1d_miss: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct LockLatency {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
    #[raw(0xe224)]
    pub l2_rqsts_all_rfo: u64,
}
#[derive(Debug, Counter)]
pub struct SplitLoads {
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x803)]
    pub ld_blocks_no_sr: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct _4kAliasing {
    #[raw(0x107)]
    pub ld_blocks_partial_address_alias: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct FbFull {
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct L2Bound {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
}
#[derive(Debug, Counter)]
pub struct L2HitLatency {
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
}
#[derive(Debug, Counter)]
pub struct L3Bound {
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct ContestedAccesses {
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x1d2)]
    pub mem_load_l3_hit_retired_xsnp_miss: u64,
    #[raw(0x4d2)]
    pub mem_load_l3_hit_retired_xsnp_hitm: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct DataSharing {
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x2d2)]
    pub mem_load_l3_hit_retired_xsnp_hit: u64,
}
#[derive(Debug, Counter)]
pub struct L3HitLatency {
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x4d1)]
    pub mem_load_retired_l3_hit: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct SqFull {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x1b2)]
    pub offcore_requests_buffer_sq_full: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct DramBound {
    #[raw(0x60006a3)]
    pub cycle_activity_stalls_l3_miss: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0xc000ca3)]
    pub cycle_activity_stalls_l1d_miss: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x248)]
    pub l1d_pend_miss_fb_full_c1: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x50005a3)]
    pub cycle_activity_stalls_l2_miss: u64,
    #[raw(0x2d1)]
    pub mem_load_retired_l2_hit: u64,
}
#[derive(Debug, Counter)]
pub struct MemBandwidth {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
}
#[derive(Debug, Counter)]
pub struct MemLatency {
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd_c4: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
}
#[derive(Debug, Counter)]
pub struct StoreBound {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
}
#[derive(Debug, Counter)]
pub struct StoreLatency {
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1000460)]
    pub offcore_requests_outstanding_cycles_with_demand_rfo: u64,
    #[raw(0xc224)]
    pub l2_rqsts_rfo_hit: u64,
    #[raw(0x21d0)]
    pub mem_inst_retired_lock_loads: u64,
}
#[derive(Debug, Counter)]
pub struct FalseSharing {
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x1b7)]
    pub offcore_response_demand_rfo_l3_hit_snoop_hitm: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct SplitStores {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x42d0)]
    pub mem_inst_retired_split_stores: u64,
}
#[derive(Debug, Counter)]
pub struct DtlbStore {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbHit {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x2049)]
    pub dtlb_store_misses_stlb_hit_c1: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbMiss {
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbMiss4k {
    #[raw(0x249)]
    pub dtlb_store_misses_walk_completed_4k: u64,
    #[raw(0x449)]
    pub dtlb_store_misses_walk_completed_2m_4m: u64,
    #[raw(0x849)]
    pub dtlb_store_misses_walk_completed_1g: u64,
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbMiss2m {
    #[raw(0x249)]
    pub dtlb_store_misses_walk_completed_4k: u64,
    #[raw(0x849)]
    pub dtlb_store_misses_walk_completed_1g: u64,
    #[raw(0x449)]
    pub dtlb_store_misses_walk_completed_2m_4m: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
}
#[derive(Debug, Counter)]
pub struct StoreStlbMiss1g {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x849)]
    pub dtlb_store_misses_walk_completed_1g: u64,
    #[raw(0x449)]
    pub dtlb_store_misses_walk_completed_2m_4m: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x249)]
    pub dtlb_store_misses_walk_completed_4k: u64,
    #[raw(0x1001049)]
    pub dtlb_store_misses_walk_active: u64,
}
#[derive(Debug, Counter)]
pub struct CoreBound {
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct Divider {
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct SerializingOperation {
    #[raw(0x159)]
    pub partial_rat_stalls_scoreboard: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilization {
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilized0 {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
}
#[derive(Debug, Counter)]
pub struct MixingVectors {
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x20e)]
    pub uops_issued_vector_width_mismatch: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilized1 {
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilized2 {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x20002b1)]
    pub uops_executed_core_cycles_ge_2: u64,
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
}
#[derive(Debug, Counter)]
pub struct PortsUtilized3m {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x30002b1)]
    pub uops_executed_core_cycles_ge_3: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct AluOpUtilization {
    #[raw(0x1a1)]
    pub uops_dispatched_port_port_0: u64,
    #[raw(0x20a1)]
    pub uops_dispatched_port_port_5: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x2a1)]
    pub uops_dispatched_port_port_1: u64,
    #[raw(0x40a1)]
    pub uops_dispatched_port_port_6: u64,
}
#[derive(Debug, Counter)]
pub struct Port0 {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1a1)]
    pub uops_dispatched_port_port_0: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct Port1 {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x2a1)]
    pub uops_dispatched_port_port_1: u64,
}
#[derive(Debug, Counter)]
pub struct Port5 {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x20a1)]
    pub uops_dispatched_port_port_5: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Port6 {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x40a1)]
    pub uops_dispatched_port_port_6: u64,
}
#[derive(Debug, Counter)]
pub struct LoadOpUtilization {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x80a1)]
    pub uops_dispatched_port_port_7: u64,
    #[raw(0x8a1)]
    pub uops_dispatched_port_port_3: u64,
    #[raw(0x10a1)]
    pub uops_dispatched_port_port_4: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x4a1)]
    pub uops_dispatched_port_port_2: u64,
}
#[derive(Debug, Counter)]
pub struct Port2 {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x4a1)]
    pub uops_dispatched_port_port_2: u64,
}
#[derive(Debug, Counter)]
pub struct Port3 {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x8a1)]
    pub uops_dispatched_port_port_3: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct StoreOpUtilization {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x10a1)]
    pub uops_dispatched_port_port_4: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct Port4 {
    #[raw(0x10a1)]
    pub uops_dispatched_port_port_4: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct Port7 {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x80a1)]
    pub uops_dispatched_port_port_7: u64,
}
#[derive(Debug, Counter)]
pub struct Retiring {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct LightOperations {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
}
#[derive(Debug, Counter)]
pub struct FpArith {
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x10b1)]
    pub uops_executed_x87: u64,
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
}
#[derive(Debug, Counter)]
pub struct X87Use {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x10b1)]
    pub uops_executed_x87: u64,
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
}
#[derive(Debug, Counter)]
pub struct FpScalar {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
}
#[derive(Debug, Counter)]
pub struct FpVector {
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct FpVector128b {
    #[raw(0x8c7)]
    pub fp_arith_inst_retired_128b_packed_single: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
}
#[derive(Debug, Counter)]
pub struct FpVector256b {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x10c7)]
    pub fp_arith_inst_retired_256b_packed_double: u64,
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
}
#[derive(Debug, Counter)]
pub struct MemoryOperations {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x83d0)]
    pub mem_inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct FusedInstructions {
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct NonFusedBranches {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
}
#[derive(Debug, Counter)]
pub struct OtherLightOps {
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x10b1)]
    pub uops_executed_x87: u64,
    #[raw(0x83d0)]
    pub mem_inst_retired_any: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
}
#[derive(Debug, Counter)]
pub struct NopInstructions {
    #[raw(0x2c0)]
    pub inst_retired_nop: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct HeavyOperations {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct FewUopsInstructions {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x4c2)]
    pub uops_retired_macro_fused: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct MicrocodeSequencer {
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Counter)]
pub struct Assists {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
}
#[derive(Debug, Counter)]
pub struct FpAssists {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
}
#[derive(Debug, Counter)]
pub struct Cisc {
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBotlnkL0CoreBoundLikely {
    #[raw(0x4a6)]
    pub exe_activity_2_ports_util: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x2a6)]
    pub exe_activity_1_ports_util: u64,
    #[raw(0x1a6)]
    pub exe_activity_exe_bound_0_ports: u64,
    #[raw(0x23c)]
    pub cpu_clk_unhalted_one_thread_active: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x20013c)]
    pub cpu_clk_unhalted_ref_xclk_any: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x1000114)]
    pub arith_divider_active: u64,
    #[raw(0x40004a3)]
    pub cycle_activity_stalls_total: u64,
    #[raw(0x40a6)]
    pub exe_activity_bound_on_stores: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x140014a3)]
    pub cycle_activity_stalls_mem_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadIpc {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadUopPi {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadUpTb {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadCpi {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadClks {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadSlots {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoThreadExecutePerIssue {
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreCoreIpc {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreFloPc {
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[raw(0x18c7)]
    pub fp_arith_inst_retired_4_flops: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreFpArithUtilization {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreIlp {
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[raw(0x1b1)]
    pub uops_executed_thread_c1: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreEpc {
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoCoreCoreClks {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpLoad {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x81d0)]
    pub mem_inst_retired_all_loads: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpStore {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x82d0)]
    pub mem_inst_retired_all_stores: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpBranch {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpCall {
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpTb {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixBpTkBranch {
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpFlop {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[raw(0x18c7)]
    pub fp_arith_inst_retired_4_flops: u64,
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArith {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[raw(0xfcc7)]
    pub fp_arith_inst_retired_vector: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArithScalarSp {
    #[raw(0x2c7)]
    pub fp_arith_inst_retired_scalar_single: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArithScalarDp {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x1c7)]
    pub fp_arith_inst_retired_scalar_double: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArithAvx128 {
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[raw(0x8c7)]
    pub fp_arith_inst_retired_128b_packed_single: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpArithAvx256 {
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x10c7)]
    pub fp_arith_inst_retired_256b_packed_double: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixIpSwpf {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0xf32)]
    pub sw_prefetch_access_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoInstMixInstructions {
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineRetire {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots_c1: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineIpAssist {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x1001eca)]
    pub fp_assist_any: u64,
    #[raw(0x3fc1)]
    pub other_assists_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineExecute {
    #[raw(0x1b1)]
    pub uops_executed_thread: u64,
    #[raw(0x10002b1)]
    pub uops_executed_core_cycles_ge_1: u64,
    #[raw(0x1b1)]
    pub uops_executed_thread_c1: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineFetchDsb {
    #[raw(0x879)]
    pub idq_dsb_uops: u64,
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoPipelineFetchMite {
    #[raw(0x1000479)]
    pub idq_mite_cycles: u64,
    #[raw(0x479)]
    pub idq_mite_uops: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendFetchUpC {
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x10e)]
    pub uops_issued_any_c1: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendDsbCoverage {
    #[raw(0x879)]
    pub idq_dsb_uops: u64,
    #[raw(0x479)]
    pub idq_mite_uops: u64,
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendDsbSwitchCost {
    #[raw(0x1ab)]
    pub dsb2mite_switches_count: u64,
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendTBpC {
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendICacheMissLatency {
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendIpDsbMissRet {
    #[raw(0x1c6)]
    pub frontend_retired_any_dsb_miss: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendIpUnknownBranch {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendL2mpkiCode {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x1c6)]
    pub frontend_retired_l2_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoFrontendL2mpkiCodeAll {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x2424)]
    pub l2_rqsts_code_rd_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBotlnkL2DsbMisses {
    #[raw(0x4001879)]
    pub idq_dsb_cycles_ok: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[raw(0x4002479)]
    pub idq_all_mite_cycles_4_uops: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x1002479)]
    pub idq_all_mite_cycles_any_uops: u64,
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBotlnkL2DsbBandwidth {
    #[raw(0x1001879)]
    pub idq_dsb_cycles_any: u64,
    #[raw(0x19c)]
    pub idq_uops_not_delivered_core: u64,
    #[raw(0x4002479)]
    pub idq_all_mite_cycles_4_uops: u64,
    #[raw(0x4001879)]
    pub idq_dsb_cycles_ok: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x1002479)]
    pub idq_all_mite_cycles_any_uops: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBotlnkL2IcMisses {
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBadSpecIpMispredict {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBadSpecIpMispIndirect {
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0xe489)]
    pub br_misp_exec_indirect: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBadSpecBranchMispredictionCost {
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[raw(0x483)]
    pub icache_tag_stalls: u64,
    #[raw(0x20010d)]
    pub int_misc_recovery_cycles_any: u64,
    #[raw(0x400019c)]
    pub idq_uops_not_delivered_cycles_0_uops_deliv_core: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall_c1_e1: u64,
    #[raw(0x1003079)]
    pub idq_ms_switches: u64,
    #[raw(0x480)]
    pub icache_16b_ifdata_stall: u64,
    #[raw(0x2ab)]
    pub dsb2mite_switches_penalty_cycles: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x2c2)]
    pub uops_retired_retire_slots: u64,
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x10e)]
    pub uops_issued_any: u64,
    #[raw(0x800d)]
    pub int_misc_clear_resteer_cycles: u64,
    #[raw(0x1e6)]
    pub baclears_any: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
    #[raw(0x3079)]
    pub idq_ms_uops: u64,
    #[raw(0x187)]
    pub decode_lcp: u64,
    #[raw(0x10d)]
    pub int_misc_recovery_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBadSpecSpecClearsRatio {
    #[raw(0xc5)]
    pub br_misp_retired_all_branches: u64,
    #[raw(0x100010d)]
    pub int_misc_clears_count: u64,
    #[raw(0x10001c3)]
    pub machine_clears_count: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBranchesCondNt {
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[raw(0x10c4)]
    pub br_inst_retired_not_taken: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBranchesCondTk {
    #[raw(0x1c4)]
    pub br_inst_retired_conditional: u64,
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
    #[raw(0x10c4)]
    pub br_inst_retired_not_taken: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBranchesCallRet {
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[raw(0x8c4)]
    pub br_inst_retired_near_return: u64,
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct InfoBranchesJump {
    #[raw(0x20c4)]
    pub br_inst_retired_near_taken: u64,
    #[raw(0x1c4)]
    pub br_inst_retired_cond: u64,
    #[raw(0x10c4)]
    pub br_inst_retired_not_taken: u64,
    #[raw(0x2c4)]
    pub br_inst_retired_near_call: u64,
    #[raw(0xc4)]
    pub br_inst_retired_all_branches: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryLoadMissRealLatency {
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryMlp {
    #[raw(0x148)]
    pub l1d_pend_miss_pending: u64,
    #[raw(0x1000148)]
    pub l1d_pend_miss_pending_cycles: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL1mpki {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x8d1)]
    pub mem_load_retired_l1_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL1mpkiLoad {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0xe124)]
    pub l2_rqsts_all_demand_data_rd: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2mpki {
    #[raw(0x10d1)]
    pub mem_load_retired_l2_miss: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2mpkiAll {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x3f24)]
    pub l2_rqsts_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2mpkiLoad {
    #[raw(0x2124)]
    pub l2_rqsts_demand_data_rd_miss: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2mpkiRfo {
    #[raw(0x4b0)]
    pub offcore_requests_demand_rfo: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2hpkiAll {
    #[raw(0x3f24)]
    pub l2_rqsts_miss: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0xff24)]
    pub l2_rqsts_references: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2hpkiLoad {
    #[raw(0xc124)]
    pub l2_rqsts_demand_data_rd_hit: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL3mpki {
    #[raw(0x20d1)]
    pub mem_load_retired_l3_miss: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryFbHpki {
    #[raw(0x40d1)]
    pub mem_load_retired_fb_hit: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL1dCacheFillBw {
    #[raw(0x151)]
    pub l1d_replacement: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL2CacheFillBw {
    #[raw(0x1ff1)]
    pub l2_lines_in_all: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL3CacheFillBw {
    #[raw(0x412e)]
    pub longest_lat_cache_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryL3CacheAccessBw {
    #[raw(0x80b0)]
    pub offcore_requests_all_requests: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryTlbPageWalksUtilization {
    #[raw(0x1085)]
    pub itlb_misses_walk_pending: u64,
    #[raw(0x1049)]
    pub dtlb_store_misses_walk_pending: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x200200)]
    pub cpu_clk_unhalted_thread_any: u64,
    #[raw(0x1008)]
    pub dtlb_load_misses_walk_pending: u64,
    #[raw(0x104f)]
    pub ept_walk_pending: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryTlbCodeStlbMpki {
    #[raw(0xe85)]
    pub itlb_misses_walk_completed: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryTlbLoadStlbMpki {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0xe08)]
    pub dtlb_load_misses_walk_completed: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryTlbStoreStlbMpki {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0xe49)]
    pub dtlb_store_misses_walk_completed: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryCoreL1dCacheFillBw2t {
    #[raw(0x151)]
    pub l1d_replacement: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryCoreL2CacheFillBw2t {
    #[raw(0x1ff1)]
    pub l2_lines_in_all: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryCoreL3CacheFillBw2t {
    #[raw(0x412e)]
    pub longest_lat_cache_miss: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryCoreL3CacheAccessBw2t {
    #[raw(0x80b0)]
    pub offcore_requests_all_requests: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryLatencyLoadL2MissLatency {
    #[raw(0x160)]
    pub offcore_requests_outstanding_demand_data_rd: u64,
    #[raw(0x1b0)]
    pub offcore_requests_demand_data_rd: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryLatencyLoadL2Mlp {
    #[raw(0x1000160)]
    pub offcore_requests_outstanding_cycles_with_demand_data_rd: u64,
    #[raw(0x160)]
    pub offcore_requests_outstanding_demand_data_rd: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryLatencyDataL2Mlp {
    #[raw(0x1000860)]
    pub offcore_requests_outstanding_cycles_with_data_rd: u64,
    #[raw(0x860)]
    pub offcore_requests_outstanding_all_data_rd: u64,
}
#[derive(Debug, Counter)]
pub struct InfoMemoryMixUcLoadPki {
    #[raw(0x4d4)]
    pub mem_load_misc_retired_uc: u64,
    #[raw(0x100)]
    pub inst_retired_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemCpuUtilization {
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemCpUsUtilized {
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemCoreFrequency {
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemGfloPs {
    #[raw(0x4c7)]
    pub fp_arith_inst_retired_128b_packed_double: u64,
    #[raw(0x3c7)]
    pub fp_arith_inst_retired_scalar: u64,
    #[raw(0x20c7)]
    pub fp_arith_inst_retired_256b_packed_single: u64,
    #[raw(0x18c7)]
    pub fp_arith_inst_retired_4_flops: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemTurboUtilization {
    #[raw(0x300)]
    pub cpu_clk_unhalted_ref_tsc: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemSmt2tUtilization {
    #[raw(0x23c)]
    pub cpu_clk_unhalted_one_thread_active: u64,
    #[raw(0x20013c)]
    pub cpu_clk_unhalted_ref_xclk_any: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemKernelUtilization {
    #[raw(0x3c)]
    pub cpu_clk_unhalted_thread_p_sup: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemKernelCpi {
    #[raw(0xc0)]
    pub inst_retired_any_p_sup: u64,
    #[raw(0x3c)]
    pub cpu_clk_unhalted_thread_p_sup: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemDramBwUse {
    #[raw(0x184)]
    pub unc_arb_coh_trk_requests_all: u64,
    #[raw(0x181)]
    pub unc_arb_trk_requests_all: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemMemReadLatency {
    #[raw(0x281)]
    pub unc_arb_trk_requests_data_read: u64,
    #[raw(0x100)]
    pub unc_clock_socket: u64,
    #[raw(0x280)]
    pub unc_arb_trk_occupancy_data_read: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemMemParallelReads {
    #[raw(0x280)]
    pub unc_arb_trk_occupancy_data_read: u64,
    #[raw(0x280)]
    pub unc_arb_trk_occupancy_data_read_c1: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemTime {}
#[derive(Debug, Counter)]
pub struct InfoSystemMux {
    #[raw(0x3c)]
    pub cpu_clk_unhalted_thread_p: u64,
    #[raw(0x200)]
    pub cpu_clk_unhalted_thread: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemSocketClks {
    #[raw(0x100)]
    pub unc_clock_socket: u64,
}
#[derive(Debug, Counter)]
pub struct InfoSystemIpFarBranch {
    #[raw(0x100)]
    pub inst_retired_any: u64,
    #[raw(0x40c4)]
    pub br_inst_retired_far_branch_user: u64,
}
