use perf_event::events::Hardware;
use perf_event::{Builder, Group};

pub fn topdown<T, F: Fn() -> T>(f: F) -> std::io::Result<T> {
    let mut group = Group::new()?;
    let cycles = group.add(&Builder::new(Hardware::BRANCH_MISSES))?;

    group.enable()?;
    let res = f();
    group.disable()?;

    let counts = group.read()?;

    let bm = counts[&cycles];

    dbg!(bm);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let res = topdown(|| {
            let mut sum = 0u64;
            for i in 0..10000 {
                if i % 3 == 0 {
                    sum = sum.wrapping_add(i * 17);
                } else if i % 5 == 0 {
                    sum = sum.wrapping_sub(i / 2);
                } else if i % 7 == 0 {
                    sum = sum.wrapping_mul(2);
                } else {
                    sum = sum.wrapping_add(i.wrapping_pow(2) % 1000);
                }
            }
            sum
        }).unwrap();
    }
}
