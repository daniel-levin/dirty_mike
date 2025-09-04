pub mod exact;

pub mod pe2 {
    pub use perf_event::*;
}

pub use dirty_mike_derive::ExactCounter;

use pe2::{Builder, Counter, events::Event};
use std::{io, marker::PhantomData};

pub trait Measurable: Sized + Send + Sync + 'static {
    type Error;

    fn measure<T, F: FnOnce() -> T>(f: F) -> Result<(T, Self), Self::Error>;

    fn measure_n<T, F: FnOnce() -> T, G: Fn() -> F>(
        n: usize,
        g: G,
    ) -> Result<Vec<(T, Self)>, Self::Error> {
        (0..n).map(|_| Self::measure(g())).collect()
    }
}

pub trait Measurements: Send + Sync + 'static {
    fn from_observations(observations: &[u64]) -> Self;
}

#[derive(Debug)]
pub struct Counters<M: Measurements> {
    entries: Vec<Counter>,
    _m: PhantomData<M>,
}

impl<M: Measurements> Default for Counters<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Measurements> Counters<M> {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            _m: PhantomData,
        }
    }

    pub fn add<E: Event>(&mut self, e: E) -> io::Result<()> {
        let b = Builder::new(e).any_cpu().build()?;
        self.entries.push(b);
        Ok(())
    }

    pub fn enable(&mut self) -> io::Result<()> {
        for e in self.entries.iter_mut() {
            e.enable()?;
        }
        Ok(())
    }

    pub fn disable(&mut self) -> io::Result<()> {
        for e in self.entries.iter_mut() {
            e.disable()?;
        }
        Ok(())
    }

    pub fn read(&mut self) -> io::Result<M> {
        Ok(M::from_observations(&self.read_impl()?))
    }

    fn read_impl(&mut self) -> io::Result<Vec<u64>> {
        self.entries.iter_mut().map(|e| e.read()).collect()
    }
}
