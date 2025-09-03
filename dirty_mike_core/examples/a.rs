use core::arch::x86_64::{
    _mm_cmpeq_epi8, _mm_loadu_epi8, _mm_movemask_epi8, _mm_set1_epi8, _popcnt32,
};
use dirty_mike_derive::ExactCounter;

#[derive(Debug, ExactCounter)]
#[allow(dead_code)]
struct Metrics {
    #[raw(0x0728)]
    core_power_lvl0_turbo_license: u64,

    #[raw(0x1828)]
    core_power_lvl1_turbo_license: u64,

    #[raw(0x2028)]
    core_power_lvl2_turbo_license: u64,

    #[raw(0x4028)]
    core_power_throttle: u64,
}

fn main() -> anyhow::Result<()> {
    let simd = Metrics::measure(|| {
        let mut count = 0;
        let mask = unsafe { _mm_set1_epi8('f' as i8) };

        let arr = include_bytes!("page.txt");
        for off in 0..(arr.len() / 16) {
            unsafe {
                let chunk =
                    _mm_loadu_epi8(arr.as_ptr().byte_offset(16isize * (off as isize)) as *const i8);
                let comps = _mm_cmpeq_epi8(mask, chunk);
                let x = _mm_movemask_epi8(comps);
                let pop = _popcnt32(x);
                count += pop;
            }
        }

        count
    })?;

    let single = Metrics::measure(|| {
        let mut count = 0;
        for i in include_bytes!("page.txt") {
            if *i == b'f' {
                count += 1;
            }
        }

        count
    })?;

    dbg!(&single);
    dbg!(&simd);

    Ok(())
}
