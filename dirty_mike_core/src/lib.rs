use thiserror::Error;

#[derive(Debug, Error)]
pub enum CounterError {}

pub trait Counter: Sized + Send + Sync + 'static {
    fn measure<T, F: Fn() -> T>(f: F) -> Result<(T, Self), CounterError>;
}
