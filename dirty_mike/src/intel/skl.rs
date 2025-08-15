#[cfg(test)]
mod skl_tests {
    use crate::Counter;
    use std::time::Duration;

    #[test]
    #[cfg_attr(feature = "ci", ignore = "requires perf_event_open capabilities")]
    fn test_basic_intel() {
        #[derive(Debug, Counter)]
        pub struct S {
            #[raw(0x13C)]
            pub cpu_clk_thread_unhalted_ref_xclk: u64,

            #[time_running]
            pub running: Duration,

            #[time_enabled]
            pub enabled: Duration,
        }

        let (_, counts) = S::measure(|| {
            let mut x = vec![];

            for i in 0..1000 {
                x.push(format!("{}", i));
            }
        })
        .unwrap();

        assert!(counts.cpu_clk_thread_unhalted_ref_xclk > 2000);
        assert!(counts.running.as_nanos() > 0);
        assert!(counts.enabled.as_nanos() > 0);
    }
}
