use perf_event::events::Hardware;
use perf_event::{Builder, Group};

pub fn topdown<T, F: Fn() -> T>(f: F) -> std::io::Result<T> {
    let mut group = Group::new()?;
    let cycles = group.add(&Builder::new(Hardware::CPU_CYCLES))?;

    group.enable()?;
    let res = f();
    group.disable()?;

    let counts = group.read()?;

    dbg!(counts);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let res = topdown(|| {
        }).unwrap();
    }
}
