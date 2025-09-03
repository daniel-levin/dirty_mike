use core::arch::x86_64::_mm_cmpeq_epi8;
use core::arch::x86_64::_mm_loadu_epi8;
use core::arch::x86_64::_mm_movemask_epi8;
use core::arch::x86_64::_mm_set1_epi8;
use core::arch::x86_64::_popcnt32;
use dirty_mike_core::exact::*;
use dirty_mike_derive::ExactCounter;

#[derive(Debug, ExactCounter)]
struct Metrics {
    #[raw(0xc0)]
    ins_retired: u64,

    #[raw(0x3c)]
    cycles: u64,
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

    dbg!(single);
    dbg!(simd);

    Ok(())
}
