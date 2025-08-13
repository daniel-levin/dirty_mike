#[cfg(test)]
mod skl_tests {
    use crate::Counter;

    #[test]
    fn test_basic_intel() {
        #[derive(Debug, Counter)]
        pub struct S {
            #[raw(0x13C)]
            pub cpu_clk_thread_unhalted_ref_xclk: u64,
        }

        let (_, counts) = S::measure(|| {
            let mut x = vec![];

            for i in 0..1000 {
                x.push(format!("{}", i));
            }
        })
        .unwrap();

        assert!(counts.cpu_clk_thread_unhalted_ref_xclk > 2000);
    }
}
