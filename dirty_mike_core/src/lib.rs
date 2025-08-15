use thiserror::Error;

#[derive(Debug, Error)]
pub enum CounterError {
    #[error("cannot open group")]
    CannotOpenGroup(#[source] std::io::Error),

    #[error("cannot add hardware counter {name} to group")]
    CannotAddHardwareCounter {
        name: &'static str,
        #[source]
        error: std::io::Error,
    },

    #[error("cannot add raw counter {id:x} to group")]
    CannotAddRawCounter {
        id: u64,
        #[source]
        error: std::io::Error,
    },
}

pub trait Counter: Sized + Send + Sync + 'static {
    fn measure<T, F: FnOnce() -> T>(f: F) -> Result<(T, Self), CounterError>;
}
