use core::arch::x86_64::*;
use dirty_mike::{exact::ExactMeasurements, intel::skl::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = include_str!("../../dirty_mike_core/examples/page.txt");

    let m1 = ExactMeasurements::<BadSpeculation, _>::measure_k(25000, |_| {
        || {
            let mut a = vec![];

            for c in s.chars() {
                if c != 'g' {
                    a.push(c);
                }
            }

            a
        }
    })
    .unwrap();

    let m2 = ExactMeasurements::<BadSpeculation, _>::measure_k(25000, |_| {
        || unsafe {
            let mut a: Vec<char> = vec![];

            let mask = _mm_set1_epi8('g' as i8);
            let bytes = s.as_bytes();
            let mut i = 0;

            while i + 16 <= bytes.len() {
                let chunk = _mm_loadu_si128(bytes.as_ptr().add(i) as *const __m128i);
                let comps = _mm_cmpeq_epi8(mask, chunk);
                let movemask = _mm_movemask_epi8(comps) as u16;

                for j in 0..16 {
                    if (movemask & (1 << j)) == 0 {
                        let byte = bytes[i + j];
                        if byte.is_ascii() {
                            a.push(byte as char);
                        }
                    }
                }
                i += 16;
            }

            while i < bytes.len() {
                let byte = bytes[i];
                if byte != b'g' && byte.is_ascii() {
                    a.push(byte as char);
                }
                i += 1;
            }

            a
        }
    })
    .unwrap();

    assert_eq!(m2.results, m1.results);

    println!(
        "{}",
        tabled::Table::new([
            m1.p_row("Scalar p0", 0f64).unwrap(),
            m2.p_row("SIMD p0", 0f64).unwrap(),
            m1.p_row("Scalar p50", 0.5f64).unwrap(),
            m2.p_row("SIMD p50", 0.5f64).unwrap(),
            m1.p_row("Scalar p100", 1f64).unwrap(),
            m2.p_row("SIMD p100", 1f64).unwrap()
        ])
        .to_string()
    );

    Ok(())
}
