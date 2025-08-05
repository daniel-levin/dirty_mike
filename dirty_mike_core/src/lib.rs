use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScopeError {}

pub trait Scope: Sized + Send + Sync + 'static {
    fn measure<T, F: Fn() -> T>(f: F) -> Result<(T, Self), ScopeError>;
}
