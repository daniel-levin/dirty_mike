use core::arch::x86_64::_mm_cmpeq_epi8;
use core::arch::x86_64::_mm_loadu_epi8;
use core::arch::x86_64::_mm_movemask_epi8;
use core::arch::x86_64::_mm_set1_epi8;
use core::arch::x86_64::_popcnt32;
use dirty_mike_core::exact::*;

fn main() -> anyhow::Result<()> {
    let exact = ExactMeasurements::builder()
        .measure(0xc0) // ins retired
        .measure(0x3c) // cycles
        .build()?;

    let simd = exact.measure(|| {
        let mut count = 0;
        let mask = unsafe { _mm_set1_epi8('f' as i8) };

        for i in include_str!("page.txt").as_bytes().chunks_exact(16) {
            unsafe {
                let chunk = _mm_loadu_epi8(i.as_ptr() as *const i8);
                let comps = _mm_cmpeq_epi8(mask, chunk);
                let x = _mm_movemask_epi8(comps);
                let pop = _popcnt32(x);
                count += pop;
            }
        }

        count
    })?;

    dbg!(simd);

    Ok(())
}
