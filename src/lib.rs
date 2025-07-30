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
    use rand::prelude::SliceRandom;

    #[test]
    fn measure_branch_misses() {
        fn work(data: &[i32]) -> u64 {
            let mut sum = 0u64;
            for &value in data {
                if value >= 16384 {
                    sum = sum.wrapping_add(value as u64);
                }
            }
            sum
        }

        let mut shuffled_data: Vec<i32> = (0..32768).collect();
        let mut rng = rand::rng();
        shuffled_data.shuffle(&mut rng);

        let mut sorted_data = shuffled_data.clone();
        sorted_data.sort();

        let res = topdown(move || {
            work(&sorted_data);
        })
        .unwrap();

        let res = topdown(move || {
            work(&shuffled_data);
        })
        .unwrap();
    }
}
